use crate::file_ops::FileItem;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::io;

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct App {
    pub items: Vec<FileItem>,
    pub filtered_items: Vec<usize>,
    pub list_state: ListState,
    pub filter_query: String,
    pub should_quit: bool,
    pub mode: Mode,
}

impl App {
    pub fn new(items: Vec<FileItem>) -> Self {
        let filtered_items: Vec<usize> = (0..items.len()).collect();
        let mut list_state = ListState::default();
        if !filtered_items.is_empty() {
            list_state.select(Some(0));
        }

        Self {
            items,
            filtered_items,
            list_state,
            filter_query: String::new(),
            should_quit: false,
            mode: Mode::Insert, // Start in Insert mode for better UX
        }
    }

    pub fn next(&mut self) {
        if self.filtered_items.is_empty() {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.filtered_items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.filtered_items.is_empty() {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn get_selected_item(&self) -> Option<&FileItem> {
        self.list_state
            .selected()
            .and_then(|i| self.filtered_items.get(i))
            .and_then(|&idx| self.items.get(idx))
    }

    pub fn filter_items(&mut self) {
        if self.filter_query.is_empty() {
            self.filtered_items = (0..self.items.len()).collect();
        } else {
            // Improved fuzzy filtering that supports partial matches
            let query = self.filter_query.to_lowercase();

            self.filtered_items = self
                .items
                .iter()
                .enumerate()
                .filter(|(_, item)| {
                    let name_lower = item.name.to_lowercase();
                    let path_lower = item.full_path_display.to_lowercase();

                    // Simple contains match
                    if name_lower.contains(&query) || path_lower.contains(&query) {
                        return true;
                    }

                    // Fuzzy matching: check if all characters in query appear in order
                    // This allows "ProjecScho" to match "Projects/School"
                    self.fuzzy_match(&path_lower, &query) || self.fuzzy_match(&name_lower, &query)
                })
                .map(|(i, _)| i)
                .collect();
        }

        // Reset selection
        if !self.filtered_items.is_empty() {
            self.list_state.select(Some(0));
        } else {
            self.list_state.select(None);
        }
    }

    // Simple fuzzy matching: check if all characters in needle appear in haystack in order
    fn fuzzy_match(&self, haystack: &str, needle: &str) -> bool {
        let mut needle_chars = needle.chars();
        let mut current_needle_char = needle_chars.next();

        for haystack_char in haystack.chars() {
            if let Some(needle_char) = current_needle_char {
                if haystack_char == needle_char {
                    current_needle_char = needle_chars.next();
                    if current_needle_char.is_none() {
                        return true; // Found all characters in order
                    }
                }
            }
        }

        current_needle_char.is_none() // True if we found all characters
    }

    pub fn add_char(&mut self, c: char) {
        self.filter_query.push(c);
        self.filter_items();
    }

    pub fn remove_char(&mut self) {
        self.filter_query.pop();
        self.filter_items();
    }

    pub fn enter_insert_mode(&mut self) {
        self.mode = Mode::Insert;
    }

    pub fn enter_normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn clear_filter(&mut self) {
        self.filter_query.clear();
        self.filter_items();
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    title: &str,
) -> io::Result<Option<FileItem>> {
    loop {
        terminal.draw(|f| ui(f, &mut app, title))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.mode {
                    Mode::Normal => {
                        match (key.code, key.modifiers) {
                            (KeyCode::Char('q'), _) => {
                                app.should_quit = true;
                                break;
                            }
                            (KeyCode::Esc, _) => {
                                app.should_quit = true;
                                break;
                            }
                            (KeyCode::Enter, _) => {
                                if let Some(item) = app.get_selected_item() {
                                    return Ok(Some(item.clone()));
                                }
                            }
                            // Standard navigation
                            (KeyCode::Down, _) | (KeyCode::Char('j'), _) => app.next(),
                            (KeyCode::Up, _) | (KeyCode::Char('k'), _) => app.previous(),
                            // Ctrl+N/Ctrl+P navigation (common in CLI tools)
                            (KeyCode::Char('n'), KeyModifiers::CONTROL) => app.next(),
                            (KeyCode::Char('p'), KeyModifiers::CONTROL) => app.previous(),
                            // Mode switching
                            (KeyCode::Char('i'), _) => app.enter_insert_mode(),
                            (KeyCode::Char('/'), _) => app.enter_insert_mode(),
                            // Utility commands
                            (KeyCode::Char('c'), _) => app.clear_filter(),
                            (KeyCode::Char('g'), _) => {
                                // Go to top
                                if !app.filtered_items.is_empty() {
                                    app.list_state.select(Some(0));
                                }
                            }
                            (KeyCode::Char('G'), _) => {
                                // Go to bottom
                                if !app.filtered_items.is_empty() {
                                    app.list_state.select(Some(app.filtered_items.len() - 1));
                                }
                            }
                            _ => {}
                        }
                    }
                    Mode::Insert => match (key.code, key.modifiers) {
                        (KeyCode::Esc, _) => {
                            app.enter_normal_mode();
                        }
                        (KeyCode::Enter, _) => {
                            if let Some(item) = app.get_selected_item() {
                                return Ok(Some(item.clone()));
                            }
                        }
                        (KeyCode::Char(c), KeyModifiers::NONE) => app.add_char(c),
                        (KeyCode::Backspace, _) => app.remove_char(),
                        // Navigation in insert mode
                        (KeyCode::Down, _) => app.next(),
                        (KeyCode::Up, _) => app.previous(),
                        (KeyCode::Char('n'), KeyModifiers::CONTROL) => app.next(),
                        (KeyCode::Char('p'), KeyModifiers::CONTROL) => app.previous(),
                        _ => {}
                    },
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(None)
}

fn ui(f: &mut Frame, app: &mut App, title: &str) {
    // Kanagawa Dragon color scheme
    let dragon_fg = Color::Rgb(195, 186, 150);     // #c3ba96 - Light foreground
    let dragon_orange = Color::Rgb(255, 160, 102);  // #ffa066 - Orange accent
    let dragon_yellow = Color::Rgb(255, 211, 87);   // #ffd357 - Yellow accent
    let dragon_green = Color::Rgb(135, 176, 135);   // #87b087 - Green accent
    let dragon_purple = Color::Rgb(158, 153, 188);  // #9e99bc - Purple accent
    let dragon_gray = Color::Rgb(84, 84, 91);       // #54545b - Gray
    let dragon_white = Color::Rgb(200, 195, 188);   // #c8c3bc - Off-white

    // Create main layout with padding on sides
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(2),    // Left padding
            Constraint::Min(0),       // Main content
            Constraint::Length(2),    // Right padding
        ])
        .split(f.area());

    // Vertical layout for main content with more space
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),    // Top spacing
            Constraint::Length(3),    // Search bar
            Constraint::Length(1),    // Spacing between search and list
            Constraint::Min(0),       // Items list
            Constraint::Length(1),    // Spacing before help
            Constraint::Length(3),    // Help section
            Constraint::Length(1),    // Bottom spacing
        ])
        .split(main_chunks[1]);

    // Clean search bar without emojis
    let mode_indicator = match app.mode {
        Mode::Normal => "NORMAL",
        Mode::Insert => "INSERT",
    };

    let (mode_style, border_style) = match app.mode {
        Mode::Normal => (
            Style::default().fg(dragon_green).add_modifier(Modifier::BOLD),
            Style::default().fg(dragon_orange),  // Orange border
        ),
        Mode::Insert => (
            Style::default().fg(dragon_yellow).add_modifier(Modifier::BOLD),
            Style::default().fg(dragon_orange),  // Orange border
        ),
    };

    let filter_paragraph = Paragraph::new(app.filter_query.as_str())
        .style(Style::default().fg(dragon_white))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Line::from(vec![
                    Span::styled("[", Style::default().fg(dragon_gray)),
                    Span::styled(mode_indicator, mode_style),
                    Span::styled("] ", Style::default().fg(dragon_gray)),
                    Span::styled("Search", Style::default().fg(dragon_purple).add_modifier(Modifier::BOLD)),
                ]))
                .border_style(border_style),
        );
    f.render_widget(filter_paragraph, content_chunks[1]);

    // Clean items list without emojis
    let items: Vec<ListItem> = app
        .filtered_items
        .iter()
        .filter_map(|&i| app.items.get(i))
        .enumerate()
        .map(|(idx, item)| {
            let type_indicator = if item.is_dir { "DIR" } else { "FILE" };
            let size_info = if let Some(size) = item.size {
                format!(" ({})", format_bytes(size))
            } else {
                String::new()
            };

            // Truncate long paths with ellipsis for better display
            let display_text = if item.full_path_display.len() > 80 {
                format!("{}...", &item.full_path_display[..77])
            } else {
                item.full_path_display.clone()
            };

            // Add line numbers for easier reference
            let line_number = format!("{:3} ", idx + 1);

            ListItem::new(Line::from(vec![
                Span::styled(line_number, Style::default().fg(dragon_gray)),
                Span::styled(format!("[{type_indicator}]"), Style::default().fg(dragon_purple)),
                Span::raw("  "),
                Span::styled(display_text, Style::default().fg(dragon_fg)),
                Span::styled(size_info, Style::default().fg(dragon_purple)),
            ]))
        })
        .collect();

    // Clean title without emojis
    let filtered_count = app.filtered_items.len();
    let total_count = app.items.len();
    let list_title = format!(
        " {} | {} of {} items{}",
        title,
        filtered_count,
        total_count,
        if filtered_count != total_count { " (filtered)" } else { "" }
    );

    let items_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(list_title)
                .title_style(Style::default().fg(dragon_purple).add_modifier(Modifier::BOLD))
                .border_style(Style::default().fg(dragon_orange))  // Orange border
        )
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(54, 108, 138))  // Darker blue for selection
                .fg(Color::Rgb(255, 255, 255))  // Pure white for maximum contrast
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(items_list, content_chunks[3], &mut app.list_state);

    // Clean help section without emojis
    let help_text = match app.mode {
        Mode::Normal => 
            "Navigation: j/k, arrows, Ctrl+N/P | Jump: g/G | Search: i, / | Actions: Enter, c, q/Esc",
        Mode::Insert => 
            "Type to filter | Navigate: arrows, Ctrl+N/P | Actions: Enter, Backspace, Esc",
    };

    let help_paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(dragon_fg))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Help ")
                .title_style(Style::default().fg(dragon_purple).add_modifier(Modifier::BOLD))
                .border_style(Style::default().fg(dragon_orange))  // Orange border
        );
    f.render_widget(help_paragraph, content_chunks[5]);
}

// Helper function to format bytes in a human-readable way
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn std::error::Error>>
{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn cleanup_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

use crate::config::Config;
use crate::file_ops::FileOperations;
use crate::tui::{cleanup_terminal, run_app, setup_terminal, App};

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    let global_search = config.global_file_search;
    let file_ops = FileOperations::new(config);

    println!("📄 Loading files...");

    // Search files (use global search based on config)
    let files = file_ops.search_files(global_search).await?;

    if files.is_empty() {
        println!("No files found.");
        return Ok(());
    }

    // Setup terminal
    let mut terminal = setup_terminal()?;

    // Create and run the app
    let app = App::new(files);
    let title = if global_search {
        "📄 Fuzzy File Navigation (Global) - Select a file to open"
    } else {
        "📄 Fuzzy File Navigation (Current Directory) - Select a file to open"
    };
    let result = run_app(&mut terminal, app, title);

    // Cleanup terminal
    cleanup_terminal(&mut terminal)?;

    // Handle the result
    match result? {
        Some(selected_item) => {
            if !selected_item.is_dir {
                println!("Opening: {}", selected_item.path.display());

                // Try to open the file with the configured editor
                if let Err(e) = file_ops.open_file(&selected_item.path).await {
                    eprintln!("Failed to open file: {e}");
                    println!("File path: {}", selected_item.path.display());
                }
            }
        }
        None => {
            println!("No file selected.");
        }
    }

    Ok(())
}

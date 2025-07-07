use crate::config::Config;
use crate::file_ops::FileOperations;
use crate::tui::{cleanup_terminal, run_app, setup_terminal, App};

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    let file_ops = FileOperations::new(config);

    // Use fast shallow search for immediate response
    eprintln!("🚀 Loading directories...");

    let directories = file_ops.search_directories_fast().await?;

    if directories.is_empty() {
        eprintln!("No directories found.");
        return Ok(());
    }

    eprintln!(
        "✅ Loaded {} directories (shallow scan for speed)",
        directories.len()
    );

    // Setup terminal
    let mut terminal = setup_terminal()?;

    // Create and run the app
    let app = App::new(directories);
    let result = run_app(
        &mut terminal,
        app,
        "🔍 Fuzzy Directory Navigation (Fast Mode)",
    );

    // Cleanup terminal
    cleanup_terminal(&mut terminal)?;

    // Handle the result
    match result? {
        Some(selected_item) => {
            if selected_item.is_dir {
                println!("{}", selected_item.path.display());
            }
        }
        None => {
            // Print nothing on cancel
        }
    }

    Ok(())
}

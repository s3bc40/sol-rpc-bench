use std::{
    io,
    sync::{Arc, Mutex},
    time::Duration,
};

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend, widgets::Paragraph};

mod app;
mod rpc;
mod ui;

/// Entry point of the application
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Shared state for benchmark results
    // Mutex<...> - A lock that ensures only one thread can access the Vec at a time (prevents data races)
    // Arc<...> - "Atomic Reference Counter" - lets multiple parts of your code share ownership of the same data
    // The benchmark runs on a background thread (created by tokio::spawn)
    // Both need to access the same AppRpcResult>
    // Arc lets them share it safely, Mutex prevents them from accessing it simultaneously
    let app = Arc::new(Mutex::new(app::App::new()));
    // Another reference to the same results
    let app_clone = app.clone();

    // Spawn background task for benchmarking
    // move - moves ownership of results_clone into the async block
    tokio::spawn(async move {
        loop {
            let bench_results = rpc::benchmark_all_rpcs().await;
            // Dereference the Mutex to get to the Vec and update it
            app_clone.lock().unwrap().update_results(bench_results);
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });

    // Main application loop
    loop {
        // Get latest results from shared state - clone them to release lock quickly
        let latest_results = {
            let current_app = app.lock().unwrap();
            current_app.results.clone()
        };

        // Render the UI
        terminal.draw(|f| {
            // Display the 5 fastest RPC and total count
            let text = if latest_results.is_empty() {
                "Benchmarking RPCs... Press 'q' to quit.".to_string()
            } else {
                let mut output = format!("Benchmarked {} RPCs:\n\n", latest_results.len());
                for (i, result) in latest_results.iter().enumerate() {
                    output.push_str(&format!(
                        "{}. {} - {}ms {}\n",
                        i + 1,
                        result.name,
                        result.latency_ms,
                        if result.healthy { "✅" } else { "🔴" }
                    ));
                }
                output.push_str("\nPress 'q' to quit");
                output
            };

            let paragraph = Paragraph::new(text);
            f.render_widget(paragraph, f.area());
        })?;

        // Handle input with event polling
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    continue;
                }
                if key.code == event::KeyCode::Char('q') {
                    app.lock().unwrap().should_quit = true;
                }
            }
        }
        if app.lock().unwrap().should_quit {
            break;
        }
    }
    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

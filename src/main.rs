use std::{
    io, process,
    sync::{Arc, Mutex},
    time::Duration,
};

use crossterm::{
    event::{self, DisableMouseCapture, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};

mod app;
mod rpc;
mod ui;

/// Entry point of the application
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Set up panic handler to see errors before terminal cleanup
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("PANIC: {:?}", panic_info);
    }));

    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app and capture the result
    let result = run_app(&mut terminal).await;

    // Restore terminal, even if there was an error
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Return the result (or error) after cleanup
    // Exit cleanly
    match result {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}

/// Main application loop separated from setup/teardown
async fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
    // Shared state for benchmark results
    // Mutex<...> - A lock that ensures only one thread can access the Vec at a time (prevents data races)
    // Arc<...> - "Atomic Reference Counter" - lets multiple parts of your code share ownership of the same data
    // The benchmark runs on a background thread (created by tokio::spawn)
    // Both need to access the same AppRpcResult>
    // Arc lets them share it safely, Mutex prevents them from accessing it simultaneously
    let app = Arc::new(Mutex::new(app::App::new()));
    let app_clone = app.clone();

    // Spawn background task
    let benchmark_task = tokio::spawn(async move {
        loop {
            let bench_results = rpc::benchmark_all_rpcs().await;
            app_clone.lock().unwrap().update_results(bench_results);
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });

    // Main UI loop
    loop {
        let (latest_results, last_update, should_quit) = {
            let current_app = app.lock().unwrap();
            (
                current_app.results.clone(),
                current_app.last_update,
                current_app.should_quit,
            )
        };

        terminal.draw(|f| {
            ui::render(f, &latest_results, last_update);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                if key.code == event::KeyCode::Char('q') {
                    app.lock().unwrap().should_quit = true;
                }
            }
        }

        if should_quit {
            // Abort the background task immediately (don't wait)
            benchmark_task.abort();
            break;
        }
    }

    Ok(())
}

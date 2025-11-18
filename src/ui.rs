use std::time::Instant;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Paragraph},
};

use crate::app::RpcResult;

/// Main rendering functions for the terminal UI.
pub fn render(f: &mut Frame, results: &[RpcResult], last_update: Option<Instant>) {
    // Create the main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
            Constraint::Min(3),
        ])
        .split(f.area());

    if results.is_empty() {
        draw_empty_state(f, chunks[0], chunks[1]);
    } else {
        // Draw bar chart in the top chunk
        draw_bar_chart(f, chunks[0], results);

        // Draw table in the bottom chunk
        draw_table(f, chunks[1], results);
    }

    // Draw help bar at the bottom
    draw_help_bar(f, chunks[2], last_update);
}

/// Draws a bar chart of RPC latencies.
fn draw_bar_chart(f: &mut Frame, area: Rect, results: &[RpcResult]) {
    // Calculate bar width based on available space
    let available_width = area.width.saturating_sub(4);
    let num_bars = results.len().min(11) as u16;
    let bar_width = (available_width / num_bars).max(3);

    // Cap max value for better visualization
    const MAX_DISPLAY_LATENCY: u64 = 1000;

    let bar_data: Vec<Bar> = results
        .iter()
        .map(|r| {
            Bar::default()
                .value(r.latency_ms)
                .label(Line::from(r.name.clone()))
                .text_value(r.latency_ms.to_string())
                .style(bar_style(r.latency_ms))
        })
        .collect();

    let bar_chart = BarChart::default()
        .data(BarGroup::default().bars(&bar_data))
        .block(Block::default().title("🚀 All RPCs").borders(Borders::ALL))
        .bar_width(bar_width)
        .bar_gap(1)
        .max(MAX_DISPLAY_LATENCY);

    f.render_widget(bar_chart, area);
}

/// Determines the style of a bar based on latency.
fn bar_style(latency: u64) -> Style {
    if latency < 200 {
        Style::default().fg(ratatui::style::Color::Green)
    } else if latency < 500 {
        Style::default().fg(ratatui::style::Color::Yellow)
    } else {
        Style::default().fg(ratatui::style::Color::Red)
    }
}

/// Draws a table of RPC results.
/// Note: For simplicity, this example uses a Paragraph widget to display text.
fn draw_table(f: &mut Frame, area: Rect, results: &[RpcResult]) {
    let mut output = "Rank | Name | Latency | Status\n".to_string();
    for (i, result) in results.iter().enumerate() {
        output.push_str(&format!(
            "{:>4} | {:<20} | {:>6}ms | {}\n",
            i + 1,
            result.name,
            result.latency_ms,
            if result.healthy { "✅" } else { "🔴" }
        ));
    }
    let paragraph = Paragraph::new(output).block(
        Block::default()
            .title("📊 All RPC Results")
            .borders(Borders::ALL),
    );
    f.render_widget(paragraph, area);
}

/// Help bar at the bottom of the UI.
pub fn draw_help_bar(f: &mut Frame, area: Rect, last_update: Option<Instant>) {
    let elapsed_secs = last_update.map(|t| t.elapsed().as_secs()).unwrap_or(0);

    let status_text = if elapsed_secs == 0 {
        "Benchmarking...".to_string()
    } else {
        format!("Updated {}s ago", elapsed_secs)
    };

    let help_text = Line::from(vec![
        Span::raw("Press 'q' to quit | "),
        Span::raw(status_text),
        Span::raw(" | "),
        Span::styled("● ", Style::default().fg(ratatui::style::Color::Green)),
        Span::raw("< 200ms | "),
        Span::styled("● ", Style::default().fg(ratatui::style::Color::Yellow)),
        Span::raw("< 500ms | "),
        Span::styled("● ", Style::default().fg(ratatui::style::Color::Red)),
        Span::raw(">= 500ms"),
    ]);

    let paragraph =
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title("💡 Help"));

    f.render_widget(paragraph, area);
}

/// Draws an empty state when there are no RPC results.
fn draw_empty_state(f: &mut Frame, bar_chart_area: Rect, table_area: Rect) {
    let chart_placeholder =
        Paragraph::new("⏳ Running first benchmark...\n\nPlease wait 3-5 seconds")
            .block(Block::default().title("🚀 All RPCs").borders(Borders::ALL))
            .centered();

    let table_placeholder = Paragraph::new("Benchmarking endpoints:\n\n• PublicNode\n• Helius\n• Solana Mainnet Beta\n• ... and 8 more")
        .block(Block::default().title("📊 All RPC Results").borders(Borders::ALL)).centered();

    f.render_widget(chart_placeholder, bar_chart_area);
    f.render_widget(table_placeholder, table_area);
}

use std::io::Error;

use ratatui::{
    CompletedFrame,
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Chart, Dataset},
};

use crate::statistics::PopulationStatSeries;

pub fn draw<'a>(
    terminal: &'a mut ratatui::DefaultTerminal,
    stats: &'a PopulationStatSeries,
) -> Result<CompletedFrame<'a>, Error> {
    let avg_fitness = stats.get_average_fitness_series();
    let datasets = vec![
        Dataset::default()
            .name("Average Fitness")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&avg_fitness),
    ];
    let chart = Chart::new(datasets)
        .block(
            Block::bordered().title(Span::styled(
                "Fitness",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
        )
        .x_axis(
            Axis::default()
                .title("Generation")
                .style(Style::default().fg(Color::Gray)),
        )
        .y_axis(
            Axis::default()
                .title("Fitness")
                .style(Style::default().fg(Color::Gray))
                .bounds([-20.0, 20.0])
                .labels([
                    Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("0"),
                    Span::styled("20", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );

    terminal.draw(|frame| frame.render_widget(chart, frame.area()))
}

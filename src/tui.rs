use ratatui::{
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Chart, Dataset},
};

use crate::statistics::PopulationStatSeries;

pub fn draw(terminal: &mut ratatui::DefaultTerminal, stats: &PopulationStatSeries) {
    let avg_fitness = stats.get_average_fitness_series();
    let datasets = vec![
        Dataset::default()
            .name("dta2")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&avg_fitness),
    ];
    let chart = Chart::new(datasets)
        .block(
            Block::bordered().title(Span::styled(
                "Chart",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
        )
        .x_axis(
            Axis::default()
                .title("X Axis")
                .style(Style::default().fg(Color::Gray)),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis")
                .style(Style::default().fg(Color::Gray))
                .bounds([-20.0, 20.0])
                .labels([
                    Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw("0"),
                    Span::styled("20", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );

    let _ = terminal.draw(|frame| frame.render_widget(chart, frame.area()));
}

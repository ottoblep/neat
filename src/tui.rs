use std::io::Error;

use ratatui::{
    CompletedFrame,
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Chart, Dataset, GraphType},
};

use crate::statistics::PopulationStatSeries;

type Limits = (f64, f64);

fn chart<'a>(
    title: &'a str,
    x_label: &'a str,
    y_label: &'a str,
    points: &'a Vec<(f64, f64)>,
) -> Chart<'a> {
    let x_limits: Limits = (0.0, (points.len().saturating_sub(1) as f64));
    let mut y_limits: Limits = points.iter().fold((f64::MIN, f64::MAX), |acc, (_, a)| {
        (acc.0.min(*a), acc.1.max(*a))
    });

    let pad = ((y_limits.0 - y_limits.0) * 0.1).max(0.5);
    y_limits.0 -= pad;
    y_limits.1 += pad;

    let datasets = vec![
        Dataset::default()
            .name(title)
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Scatter)
            .style(Style::default().fg(Color::Cyan))
            .data(&points),
    ];

    Chart::new(datasets)
        .block(
            Block::bordered().title(Span::styled(
                title,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
        )
        .x_axis(
            Axis::default()
                .title(x_label)
                .style(Style::default().fg(Color::Gray))
                .bounds([x_limits.0, x_limits.1])
                .labels([
                    Span::raw(format!("{:.2}", x_limits.0)),
                    Span::raw(format!("{:.2}", (x_limits.0 + x_limits.1) / 2.0)),
                    Span::raw(format!("{:.2}", x_limits.1)),
                ]),
        )
        .y_axis(
            Axis::default()
                .title(y_label)
                .style(Style::default().fg(Color::Gray))
                .bounds([y_limits.0, y_limits.1])
                .labels([
                    Span::raw(format!("{:.2}", y_limits.0)),
                    Span::raw(format!("{:.2}", (y_limits.0 + y_limits.1) / 2.0)),
                    Span::raw(format!("{:.2}", y_limits.1)),
                ]),
        )
}

pub fn draw<'a>(
    terminal: &'a mut ratatui::DefaultTerminal,
    stats: &'a PopulationStatSeries,
) -> Result<CompletedFrame<'a>, Error> {
    let avg_fitness = stats.get_average_fitness_series();
    let chart = chart("Average Fitness", "Generation", "Fitness", &avg_fitness);
    terminal.draw(|frame| frame.render_widget(chart, frame.area()))
}

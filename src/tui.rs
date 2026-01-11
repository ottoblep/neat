use std::io::Error;

use ratatui::{
    CompletedFrame,
    layout::{Constraint, Direction, Layout},
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
    datasets: &'a Vec<(&'a str, Vec<(f64, f64)>)>,
) -> Chart<'a> {
    let combined = datasets.iter().map(|(_, b)| b).flatten();
    let x_limits: Limits = combined.clone().fold((f64::MAX, f64::MIN), |acc, (a, _)| {
        (acc.0.min(*a), acc.1.max(*a))
    });
    let y_limits: Limits = combined.clone().fold((f64::MAX, f64::MIN), |acc, (_, a)| {
        (acc.0.min(*a), acc.1.max(*a))
    });

    let datasets = datasets
        .into_iter()
        .map(|(title, points)| {
            Dataset::default()
                .name(*title)
                .marker(symbols::Marker::Dot)
                .graph_type(GraphType::Scatter)
                .style(Style::default().fg(Color::Cyan))
                .data(&points)
        })
        .collect();

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
    let best_fitness_data: Vec<(f64, f64)> = stats
        .best_fitness
        .clone()
        .into_iter()
        .enumerate()
        .map(|(a, b)| (a as f64, b as f64))
        .collect();
    let avg_fitness_data: Vec<(f64, f64)> = stats
        .average_fitness
        .clone()
        .into_iter()
        .enumerate()
        .map(|(a, b)| (a as f64, b as f64))
        .collect();

    let fitness_datasets = vec![
        ("Best Fitness", best_fitness_data),
        ("Avg Fitness", avg_fitness_data),
    ];

    let fitness_chart = chart("Best Fitness", "Generation", "Fitness", &fitness_datasets);

    terminal.draw(|frame| {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area());

        frame.render_widget(fitness_chart, layout[0]);
    })
}

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

const NAMED_COLORS: [Color; 16] = [
    Color::Cyan,
    Color::Magenta,
    Color::Green,
    Color::Red,
    Color::Yellow,
    Color::Blue,
    Color::Gray,
    Color::LightRed,
    Color::LightGreen,
    Color::LightYellow,
    Color::LightBlue,
    Color::LightMagenta,
    Color::LightCyan,
    Color::White,
    Color::Black,
    Color::DarkGray,
];

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
        .enumerate()
        .map(|(num, (title, points))| {
            Dataset::default()
                .name(*title)
                .marker(symbols::Marker::Dot)
                .graph_type(GraphType::Scatter)
                .style(Style::default().fg(*NAMED_COLORS.get(num).unwrap_or(&Color::Black)))
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

    let avg_genome_size_data: Vec<(f64, f64)> = stats
        .average_genome_size
        .clone()
        .into_iter()
        .enumerate()
        .map(|(a, b)| (a as f64, b as f64))
        .collect();

    let diversity_data: Vec<(f64, f64)> = stats
        .approx_diversity
        .clone()
        .into_iter()
        .enumerate()
        .map(|(a, b)| (a as f64, b as f64))
        .collect();

    let fitness_datasets = vec![
        ("Best Fitness", best_fitness_data),
        ("Avg Fitness", avg_fitness_data),
    ];
    let genome_datasets = vec![("Avg Genome Size", avg_genome_size_data)];
    let diversity_datasets = vec![("Diversity", diversity_data)];

    let fitness_chart = chart("Fitness", "Generation", "Fitness", &fitness_datasets);
    let genome_size_chart = chart("Genome Size", "Generation", "Size", &genome_datasets);
    let diversity_chart = chart("Diversity", "Generation", "Diversity", &diversity_datasets);

    terminal.draw(|frame| {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(frame.area());

        frame.render_widget(fitness_chart, layout[0]);

        let bottom = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[1]);

        frame.render_widget(genome_size_chart, bottom[0]);
        frame.render_widget(diversity_chart, bottom[1]);
    })
}

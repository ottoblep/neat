use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Axis, Block, Chart, Dataset},
};

pub fn draw(terminal: &mut ratatui::DefaultTerminal) {
    let datasets = vec![Dataset::default()];
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

    terminal.draw(|frame| frame.render_widget(chart, frame.area()))?;
}

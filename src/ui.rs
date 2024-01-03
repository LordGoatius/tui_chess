use ratatui::{
    layout::{Alignment, Direction},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph},
    prelude::{Constraint, Layout, Rect, Line},
};

use crate::app::App;
use crate::tui::Frame;

pub fn render(app: &mut App, f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
                     Constraint::Percentage(12),
                     Constraint::Percentage(76),
                     Constraint::Percentage(12),
        ])
        .split(f.size());
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
                     Constraint::Percentage(20),
                     Constraint::Percentage(60),
                     Constraint::Percentage(20),
        ])
        .split(layout[1]);

    f.render_stateful_widget(app.game.clone(), cols[1], &mut app.game_state);
    //f.render_widget(Paragraph::new(vec![
    //        Line::from("Title"),
    //        Line::from("Horizontal Layout Example. Press q to quit"),
    //        Line::from("Each line has 2 constraints, plus Min(0) to fill the remaining space."),
    //        Line::from("E.g. the second line of the Len/Min box is [Length(2), Min(2), Min(0)]"),
    //        Line::from("Note: constraint labels that don't fit are truncated"),                    
    //]).alignment(Alignment::Center), int_cols[0]);

    f.render_widget(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)
            .border_style(Style::new().fg(Color::Rgb(0x33, 0xa0, 0x33)))
            .title(" Top ")
            .title_style(Style::new().fg(Color::DarkGray))
            .title_alignment(Alignment::Center), layout[0]);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(Color::Rgb(0x33, 0xa0, 0x33)))
            .title(" Board ")
            .title_style(Style::new().fg(Color::DarkGray))
            .title_alignment(Alignment::Center), cols[1]);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(Color::Rgb(0x33, 0xa0, 0x33)))
            .title(" Left ")
            .title_style(Style::new().fg(Color::DarkGray))
            .title_alignment(Alignment::Center), cols[0]);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(Color::Rgb(0x33, 0xa0, 0x33)))
            .title(" Right ")
            .title_style(Style::new().fg(Color::DarkGray))
            .title_alignment(Alignment::Center), cols[2]);
    f.render_widget(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)
            .border_style(Style::new().fg(Color::Rgb(0x33, 0xa0, 0x33)))
            .title(" Bottom ")
            .title_style(Style::new().fg(Color::DarkGray))
            .title_alignment(Alignment::Center), layout[2]);
    //f.render_widget(
    //Paragraph::new(format!(
    //    "
    //      Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
    //      Press `j` and `k` to increment and decrement the counter respectively.\n\
    //      Counter: {}
    //    ",
    //    app.counter
    //))
    //.block(
    //    Block::default()
    //        .title("Counter App")
    //        .title_alignment(Alignment::Center)
    //        .borders(Borders::ALL)
    //        .border_type(BorderType::Rounded),
    //)
    //.style(Style::default().fg(Color::White))
    //.alignment(Alignment::Center),
    //f.size(),
    //)
}

use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::app::CurrentScreen;
use crate::App;

const PINK: Color = Color::Rgb(0xff, 0xd1, 0xdc);

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size().inner(Margin::new(1, 1)));

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    let border = Block::default()
        .title("Wapiti")
        .border_style(Style::default().fg(PINK))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    f.render_widget(border, f.size());

    let header = Block::default()
        .border_style(Style::default().fg(PINK))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    f.render_widget(header, chunks[0]);

    //FOOTER
    let current_key_hints = match app.current_screen {
        CurrentScreen::Main => Span::styled("(q) to quit / (l) to login", Style::default().fg(PINK)),
        CurrentScreen::Login => Span::styled("(ESC) to exit / (ENTER) to save", Style::default().fg(PINK))

    };

    let footer_block = Block::default()
        .border_style(Style::default().fg(PINK))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    let footer = Paragraph::new(Line::from(current_key_hints))
        .block(footer_block);

    f.render_widget(footer, chunks[2]);

    let list = Block::default()
        .border_style(Style::default().fg(PINK))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    f.render_widget(list, horizontal_chunks[0]);

    let main = Block::default()
        .border_style(Style::default().fg(PINK))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    f.render_widget(main, horizontal_chunks[1]);

    //LOGIN POPUP
    if let CurrentScreen::Login = &app.current_screen {
        let popup_block = Block::default()
            .title("Login")
            .border_style(Style::default().fg(PINK))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let area = centered_rect(60, 25, f.size());
        f.render_widget(Clear, area);
        f.render_widget(popup_block, area);

        let edit_chunks = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(3)
            .constraints([
              Constraint::Min(1)
            , Constraint::Length(3)
            , Constraint::Min(1)
            ])
            .split(area);

        let edit_block = Block::default()
            .title("Instance URL")
            .border_style(Style::default().fg(PINK))
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let edit_text = Paragraph::new(app.instance_url.clone()).block(edit_block);
        f.render_widget(edit_text, edit_chunks[1]);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

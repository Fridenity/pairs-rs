use super::{
    state::{InputMode, PopupMsg, PopupSeverity, Screen, TitleButtons},
    App,
};
use itertools::Itertools;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthStr;

fn title<B: Backend>(f: &mut Frame<B>, app: &App) {
    let border = Block::default()
        .title("Pairs-rs")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().add_modifier(Modifier::DIM | Modifier::BOLD));
    f.render_widget(border, f.size());

    let chunks_y = Layout::default()
        .direction(Direction::Vertical)
        .margin(6)
        .constraints(
            [
                Constraint::Length(15),
                Constraint::Length(9),
                Constraint::Min(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let buttons_y = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(chunks_y[1]);

    let button_areas = (0..app.ui_defaults.all_buttons.len())
        .map(|i| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(40),
                        Constraint::Percentage(20),
                        Constraint::Percentage(40),
                    ]
                    .as_ref(),
                )
                .split(buttons_y[i])[1]
        })
        .collect_vec();

    let pair_rs_banner = "                                                                        
                          _/                                                      
     _/_/_/      _/_/_/      _/  _/_/    _/_/_/              _/  _/_/    _/_/_/   
    _/    _/  _/    _/  _/  _/_/      _/_/      _/_/_/_/_/  _/_/      _/_/        
   _/    _/  _/    _/  _/  _/            _/_/              _/            _/_/     
  _/_/_/      _/_/_/  _/  _/        _/_/_/                _/        _/_/_/        
 _/                                                                               
_/                                                                                 
";
    let mut pair_rs_banner_txt = Text::from(pair_rs_banner);
    pair_rs_banner_txt.patch_style(
        Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::BOLD),
    );
    let pair_rs_banner_para = Paragraph::new(pair_rs_banner_txt).alignment(Alignment::Center);

    f.render_widget(pair_rs_banner_para, chunks_y[0]);

    app.ui_defaults
        .all_buttons
        .iter()
        .zip(button_areas)
        .enumerate()
        .for_each(|(i, (s, a)): (usize, (&TitleButtons, Rect))| {
            let is_selected = i as i8 == app.curr_index;
            let color = if is_selected {
                Color::LightBlue
            } else {
                Color::Gray
            };
            let mod_ = if is_selected {
                Modifier::BOLD
            } else {
                Modifier::DIM
            };
            let style = Style::default().fg(color).add_modifier(mod_);

            let para = Paragraph::new(Span::styled(s.to_string(), style))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(style));
            f.render_widget(para, a);
        });
}

#[allow(unused_variables)] // TODO:
fn player_count_input<B: Backend>(f: &mut Frame<B>, app: &App) {
    todo!()
}

#[allow(unused_variables)] // TODO:
fn gameplay<B: Backend>(f: &mut Frame<B>, app: &App) {
    todo!()
}

#[allow(unused_variables)] // TODO:
fn options<B: Backend>(f: &mut Frame<B>, app: &App) {
    todo!()
}

fn player_name_input<B: Backend>(f: &mut Frame<B>, app: &App) {
    let block = Block::default()
        .title("Players")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().add_modifier(Modifier::DIM));
    f.render_widget(block, f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(6)
        .constraints(
            [
                Constraint::Length(5),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to go back, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start typing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Action1 => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop typing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to submit"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Action1 => Style::default().fg(Color::LightBlue),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Action1 => {
            f.set_cursor(chunks[1].x + app.input.width() as u16 + 1, chunks[1].y + 1)
        }
    }

    let player_names: Vec<ListItem> = app
        .ui_var
        .player_names
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = Spans::from(vec![
                Span::styled(
                    format!("P{}: ", i + 1),
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(m),
            ]);
            ListItem::new(content)
        })
        .collect();
    let player_name_list = List::new(player_names).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Player Names List"),
    );
    f.render_widget(player_name_list, chunks[2]);
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    if let Some(PopupMsg { msg, severity }) = &app.popup {
        let (title, color) = match severity {
            PopupSeverity::Info => ("Hint", Color::LightBlue),
            PopupSeverity::Warn => ("Warning", Color::Yellow),
            PopupSeverity::Err => ("Error", Color::Red),
        };

        let block = Block::default()
            .title(title)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color));
        let area = centered_rect(20, 20, f.size());
        let paragraph = Paragraph::new(msg.as_str()).block(block);
        // f.render_widget(Clear, area);
        f.render_widget(paragraph, area);
    }

    match &app.curr_screen {
        Screen::Title => title(f, app),
        Screen::PlayerCountInput => player_count_input(f, app),
        Screen::PlayerNameInput => player_name_input(f, app),
        Screen::Gameplay => gameplay(f, app),
        Screen::Options => options(f, app),
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

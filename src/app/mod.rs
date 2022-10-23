pub mod state;
pub mod ui;

use crossterm::event::{self, Event, KeyCode};
use state::{InputMode, PopupMsg, Screen, TitleButtons};
use std::io;
use tui::{backend::Backend, Terminal};

pub struct UiDefaults<const N: usize = 3> {
    all_buttons: [TitleButtons; N],
}

impl Default for UiDefaults {
    fn default() -> Self {
        Self {
            all_buttons: [
                TitleButtons::Start,
                TitleButtons::Options,
                TitleButtons::Exit,
            ],
        }
    }
}

pub struct UiVar {
    player_names: Vec<String>,
}

impl Default for UiVar {
    fn default() -> Self {
        Self {
            player_names: vec![],
        }
    }
}

pub struct App {
    input: String,
    input_mode: InputMode,
    popup: Option<PopupMsg>,
    curr_index: i8,
    curr_index_mod: usize,
    curr_screen: Screen,
    ui_var: UiVar,
    ui_defaults: UiDefaults,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            popup: None,
            curr_index: 0,
            curr_index_mod: 1,
            curr_screen: Screen::Title,
            ui_var: UiVar::default(),
            ui_defaults: UiDefaults::default(),
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if let Some(_) = app.popup {
                app.popup = None
            }
            match app.curr_screen {
                Screen::Title => {
                    app.curr_index_mod = app.ui_defaults.all_buttons.len();
                    let mod_curr_index = |app: &mut App| {
                        app.curr_index = app.curr_index.rem_euclid(app.curr_index_mod as i8)
                    };
                    match key.code {
                        KeyCode::Down | KeyCode::Right => {
                            app.curr_index += 1;
                            mod_curr_index(&mut app);
                        }
                        KeyCode::Up | KeyCode::Left => {
                            app.curr_index -= 1;
                            mod_curr_index(&mut app);
                        }
                        KeyCode::Enter => {
                            match app.ui_defaults.all_buttons[app.curr_index as usize] {
                                TitleButtons::Start => {
                                    app.curr_screen = Screen::PlayerCountInput;
                                }
                                TitleButtons::Options => app.curr_screen = Screen::Options,
                                TitleButtons::Exit => return Ok(()),
                            }
                        }
                        _ => {}
                    }
                }
                Screen::PlayerCountInput | Screen::PlayerNameInput => match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            app.input_mode = InputMode::Action1;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Action1 => match key.code {
                        KeyCode::Enter => {
                            app.input = app.input.trim().into();
                            if app.input.is_empty() {
                                app.popup = Some(PopupMsg::warn("Name is required.".into()));
                                continue;
                            }
                            if app.ui_var.player_names.contains(&app.input) {
                                app.popup = Some(PopupMsg::warn(format!(
                                    "Name \"{}\" already exists.",
                                    &app.input
                                )));
                                continue;
                            }
                            app.ui_var.player_names.push(app.input.drain(..).collect());
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                },
                Screen::Gameplay => todo!(),
                Screen::Options => todo!(),
            }
        }
    }
}

mod app;
mod ui;

use crate::app::JsonEditor;
use crate::ui::ui;
use app::{ActiveScreen, EditingMode};
use crossterm::event;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    // Set up the terminal.
    let mut stderr = io::stderr();
    enable_raw_mode()?;
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    // Run the app on stderr.
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr))?;
    let mut json_editor = JsonEditor::default();
    let _ = run_app(&mut terminal, &mut json_editor)?;

    // TODO: Output the json.

    // Clean up after ourselves.
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut JsonEditor) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue; // Skip events that are not KeyEventKind::Press.
            }

            match app.current_screen {
                ActiveScreen::Main => match key.code {
                    KeyCode::Char('q') => app.current_screen = ActiveScreen::Exiting,
                    KeyCode::Char('e') => {
                        app.current_screen = ActiveScreen::Editing;
                        app.editing_mode = Some(EditingMode::Key);
                    }
                    _ => {}
                },

                ActiveScreen::Exiting => match key.code {
                    KeyCode::Char('y') => return Ok(true),
                    KeyCode::Char('n' | 'q') => return Ok(false),
                    _ => {}
                },

                ActiveScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        if let Some(mode) = &app.editing_mode {
                            match mode {
                                EditingMode::Key => app.editing_mode = Some(EditingMode::Value),
                                EditingMode::Value => {
                                    app.save_kv_pair();
                                    app.current_screen = ActiveScreen::Main;
                                }
                            }
                        }
                    }

                    KeyCode::Backspace => {
                        if let Some(mode) = &app.editing_mode {
                            match mode {
                                EditingMode::Key => app.key_input.pop(),
                                EditingMode::Value => app.value_input.pop(),
                            };
                        }
                    }

                    KeyCode::Esc => {
                        app.current_screen = ActiveScreen::Main;
                        app.editing_mode = None;
                    }

                    KeyCode::Tab => app.toggle_editing_mode(),

                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.editing_mode {
                            match editing {
                                EditingMode::Key => app.key_input.push(value),
                                EditingMode::Value => app.value_input.push(value),
                            }
                        }
                    }
                    _ => {}
                },

                ActiveScreen::Editing => {}
            }
        }
    }
}

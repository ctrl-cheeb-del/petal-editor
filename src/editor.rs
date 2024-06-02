use core::cmp::min;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
mod terminal;
mod view;
use terminal::{Position, Size, Terminal};
use view::View;

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}

pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View,
    offset_y: usize,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }
        Ok(Self {
            should_quit: false,
            location: Location::default(),
            view,
            offset_y: 0,
        })
    }
    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }

    fn move_point(&mut self, key_code: KeyCode) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size().unwrap_or_default();
        match key_code {
            KeyCode::Down => {
                if y < height - 1 {
                    y += 1;
                } else {
                    self.offset_y += 1;  // Scroll down
                }
            }
            KeyCode::Up => {
                if y > 0 {
                    y -= 1;
                } else if self.offset_y > 0 {
                    self.offset_y -= 1;  // Scroll up
                }
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location { x, y };
    }


    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code,
                modifiers,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                },
                (KeyCode::Char(c), _) => {
                    self.view.insert_char(self.location.y, self.location.x, c);
                    self.location.x += 1; // Move cursor right after inserting
                },
                (KeyCode::Backspace, _) => {
                    if self.location.x > 0 {
                        self.location.x -= 1;
                        self.view.delete_char(self.location.y, self.location.x);
                    }
                },
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                    if let Some(file_name) = env::args().nth(1) {
                        let _ = self.view.save_buffer(&file_name);
                    }
                },
                (
                    KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::PageDown
                    | KeyCode::PageUp
                    | KeyCode::End
                    | KeyCode::Home,
                    _,
                ) => {
                    self.move_point(code);
                },
                _ => {}
            },
            Event::Resize(width_u16, height_u16) => {
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                self.view.resize(Size { height, width });
            },
            _ => {}
        }
    }
    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render(self.offset_y);
        let _ = Terminal::move_caret_to(Position {
            col: self.location.x,
            row: self.location.y,
        });
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
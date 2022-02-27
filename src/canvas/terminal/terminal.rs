use crate::canvas::canvas::*;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::warn;
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    buffer::{Buffer, Cell},
    layout::Rect,
    widgets::Widget,
    Frame, Terminal,
};

pub struct TerminalCanvas {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl Canvas for TerminalCanvas {
    fn draw(&mut self, buffer: &[char]) -> Result<(), Box<dyn std::error::Error>> {
        let Dim { width, height } = self.dim();
        let area = Rect {
            x: 0,
            y: 0,
            // FIXME: Seems unidiomatic.
            width: width as u16,
            height: height as u16,
        };
        self.terminal.draw(|f| draw_image(f, area, buffer))?;
        Ok(())
    }

    fn dim(&self) -> Dim {
        match self.terminal.size() {
            Ok(frame) => Dim {
                width: frame.width.into(),
                height: frame.height.into(),
            },
            Err(err) => panic!("querying canvas dimension: {}", err),
        }
    }

    fn cell_aspect_ratio(&self) -> (u32, u32) {
        (1, 2)
    }
}

impl Drop for TerminalCanvas {
    fn drop(&mut self) {
        match self.cleanup() {
            Ok(_) => (),
            Err(err) => warn!("failed restoring terminal: {}", err),
        }
    }
}

fn draw_image<B: Backend>(f: &mut Frame<B>, area: Rect, buffer: &[char]) {
    f.render_widget(ASCIIImage { buffer }, area)
}

impl TerminalCanvas {
    /// new returns a new `TerminalCanvas` which implements the `Canvas` trait.
    pub fn new() -> Result<TerminalCanvas, std::io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(TerminalCanvas { terminal })
    }

    /// cleanup cleans up the terminal session and restores the terminal window to its original
    /// state.
    fn cleanup(&mut self) -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()
    }
}

/// ASCIIImage represents an ASCII encoded image.
struct ASCIIImage<'a> {
    buffer: &'a [char],
}

impl<'a> Widget for ASCIIImage<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let b = Buffer {
            area,
            content: self
                .buffer
                .iter()
                .map(|c| Cell::default().set_char(*c).clone())
                .collect(),
        };
        buf.merge(&b);
    }
}

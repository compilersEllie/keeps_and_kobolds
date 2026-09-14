use anyhow::{Context, Result};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};

use ratatui::{Terminal, backend::Backend};
use signal_hook::consts::signal::SIGTSTP;

use std::io::Write;
use std::sync::{Arc, Mutex};

use tokio_stream::StreamExt;
use tokio::sync::mpsc;

use crate::net::NetUpdate;
use crate::preferences::Preferences;

#[derive(Debug)]
pub enum InputSignal {
    Continue,
    Break,
    Suspend,
}

pub struct App<B: Backend> {
    pub preferences: Preferences,
    pub terminal: Terminal<B>,
    // TODO(feat): Add chat commands #3
    // TODO(feat): Add rx/tx channels for multiplayer #2
    to_network: mpsc::Sender<NetUpdate>,
    from_network: mpsc::Receiver<NetUpdate>,
    // TODO(feat): Add rx/tx channels for multiplayer chat #2
    // e.g. pub registry: CommandRegistry,
    // TODO(feat): Add times so messages can time out. #5
    pub chat_history: Arc<Mutex<Vec<String>>>,
}

impl<B: Backend + Write> App<B>
where
    <B as Backend>::Error: 'static + Sync + Send,
{
    pub fn new(
        terminal: Terminal<B>,
        to_network: mpsc::Sender<NetUpdate>,
        from_network: mpsc::Receiver<NetUpdate>,
    ) -> Result<Self> {
        let preferences = Preferences::load()?;
        let chat_history = Arc::new(Mutex::new(vec![]));
        let app = App {
            preferences,
            terminal,
            to_network,
            from_network,
            chat_history,
        };
        Ok(app)
    }

    pub async fn enter(&mut self) -> Result<()> {
        enable_raw_mode().with_context(|| "Failed to enable raw mode")?;
        execute!(
            self.terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        )?;
        self.terminal.clear()?;
        Ok(())
    }

    pub async fn leave(&mut self) -> Result<()> {
        disable_raw_mode().with_context(|| "Failed to disable raw mode")?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub async fn handle_input(&mut self) -> Result<InputSignal> {
        // TODO(feat): Accumulate input events via channels. #2
        // TODO(feat): Process events on this thread. #2

        if !event::poll(std::time::Duration::from_millis(10))? {
            return Ok(InputSignal::Continue);
        }
        let event = event::read()?;
        match event {
            event::Event::Mouse(mouse_event) => {
                log::debug!("{:?}", mouse_event);
            }
            event::Event::Key(key) => {
                log::debug!("{:?}", key);
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    return Ok(InputSignal::Break);
                }
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('z') {
                    return Ok(InputSignal::Suspend);
                }
            }
            other => {
                log::debug!("{:?}", other);
            }
        }
        Ok(InputSignal::Continue)
    }

    pub async fn run(&mut self) -> Result<()> {
        //loop {
            crate::world::wmain()?;
            // TODO(feat): Handle simulation on another thread #3
            // TODO(feat): Handle messages on another thread #3
            // TODO(feat): Handle rendering on another thread #3
            // TODO(feat): Render screen on this thread #3
       // }
        Ok(())
    }
}

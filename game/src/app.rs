use anyhow::{Context, Result};
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEvent,
};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};

use futures::FutureExt;
use ratatui::{Terminal, backend::Backend};
use signal_hook::consts::signal::SIGTSTP;
use tokio::task::JoinHandle;

use std::io::Write;
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use tokio_stream::StreamExt;

use crate::BUFFER_SIZE;
use crate::net::NetUpdate;
use crate::preferences::Preferences;

#[derive(Clone, Debug)]
pub enum UIEvent {
    // State changes:
    Init,
    Quit,
    Error,
    Closed,

    // Tasks:
    Tick,   // Simulation step
    Render, // Render update

    // Input:
    Event(Event),
}

#[derive(Debug)]
pub enum InputSignal {
    Continue,
    Break,
    Suspend,
}

pub struct App<B: Backend> {
    pub preferences: Preferences,
    // TODO(feat): Store in prefs #3
    tick_rate: f64,
    frame_rate: f64,
    // TODO(feat): Add chat commands #3
    // e.g. pub registry: CommandRegistry,
    pub chat_history: Arc<Mutex<Vec<String>>>,
    // TODO(feat): Add times so messages can time out. #5

    // IO:
    pub terminal: Terminal<B>,
    pub event_reader_task: Option<JoinHandle<()>>,

    // Channels:
    to_network: mpsc::Sender<NetUpdate>,
    from_network: mpsc::Receiver<NetUpdate>,

    to_event_handler: mpsc::Sender<UIEvent>,
    from_event_reader: mpsc::Receiver<UIEvent>,
    // TODO(feat): Add rx/tx channels for multiplayer #2
    // TODO(feat): Add rx/tx channels for multiplayer chat #2
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
        let (event_tx, event_rx) = mpsc::channel::<UIEvent>(BUFFER_SIZE);

        let app = App {
            preferences,
            tick_rate: 10.0,
            frame_rate: 30.0,
            terminal,
            event_reader_task: None,
            to_network,
            from_network,
            to_event_handler: event_tx,
            from_event_reader: event_rx,
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

    pub async fn setup_event_reader(&mut self) -> Result<()> {
        let tick_delay = std::time::Duration::from_secs_f64(1.0 / self.tick_rate);
        let render_delay = std::time::Duration::from_secs_f64(1.0 / self.frame_rate);
        let event_tx = self.to_event_handler.clone();
        self.event_reader_task = Some(tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_delay);
            let mut render_interval = tokio::time::interval(render_delay);
            loop {
                let tick_delay = tick_interval.tick();
                let render_delay = render_interval.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                  maybe_event = crossterm_event => {
                    match maybe_event {
                      Some(Ok(evt)) => {
                        match event_tx.send(UIEvent::Event(evt)).await {
                            Ok(()) => {},
                            Err(_) => break,
                        }
                      }
                      Some(Err(_)) => {
                        match event_tx.send(UIEvent::Error).await {
                        Ok(()) => {},
                        Err(_) => break,
                    }
                      }
                      None => {},
                    }
                  },
                  _ = tick_delay => {
                      match event_tx.send(UIEvent::Tick).await {
                      Ok(()) => {},
                      Err(_) => break,
                  }
                  },
                  _ = render_delay => {
                      match event_tx.send(UIEvent::Render).await {
                      Ok(()) => {},
                      Err(_) => break,
                  }
                  },
                }
            }
        }));
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        self.setup_event_reader().await?;

        loop {
            let event = tui.next().await?; // blocks until next event (including render and tick

            if let Event::Render = event.clone() {
              // application render
              tui.draw(|f| {
                ui(f, &app);
              })?;
            }

            // application update
            update(&mut app, event);

            // application exit
            if app.should_quit {
              break;
            }
        }
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

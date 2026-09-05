#![allow(unused_variables)] // TODO(cleanup): Remove #5
#![allow(unused)] // TODO(cleanup): Remove #5
use anyhow::Result;
use directories::ProjectDirs;
use ratatui::{Terminal, backend::CrosstermBackend};

mod actions;
mod app;
mod character;
mod effects;
mod goal;
mod item;
mod map;
mod menus;
mod net;
mod preferences;
mod render;
mod world;

mod typed_id;

use crate::app::App;

pub const QUALIFIER: &str = "systems";
pub const ORGANIZATION: &str = "mimir";
pub const APPLICATION: &str = "knk";

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

static mut LOGS_UNINITIALISED: bool = true;

#[cfg(not(target_arch = "wasm32"))]
fn build_logger(finish: impl FnOnce(&mut env_logger::Builder)) {
    if unsafe { LOGS_UNINITIALISED } {
        unsafe {
            LOGS_UNINITIALISED = false;
        }
        finish(
            env_logger::Builder::from_env(
                env_logger::Env::default()
                    .filter_or("RUST_LOG", "debug")
                    .write_style_or("RUST_LOG_STYLE", "AUTO"),
            )
            .format_timestamp(None),
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
pub fn ensure_initialized() {
    build_logger(|env| {
        let _ = env.is_test(true).try_init();
    });
}

#[cfg(target_arch = "wasm32")]
pub fn ensure_initialized() {
    if unsafe { LOGS_UNINITIALISED } {
        unsafe {
            LOGS_UNINITIALISED = false;
        }
        wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(not(test))]
pub fn ensure_initialized() {
    use std::fs::OpenOptions;
    build_logger(|env| {
        // TODO(fix): Use ProjectDirs for log dir #3
        let log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!(".{}.log", APPLICATION))
            .expect("Failed to setup log file.");
        env_logger::Builder::init(env.target(env_logger::fmt::Target::Pipe(Box::new(log_file))));
    });
    build_logger(env_logger::Builder::init);
}

pub fn dirs() -> ProjectDirs {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .expect("Couldn't find project directories")
}

#[tokio::main]
async fn main() -> Result<()> {
    ensure_initialized();

    let terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
    let mut app = App::new(terminal)?;

    // let gameNet = GameNet::new();
    net::main().await?;

    /*
    app.enter().await?;
    app.run().await?;
    app.leave().await?;
    */

    Ok(())
}

// TODO(feat): Use include_directories and directories to setup data #2
// TODO(feat): Use protest for testing #3
// TODO(feat): Setup ratatatui #1
// TODO(feat): Setup crossterm and crokey key handling#1
// TODO(feat): Setup log, pretty log, tracing #2
// TODO(feat): Setup dialogues with tui-markdown #2
// TODO(feat): Create descriptions from item stats #3
// TODO(feat): Create descriptions from story points #3
// TODO(feat): Support for mounts #3
// TODO(feat): Support for pets (on body, roving) #3
// TODO(feat): Support for parties (NPC ane multipkayer? #2
// TODO(feat): Design a fighting system #2
// TODO(feat): Design an action system #2
// TODO(feat): Design a settings menu #3
// TODO(feat): Design a main menu #3
// TODO(feat): Design a pause menu? #3
// TODO(feat): Setup player keybinds via toml #2
// TODO(feat): Setup redb for save games with multiple save slots #2
// TODO(feat): Setup multiplayer joining / friend codes and invites#5
// TODO(feat): Setup multiplayer game sync, use  ggrs or tokio-tungstenite websockets #1
// TODO(feat): Setup versioning for data and game and protocols #2
// TODO(feat): Setup AI systems for characters (pathing, fighting) #3
// TODO(feat): Setup auto releases with https://docs.cocogitto.io/ci_cd/action.html #1
// TODO(perf): Use https://github.com/lumol-org/soa-derive and rayon for faster arrays #4
// TODO(feat): Use a free relay server for p2p https://www.metered.ca/tools/openrelay/ or https://localxpose.io/tunneling-service #4
// TODO(feat): Use Steam for multiplayer networkinng https://docs.rs/steamworks/latest/steamworks/ #4

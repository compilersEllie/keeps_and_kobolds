#![allow(unused_variables)] // TODO(cleanup): Remove
#![allow(unused)] // TODO(cleanup): Remove

mod actions;
mod character;
mod effects;
mod goal;
mod item;
mod map;
mod menus;
mod render;
mod world;

mod typed_id;

fn main() {
    println!("Hello, world!");

    /*
    let mut x: f32 = 0;
    loop {
        let mut ix: u128 = x.round();
        x += 1;
        ix += 1;
        if x.round() != ix {
            println!("{x} != {ix}");
            break;
        }
    }
    */
}
// TODO(feat): Use include_directories and directories to setup data
// TODO(feat): Use protest for testing
// TODO(feat): Setup ratatatui
// TODO(feat): Setup crossterm and crokey key handling
// TODO(feat): Setup log & pretty log
// TODO(feat): Setup dialogues with tui-markdown
// TODO(feat): Create descriptions from item stats
// TODO(feat): Create descriptions from story points
// TODO(feat): Support for mounts
// TODO(feat): Support for pets (on body, roving)
// TODO(feat): Support for parties (NPC ane multipkayer?
// TODO(feat): Design a fighting system
// TODO(feat): Design an action system
// TODO(feat): Design a settings menu
// TODO(feat): Design a main menu
// TODO(feat): Design a pause menu?
// TODO(feat): Setup player keybinds via toml
// TODO(feat): Setup redb for save games with multiple save slots
// TODO(feat): Setup multiplayer joining / friend codes and invites
// TODO(feat): Setup multiplayer game sync
// TODO(feat): Setup versioning for data and game and protocols
// TODO(feat): Setup AI systems for characters (pathing, fighting)
// TODO(feat): Setup auto releases with https://docs.cocogitto.io/ci_cd/action.html

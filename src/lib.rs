#![feature(nll)]

extern crate byteorder;
extern crate ghakuf;
extern crate serialport;

mod serial;
mod midi_parser;
mod note_scheduler;
mod player;
mod song;
mod util;

pub use player::{Player, PlayerOptions};
pub use song::{Event, Song, Track};

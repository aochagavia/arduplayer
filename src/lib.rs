extern crate byteorder;
extern crate ghakuf;
extern crate serialport;

mod command;
mod serial;
mod midi_parser;
mod note_scheduler;
mod player;
mod song;
mod util;

pub use player::{Player, SongMetadata};
pub use song::{Event, Song, Track};

// See https://www.vgmusic.com/music/console/nintendo/nes/

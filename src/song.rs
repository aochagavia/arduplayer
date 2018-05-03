use std::collections::VecDeque;
use std::path::Path;

use { midi_parser, util};

pub struct Song {
    pub time_base: u16,
    pub tracks: Vec<Track>
}

pub struct Track {
    pub name: Option<String>,
    pub inner: Vec<Event>
}

impl Track {
    pub fn transpose(mut self, octaves: i8) -> Track {
        for event in &mut self.inner {
            match event {
                Event::Play { tone, .. }
                | Event::Stop { tone, .. } => *tone = util::transpose(*tone, octaves),
                _ => ()
            }
        }

        self
    }
}

#[derive(Copy, Clone)]
pub enum Event {
    Play { tone: u8, velocity: u8 },
    Stop { tone: u8 },
    Wait(u32)
}

impl Event {
    pub fn is_wait(self) -> bool {
        match self {
            Event::Wait(_) => true,
            _ => false
        }
    }

    pub fn unwrap_wait(&mut self) -> &mut u32 {
        match self {
            Event::Wait(x) => x,
            _ => panic!("Event is not a Wait!")
        }
    }
}

impl Song {
    pub fn from_midi<P: AsRef<Path>>(path: P) -> Song {
        midi_parser::load(path.as_ref())
    }
}

pub fn merge_tracks(tracks: Vec<Track>) -> Track {
    let mut events = Vec::new();

    let mut tracks: Vec<VecDeque<_>> = tracks.into_iter().map(|t| t.inner.into()).collect();

    // Iterate as long as there are events to process
    while tracks.iter().any(|t| t.len() > 0) {
        // If any track starts with something that is not a wait, pop it until it
        for track in &mut tracks {
            pop_non_waits(track, &mut events);
        }

        // We know that all remaining tracks start with waits, let's get the smallest one
        if let Some(&mut wait) = tracks.iter_mut().filter_map(|t| t.front_mut()).map(|w| w.unwrap_wait()).min() {
            // Now subtract the wait from all  waits
            for other_wait in tracks.iter_mut().filter_map(|t| t.front_mut()).map(|w| w.unwrap_wait()) {
                *other_wait -= wait;
            }

            // Don't forget to add the wait to our event queue
            events.push(Event::Wait(wait));
        }

        // Finally, remove all waits of 0
        for track in &mut tracks {
            if let Some(&mut 0) = track.front_mut().map(|w| w.unwrap_wait()) {
                track.pop_front();
            }
        }

        // Now go to the next iteration!
    }

    Track {
        name: None,
        inner: events
    }
}

fn pop_non_waits(track: &mut VecDeque<Event>, buf: &mut Vec<Event>) {
    loop {
        if track.is_empty() || track[0].is_wait() {
            break;
        }

        buf.push(track.pop_front().unwrap());
    }
}

// A simplified representation of a note
// pub struct Note {
//     id: u8,
//     channel: u8, // We can probably ignore channel information
//     velocity: u8,
//     on: bool
// }


//struct TrackFlattener {
//    tracks: Vec<Track>,
//    delta_times: Vec<u32>
//}
//
//impl TrackFlattener {
//    pub fn new(tracks: Vec<Track>) -> TrackFlattener {
//        let tracks_len = tracks.len();
//        TrackFlattener {
//            tracks,
//            delta_times: vec![0u32; tracks_len]
//        }
//    }
//
//
//
//    // Merges all tracks into one
//    pub fn flatten_tracks(&mut self) -> Track {
//        let mut merged = Vec::new();
//        while let Some(track_index) = self.next_note() {
//            // ...
//        }
//
//        merged
//    }
//
//    pub fn next_note(&mut self) -> Option<usize> {
//        let base_time = *self.delta_times.iter().min().unwrap();
//
//        // FIXME: we need to keep delta time of each note somewhere. Right now we throw it away
//        // FIXME 2: sometimes we ignore events, but those have a delta time as well...
//        // Throwing them away may cause problems with the timing of the notes... right?
//
//        // self.tracks.iter().enumerate().filter(|(_, t)| t.notes.len() > 0)
//
//        //.min_by_key(|&t| t.notes[0].delta_time )
//
//        unimplemented!()
//    }
//}

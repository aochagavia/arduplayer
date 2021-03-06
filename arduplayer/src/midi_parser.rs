use std::path::Path;

use ghakuf::reader::{Handler, Reader};
use ghakuf::messages::{MetaEvent, MidiEvent, SysExEvent};

use song::{Song, Track, Event};

/// A parser to extract NoteOn and NoteOff MIDI events, per track
pub struct MidiParser {
    time_base: Option<u16>,
    tracks: Vec<MidiTrack>,
}

/// Represents a track that is being parsed
///
/// Note that, after parsing, the events will be exported to a `song::Track`
pub struct MidiTrack {
    name: Option<String>,
    notes: Vec<Event>,
    unknown_events: u32,
    ignored_dt: u32
}

impl MidiTrack {
    fn next_wait(&mut self) -> u32 {
        let dt = self.ignored_dt;
        self.ignored_dt = 0;
        dt
    }
}

impl MidiParser {
    fn new() -> MidiParser {
        MidiParser { time_base: None, tracks: Vec::new() }
    }

    fn current_track_opt(&mut self) -> Option<&mut MidiTrack> {
        let i = self.tracks.len() - 1;
        self.tracks.get_mut(i)
    }

    fn current_track(&mut self) -> &mut MidiTrack {
        self.current_track_opt().unwrap()
    }

    fn add_track(&mut self) {
        let name = None;
        let notes = Vec::new();
        let unknown_events = 0;
        let ignored_dt = 0;
        self.tracks.push(MidiTrack { name, notes, unknown_events, ignored_dt });
    }

    /// Load a song located at the given path
    ///
    /// Note: panics if the file does not exist. Other errors are logged to stderr
    /// and ignored afterwards
    pub fn load_song(path: &Path) -> Song {
        let mut handler = MidiParser::new();
        let mut reader = Reader::new(&mut handler, path).unwrap();
        if let Some(err) = reader.read().err() {
            eprintln!("Error reading midi file: {}", err)
        }

        // debug_tracks(&handler.tracks);

        Song {
            time_base: handler.time_base.unwrap(),
            tracks: handler.tracks.into_iter().map(|t| Track::new(t.notes)).collect()
        }
    }
}

// fn debug_tracks(tracks: &[MidiTrack]) {
//     use note_scheduler::NoteScheduler;
//     println!("Tracks:");
//     for (index, t) in tracks.iter().enumerate() {
//         // Play the thing virtually
//         let mut player = NoteScheduler::new(50);
//         for &event in &t.notes {
//             match event {
//                 Event::Play { tone, .. } => {
//                     player.start_note(tone);

//                 }
//                 Event::Stop { tone } => {
//                     player.stop_note(tone);

//                 }
//                 Event::Wait(_) => {
//                     // Ignore wait events, since we are only interested in seeing how many
//                     // notes are played in parallel
//                 }
//             }
//         }

//         println!("{}. {} (min buzzers: {})",
//             index,
//             t.name.as_ref().unwrap_or(&String::from("<unknown>")),
//             player.playing.len()
//         );
//     }
// }

impl Handler for MidiParser {
    fn header(&mut self, _format: u16, _track: u16, time_base: u16) {
        assert_eq!(self.time_base, None);
        self.time_base = Some(time_base);
    }

    fn meta_event(&mut self, delta_time: u32, event: &MetaEvent, data: &Vec<u8>) {
        match event {
            &MetaEvent::SequenceOrTrackName => {
                assert_eq!(delta_time, 0);

                let name = String::from_utf8_lossy(data);
                self.current_track().name = Some(name.to_string());
                // println!("Loading track {}", name);
            }
            &MetaEvent::EndOfTrack => {
                //assert_eq!(delta_time, 0);

                // Track ending, we can safely ignore it
            }
            _ => {
                self.current_track().ignored_dt += delta_time;
                // println!("Meta event: {} {}", delta_time, event);
            }
        }
    }

    fn midi_event(&mut self, delta_time: u32, event: &MidiEvent) {
        // Note: we are ignoring channel information

        // Important: some files encode `NoteOff` as a `NoteOn` with velocity 0
        match *event {
            MidiEvent::NoteOn { note, velocity: 0, .. } => {
                let wait = Event::Wait(self.current_track().next_wait() + delta_time);
                let stop = Event::Stop { tone: note };

                self.current_track().notes.push(wait);
                self.current_track().notes.push(stop);
            }
            MidiEvent::NoteOn { note, velocity, .. } => {
                let wait = Event::Wait(self.current_track().next_wait() + delta_time);
                let play = Event::Play { tone: note, velocity };

                self.current_track().notes.push(wait);
                self.current_track().notes.push(play);
            }
            MidiEvent::NoteOff { note, .. } => {
                let wait = Event::Wait(self.current_track().next_wait() + delta_time);
                let stop = Event::Stop { tone: note };

                self.current_track().notes.push(wait);
                self.current_track().notes.push(stop);
            }
            MidiEvent::ProgramChange { .. } => {
                // Looks like the program change event is used to set the instrument
                // Therefore we can ignore it

                self.current_track().ignored_dt += delta_time;
            }
            MidiEvent::ControlChange { .. } => {
                // Example control changes: vibrato, piano pedal, reverb, etc
                // Since our little buzzers don't have anything like this, we can ignore it

                self.current_track().ignored_dt += delta_time;
            }
            _ => {
                // println!("Unknown midi event: {}", event);

                self.current_track().ignored_dt += delta_time;
                self.current_track().unknown_events += 1;
            }
        }
    }

    fn sys_ex_event(&mut self, delta_time: u32, _event: &SysExEvent, _data: &Vec<u8>) {
        // println!("Sys Ex event found, ignoring track: {} {}", delta_time, event);

        self.current_track().ignored_dt += delta_time;
        self.current_track().unknown_events += 1;
    }

    fn track_change(&mut self) {
        // println!("Track change");
        self.add_track();
    }
}

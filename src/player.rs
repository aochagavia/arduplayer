use std::time::Duration;
use std::thread;

use serialport::SerialPort;

use command::Command;
use note_scheduler::NoteScheduler;
use song::{self, Event, Song};

use {serial, util};

pub struct SongMetadata<'a> {
    pub tracks: &'a [(usize, i8)],
    pub delay_mul: f64
}

pub struct Player {
    port: Box<SerialPort>,
    scheduler: NoteScheduler
}

impl Player {
    pub fn new(speakers: u8) -> Player {
        let port_name = serial::get_port_name();
        let port = serial::open_port(&port_name);
        let scheduler = NoteScheduler::new(speakers);

        Player { port, scheduler }
    }

    pub fn play_song(&mut self, song: Song, metadata: SongMetadata) {
        // Track merging:
        // * Remove repeated notes per track
        // * If a note is repeated between tracks, we will need to nest it
        //   (i.e. in [PLAY A, PLAY A, STOP A, STOP A], A should only stop at the end)

        let keep = |id| metadata.tracks.iter().find(|&&(track_id, _)| id == track_id);

        let tracks: Vec<_> = song.tracks.into_iter().enumerate()
            // Keep only the tracks that are mentioned in the metadata
            .filter_map(|(i, t)| (keep)(i).map(|&(_, transpose)| (t, transpose)))
            // Transpose them
            .map(|(t, transpose)| t.transpose(transpose))
            .collect();

        let track = song::merge_tracks(tracks);
        let track_transpose = 0;

        for event in &track.inner {
            match *event {
                Event::Play { tone, .. } => {
                    let tone = util::transpose(tone, track_transpose);
                    if let Some(speaker_id) = self.scheduler.start_note(tone) {
                        let freq = util::midi_code_to_freq(tone).unwrap();
                        Command::new(speaker_id, freq).write(&mut *self.port).expect("Something went wrong");
                    }
                }
                Event::Stop { tone } => {
                    let tone = util::transpose(tone, track_transpose);
                    if let Some(speaker_id) = self.scheduler.stop_note(tone) {
                        Command::new(speaker_id, 0).write(&mut *self.port).expect("Something went wrong");
                    }
                }
                Event::Wait(time) => {
                    if time != 0 {
                        thread::sleep(Duration::from_millis((time as f64 * metadata.delay_mul) as u64));
                    }
                }
            }
        }

        // Just in case, stop playing in all the speakers
    }

    pub fn play_note(&mut self, midi_code: u8, on: bool) {
        // FIXME: this should have no delay on the arduino side!
        // Get available speaker, if any
        let speaker_id = if on {
            self.scheduler.start_note(midi_code)
        } else {
            self.scheduler.stop_note(midi_code)
        };

        if let Some(speaker_id) = speaker_id {
            let freq = if on {
                util::midi_code_to_freq(midi_code).unwrap()
            } else {
                0
            };

            Command::new(speaker_id, freq ).write(&mut *self.port).expect("Something went wrong");
        }
    }
}

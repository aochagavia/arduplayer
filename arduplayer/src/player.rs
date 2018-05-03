use std::time::Duration;
use std::thread;

use serialport::{self, SerialPort};

use note_scheduler::NoteScheduler;
use song::{self, Event, Song};

use {serial, util};

/// Options to be used when playing a MIDI file
pub struct PlayerOptions<'a> {
    /// Pairs of track number and desired transposition
    ///
    /// Tracks that are not in this list will be ignored
    pub tracks: &'a [(usize, i8)],
    /// Higher means slower playback
    pub delay_mul: f64
}

impl<'a> PlayerOptions<'a> {
    pub fn borrow(&'a self) -> PlayerOptions<'a> {
        PlayerOptions {
            tracks: self.tracks,
            delay_mul: self.delay_mul
        }
    }
}

/// Arduplayer's main interface to play songs and notes
pub struct Player {
    port: Box<SerialPort>,
    scheduler: NoteScheduler
}

impl Player {
    /// Create a new `Player` with the given number of buzzers
    pub fn new(buzzers: u8) -> Result<Player, serialport::Error> {
        let port_name = serial::get_port_name();
        let port = serial::open_port(&port_name)?;
        let scheduler = NoteScheduler::new(buzzers);

        Ok(Player { port, scheduler })
    }

    /// Play the `Song` using the provided `PlayerOptions`
    pub fn play_song(&mut self, song: Song, options: PlayerOptions) {
        // Filter out track numbers not mentioned in the options (useful to get
        // rid of tracks that are too noisy or useless ones like drums)
        let keep = |id| options.tracks.iter().find(|&&(track_id, _)| id == track_id);
        let tracks: Vec<_> = song.tracks.into_iter().enumerate()
            // Keep only the tracks that are mentioned in the options
            .filter_map(|(i, track)| (keep)(i).map(|&(_, transpose)| (track, transpose)))
            // Transpose them
            .map(|(track, transpose)| track.transpose(transpose))
            .collect();

        let track = song::merge_tracks(tracks);

        for event in track.events() {
            match *event {
                Event::Play { tone, .. } => self.play_note(tone, true),
                Event::Stop { tone } => self.play_note(tone, false),
                Event::Wait(time) => {
                    if time != 0 {
                        thread::sleep(Duration::from_millis((time as f64 * options.delay_mul) as u64));
                    }
                }
            }
        }
    }

    /// Play (or stop playing) a single note
    pub fn play_note(&mut self, midi_code: u8, on: bool) {
        // Get available buzzer, if any
        let maybe_buzzer_id = if on {
            self.scheduler.start_note(midi_code)
        } else {
            self.scheduler.stop_note(midi_code)
        };

        if let Some(buzzer_id) = maybe_buzzer_id {
            let freq = if on {
                util::midi_code_to_freq(midi_code).unwrap()
            } else {
                0
            };

            serial::write_note(&mut *self.port, buzzer_id, freq).expect("Something went wrong");
        }
    }
}

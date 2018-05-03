use std::collections::HashMap;

// Schedules notes to be played in one of the speakers
pub struct NoteScheduler {
    pub playing: HashMap<u8, usize>,
    speakers: Vec<bool>,
    // The variable below is used for debugging purposes
    pub playing_count: u8,
    pub wrong_count: u32
}

impl NoteScheduler {
    pub fn new(speakers: u8) -> NoteScheduler {
        NoteScheduler {
            playing: HashMap::new(),
            speakers: vec![false; speakers as usize],
            playing_count: 0,
            wrong_count: 0
        }
    }

    // Register the note as playing and return the speaker it should be played in
    pub fn start_note(&mut self, note: u8) -> Option<u8> {
        // Ignore this note if it is already being played
        if let Some(&speaker_index) = self.playing.get(&note) {
            println!("Attempt to start playing a note that is being played: {}", self.wrong_count);
            self.wrong_count += 1;
            return Some(speaker_index as u8);
        }

        // Find an available speaker and use it
        if let Some((index, speaker)) = self.speakers.iter_mut().enumerate().find(|&(_, &mut in_use)| !in_use) {
            *speaker = true;
            self.playing.insert(note, index);

            // Keep track of the amount of notes being played at the same time
            self.playing_count = ::std::cmp::max(self.playing.len() as u8, self.playing_count);

            Some(index as u8)
        } else {
            None
        }
    }

    pub fn stop_note(&mut self, note: u8) -> Option<u8> {
        // Ensure the note is already playing, and remove it from the speaker
        if let Some(speaker) = self.playing.remove(&note) {
            // Speaker is no longer in use
            self.speakers[speaker] = false;
            Some(speaker as u8)
        } else {
            None
        }
    }
}

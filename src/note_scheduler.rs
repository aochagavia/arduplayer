// Schedules notes to be played in one of the speakers
pub struct NoteScheduler {
    pub playing: Vec<(u8, usize)>,
    speakers: Vec<bool>,
    // The variables below are used for debugging purposes
    pub playing_max_count: u8,
    pub wrong_count: u32
}

impl NoteScheduler {
    pub fn new(speakers: u8) -> NoteScheduler {
        NoteScheduler {
            playing: Vec::with_capacity(speakers as usize),
            speakers: vec![false; speakers as usize],
            playing_max_count: 0,
            wrong_count: 0
        }
    }

    // Register the note as playing and return the speaker it should be played in
    pub fn start_note(&mut self, note: u8) -> Option<u8> {
        // Find an available speaker and use it
        if let Some((index, speaker)) = self.speakers.iter_mut().enumerate().find(|&(_, &mut in_use)| !in_use) {
            *speaker = true;
            self.playing.push((note, index));

            // Keep track of the amount of notes being played at the same time
            self.playing_max_count = ::std::cmp::max(self.playing.len() as u8, self.playing_max_count);

            Some(index as u8)
        } else {
            None
        }
    }

    pub fn stop_note(&mut self, note: u8) -> Option<u8> {
        // Ensure the note is already playing, and remove it from the speaker
        if let Some(pos) = self.playing.iter().position(|&(n, _)| n == note) {
            let (_, speaker) = self.playing.swap_remove(pos);
            // Speaker is no longer in use
            self.speakers[speaker] = false;
            Some(speaker as u8)
        } else {
            None
        }
    }
}

/// A data structure to keep track of the buzzers that are free and
/// assign them to new notes when required
pub struct NoteScheduler {
    pub playing: Vec<(u8, usize)>,
    buzzers: Vec<bool>,
    // The variables below are used for debugging purposes
    pub playing_max_count: u8,
    pub wrong_count: u32
}

impl NoteScheduler {
    /// Create a new scheduler with the given amount of buzzers
    pub fn new(buzzers: u8) -> NoteScheduler {
        NoteScheduler {
            playing: Vec::with_capacity(buzzers as usize),
            buzzers: vec![false; buzzers as usize],
            playing_max_count: 0,
            wrong_count: 0
        }
    }

    /// Register the note as playing and return the buzzer it should be played in
    pub fn start_note(&mut self, note: u8) -> Option<u8> {
        // Find an available buzzer and use it
        if let Some((index, buzzer)) = self.buzzers.iter_mut().enumerate().find(|&(_, &mut in_use)| !in_use) {
            *buzzer = true;
            self.playing.push((note, index));

            // Keep track of the amount of notes being played at the same time
            self.playing_max_count = ::std::cmp::max(self.playing.len() as u8, self.playing_max_count);

            Some(index as u8)
        } else {
            None
        }
    }

    /// Register that the note has stopped playing and return the buzzer it was playing on
    pub fn stop_note(&mut self, note: u8) -> Option<u8> {
        // Ensure the note is already playing, and remove it from the buzzer
        if let Some(pos) = self.playing.iter().position(|&(n, _)| n == note) {
            let (_, buzzer) = self.playing.swap_remove(pos);
            // buzzer is no longer in use
            self.buzzers[buzzer] = false;
            Some(buzzer as u8)
        } else {
            None
        }
    }
}

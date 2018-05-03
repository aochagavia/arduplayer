pub fn transpose(midi_code: u8, octaves: i8) -> u8 {
    let result = (midi_code as i16) + octaves as i16 * 12;
    if 24 <= result && result <= 107 {
        result as u8
    } else {
        panic!("Transpose out of bounds: {} by {} octaves", midi_code, octaves)
    }
}

pub fn midi_code_to_freq(note: u8) -> Option<u16> {
    let in_range = 24 <= note && note <= 107;
    if in_range {
        Some(NOTE_FREQS[(note - 24) as usize])
    } else {
        None
    }
}

pub fn freq_to_delay(freq: u16) -> u16 {
    // Frequency is in hertz (times per second)
    // C4 has a frequency of 262, which means that it goes on and off 262 times per second
    // This formula calculates the delay needed to generate that frequency

    if freq == 0 {
        return 0;
    }

    // The period is measured in microseconds and is divided by two because on and off
    (1000.0 * 1000.0 / 2.0 / freq as f64) as u16
}

// Notes from C1 to B7, equivalent to the range [24, 107] in midi notation (84 notes)
const NOTE_FREQS: &[u16] = &[33, 35, 37, 39, 41, 44, 46, 49, 52, 55, 58, 62, 65, 69, 73, 78, 82, 87,
    93, 98, 104, 110, 117, 123, 131, 139, 147, 156, 165, 175, 185, 196, 208, 220, 233, 247, 262,
    277, 294, 311, 330, 349, 370, 392, 415, 440, 466, 494, 523, 554, 587, 622, 659, 698, 740, 784,
    831, 880, 932, 988, 1047, 1109, 1175, 1245, 1319, 1397, 1480, 1568, 1661, 1760, 1865, 1976,
    2093, 2217, 2349, 2489, 2637, 2794, 2960, 3136, 3322, 3520, 3729, 3951];

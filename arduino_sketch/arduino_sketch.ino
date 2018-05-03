#include <stdint.h>

#include "note_buf.h"
#include "soft_pwm.h"

NoteBuf note_buf;
SoftPWM soft_pwm;

void setup() {
    soft_pwm.setup_pins();
    pinMode(7, INPUT_PULLUP);
    Serial.begin(9600);
}

void loop() {
    // Update 
    read_notes_from_serial();
    //play_notes_from_buffer();

    // Turn pins on and off in a synchronized way so we can generate the correct
    // square waves for each buzzer
    soft_pwm.tick();
}

// Amount of milliseconds to wait between receiving a note from serial and
// playing it. This is required to prevent problems when lots of notes are
// played in a short time.
uint32_t NOTE_WAIT = 0000;

void read_notes_from_serial() {
    // Message structure:
    // * 1 byte  - pin number
    // * 2 bytes - note delay
    while (Serial.available() > 2) {
        uint8_t bytes[3];
        Serial.readBytes(bytes, 3);

        uint8_t pin_id = bytes[0];
        uint16_t delay = bytes_to_int(bytes[1], bytes[2]);

        //Note note;
        //note.pin_id = pin_id;
        //note.delay = delay;
        //note.play_at = millis() + NOTE_WAIT;

        soft_pwm.set_delay(pin_id, delay);
        //note_buf.push_back(note);
    }
}

// Note: we assume little-endian
uint16_t bytes_to_int(uint8_t x, uint8_t y) {
    return ((uint16_t) x) | (((uint16_t) y) << 8);
}

void play_notes_from_buffer() {
    // Process notes in the buffer
    while (true) {
        if (note_buf.empty())
          break;

        Note& note = note_buf.peek_front();

        // Stop if the next note is not yet scheduled
        if (note.play_at > millis())
          break;

        soft_pwm.set_delay(note.pin_id, note.delay);

        // Don't forget to remove the note from the buffer
        note_buf.pop_front();
    }
}

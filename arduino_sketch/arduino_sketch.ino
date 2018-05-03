#include <stdint.h>

#include "soft_pwm.h"

void setup() {
    soft_pwm::setup_pins();
    Serial.begin(9600);
}

void loop() {
    read_notes_from_serial();

    // Turn pins on and off in a synchronized way so we can generate the correct
    // square waves for each buzzer
    soft_pwm::tick();
}

void read_notes_from_serial() {
    // Message structure:
    // * 1 byte  - pin number
    // * 2 bytes - note delay
    while (Serial.available() > 2) {
        uint8_t bytes[3];
        Serial.readBytes(bytes, 3);

        uint8_t pin_id = bytes[0];
        uint16_t delay = bytes_to_int(bytes[1], bytes[2]);

        soft_pwm::set_delay(pin_id, delay);
    }
}

// Note: we assume little-endian
uint16_t bytes_to_int(uint8_t x, uint8_t y) {
    return ((uint16_t) x) | (((uint16_t) y) << 8);
}


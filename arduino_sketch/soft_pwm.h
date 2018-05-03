const uint8_t PIN_MAP_SIZE = 6;
uint8_t pin_map[PIN_MAP_SIZE] = { 8, 9, 10, 11, 12, 7 };
uint16_t pin_delays[PIN_MAP_SIZE] = { 0, 0, 0, 0, 0, 0 };
uint16_t pin_last_writes[PIN_MAP_SIZE] = { 0, 0, 0, 0, 0, 0 };
uint16_t pin_next_writes[PIN_MAP_SIZE] = { 0, 0, 0, 0, 0, 0 };

class SoftPWM {
    public:
    void setup_pins() {
        for (int i = 0; i < PIN_MAP_SIZE; i++) {
            pinMode(pin_map[i], OUTPUT);
        }
    }

    void set_delay(uint8_t pin_id, uint16_t delay) {
        // Ignore out of range pin ids
        if (pin_id >= PIN_MAP_SIZE) {
            return;
        }

        // Pins that stop playing will not be toggled again (see `tick`),
        // which means they could end up in a HIGH state. Therefore we need to
        // manually set them to LOW.
        if (delay == 0) {
            digitalWrite(pin_map[pin_id], LOW);
        }

        // Set the delay of the pin, so we start using it next tick
        pin_delays[pin_id] = delay;
    }

    void tick() {
        uint16_t now = micros();
        for (int i = 0; i < PIN_MAP_SIZE; i++) {
            // Skip pins that are not playing
            if (!pin_delays[i])
                continue;

            // We use a 50% duty cycle of variable frequency
            // frequency (Hz) = 10^6 / 2 / delay
            if (now - pin_last_writes[i] > pin_delays[i]) {
                pin_last_writes[i] = now;
                pin_next_writes[i] = !pin_next_writes[i];

                digitalWrite(pin_map[i], pin_next_writes[i]);
            }
        }
    }
};

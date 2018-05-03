// We assume less than 32 notes will be played per second
const uint8_t RING_BUF_CAP = 64;

// A struct representing a note
struct Note {
  // The note will start playing in the future, at the time specified by `play_at`
  uint32_t play_at = 0;
  // The delay required to generate the desired frequency, measured in microseconds (0 means stop playing)
  uint16_t delay = 0;
  // The speaker's pin id associated to this note
  uint8_t pin_id = 0;
};

// A buffer representing a queue of notes
class NoteBuf {
  Note buf[RING_BUF_CAP];
  uint8_t front_ix = 0;
  uint8_t length = 0;

  public:
  Note& peek_front() {
    if (this->length > 0) {
      return this->buf[this->front_ix];
    }
  }

  void pop_front() {
    if (this->length > 0) {
      this->front_ix = (this->front_ix + 1) % RING_BUF_CAP;
      this->length--;
    }
  }

  void push_back(Note note) {
    if (this->length < RING_BUF_CAP) {
      this->buf[(this->front_ix + this->length) % RING_BUF_CAP] = note;
      this->length++;
    }
  }

  bool empty() {
    return this->length == 0;
  }
};

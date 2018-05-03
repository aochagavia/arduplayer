extern crate arduplayer;
extern crate ggez;

mod gui;

use arduplayer::Player;

fn main() {
    // Improve GUI
    // * Give a color to the key that is being played

    let mut player = Player::new(6).expect("Failed to initialize serial port");
    gui::run_gui(|tone, on| {
        player.play_note(tone + 60, on);
        println!("Play: tone {}", tone);
    });
}

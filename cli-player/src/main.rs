extern crate arduplayer;
extern crate rand;

use std::collections::HashMap;
use std::env;
use std::hash::Hash;

use arduplayer::{Player, PlayerOptions, Song};
use rand::Rng;

fn main() {
    let songs = songs();

    if let Some(song_name) = env::args().nth(1) {
        let song_name = if song_name == "random" {
            random_key(&songs)
        } else {
            &*song_name
        };

        let mut player = Player::new(6).expect("Could not initialize serial port");
        let song = Song::from_midi(format!("music/{}.mid", song_name));
        let options = &songs[song_name];
        player.play_song(song, options.borrow());
    } else {
        println!("Please specify a song:");
        for song_name in songs.keys() {
            println!("* {}", song_name);
        }
    }

}

fn random_key<K: Eq + Hash, V>(map: &HashMap<K, V>) -> &K {
    let songs: Vec<_> = map.keys().collect();
    let chosen = rand::thread_rng().choose(&songs);
    *chosen.unwrap()
}

fn songs() -> HashMap<&'static str, PlayerOptions<'static>> {
    let mut map = HashMap::new();

    map.insert("PkmRS-Center", PlayerOptions {
        tracks: &[(1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
        delay_mul: 5.0
    });

    map.insert("SSBKirbyStage", PlayerOptions {
        tracks: &[(1, 0), (2, 0), (4, 0), (5, 0), (6, 0), (8, 0), (9, 0), (10, 0), (11, 2), (13, 0)],
        delay_mul: 3.0
    });

    map.insert("cliffs", PlayerOptions {
        tracks: &[(2, 0)],
        delay_mul: 0.5
    });

    map.insert("pacman", PlayerOptions {
        tracks: &[(1, 0), (2, 0)],
        delay_mul: 5.0
    });

    map.insert("smwintro", PlayerOptions {
        tracks: &[(1, -1), (2, -1), (3, -1), (4, -1)],
        delay_mul: 3.0
    });

    map.insert("OoTBoF", PlayerOptions {
        tracks: &[(1, 0), (2, 0), (4, 0), (6, 0)],
        delay_mul: 2.0
    });

    map.insert("SSB_hammer", PlayerOptions {
        tracks: &[(1, 0), (2, 0)],
        delay_mul: 1.0
    });

    map.insert("Fox_Wins", PlayerOptions {
        tracks: &[(0, 0)],
        delay_mul: 5.0
    });

    map.insert("HappyBirthday", PlayerOptions {
        tracks: &[(1, 0), (2, 0), (3, 0)],
        delay_mul: 3.0
    });

    map.insert("OoTSoT", PlayerOptions {
        tracks: &[(1, 1), (2, 1), (4, 0)],
        delay_mul: 2.0
    });

    map.insert("symph40", PlayerOptions {
        tracks: &[(1, 0), (2, 0), (3, 0), (4, 0)],
        delay_mul: 0.5
    });

    map.insert("Z64gerud", PlayerOptions {
        tracks: &[(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0)],
        delay_mul: 4.0
    });

    map
}

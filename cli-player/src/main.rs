extern crate arduplayer;

use arduplayer::{Player, SongMetadata, Song};

// const SONGS: &[(&str, &[u8], &[i8], f64)] = &[
//     ("smwintro", &[(1, -1)], 4.0), // 2 speakers
//     ("pacman", &[(1, 0), (2, 0)], 5.0), // 1 speaker
//     ("cliffs", &[2], &[0], 0.5), // 4 speakers
// ];
// * PkmRS-Center.mid, &[(1, 0), (2, 0), (3, 0), (4, 0), (5, 0)], 5.0
// * SSBKirbyStage.mid, &[(1, 0), (2, 0), (4, 0), (5, 0), (6, 0), (8, 0), (9, 0), (10, 0), (11, 2), (13, 0)], 3.0
// * HappyBirthday.mid, &[(1, 0), (2, 0), (3, 0)], 3.0
// * Song_of_time_Bluegrass.mid, &[(1, 1), (2, 1), (4, 0)], time_mul 2
// * wilhelmus.mid, &[(1, 0), (2, -1)], time_mul 1.0


// fn songs() -> HashMap<&'static str, SongMetadata> {
//     SONGS.into_iter().map(|&(name, tracks, transpose, delay_mul)| {
//         (name, SongMetadata { tracks, transpose, delay_mul })
//     }).collect()
// }

fn main() {
    // Songs that work well
    // * Wim Sonneveld - Het Dorp.mid, 2 speakers, track 6, time_mul 3
    // * SSB_hammer.mid, 1 speaker, tracks [1, 2], time_mul 1 (would improve a bit with track merging)
    // * OoTBoF.mid track 3, time_mul 2 (needs track merging)
    // * Ocarina_of_time, track 1, transpose -1, time_mul 2
    // * epona.mid, track 1, time_mul 2
    // * zeldas_lullaby.mid, track 1, time_mul 0.5 (would improve with track merging)
    // * SSB_link_stage.mid, 1 speaker, tracks [1, 4, 7+1] time_mul 3 (needs track merging)
    // * Z64gerud.mid, 1 speaker, track 5, time_mul 5 (takes a while to begin, needs track merging)
    // &[(1, 2), (2, 2), (4, 2), (5, 2), (6, 2), (7, 2), (8, 2), (9, 2), (11, 2), (12, 2), (13, 2), (14, 2), (15, 2), (16, 2), (17, 2), (18, 2), (19, 2), (20, 2), (21, 2), (22, 2), (23, 2)]

    let mut player = Player::new(6);
    let metadata = SongMetadata {
        tracks: &[(1, 0), (2, 0), (3, 0), (4, 0), (5, 0)],
        delay_mul: 5.0
    };

    let song = Song::from_midi("music/PkmRS-Center.mid");
    player.play_song(song, metadata);
}

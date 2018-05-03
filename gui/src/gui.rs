use ggez::conf;
use ggez::graphics::{self, DrawMode, Color};
use ggez::event::{self, Keycode, Mod};
use ggez::timer;
use ggez::{Context, GameResult};

use arduplayer::Player;

pub fn run_gui(player: Player)
{
    let mut c = conf::Conf::new();
    c.window_setup.title = "Retro keyboard".into();
    let ctx = &mut Context::load_from_conf("arduplayer", "ggez", c).unwrap();

    let state = &mut GuiState { player };
    event::run(ctx, state).unwrap();
}

struct GuiState {
    player: Player
}

impl event::EventHandler for GuiState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        draw_octave(ctx, self.player.playing());
        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if repeat {
            return;
        }

        if let Some(tone) = key_to_midi_tone(keycode) {
            self.player.play_note(tone + 60, true);
            println!("Play: tone {}", tone);
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if repeat {
            return;
        }

        if let Some(tone) = key_to_midi_tone(keycode) {
            self.player.play_note(tone + 60, false);
            println!("Play: tone {}", tone);
        }
    }
}

fn key_to_midi_tone(keycode: Keycode) -> Option<u8> {
    use self::Keycode::*;
    Some(match keycode {
        A => 0, // C0
        W => 1,
        S => 2,
        E => 3,
        D => 4,
        F => 5,
        T => 6,
        G => 7,
        Y => 8,
        H => 9,
        U => 10,
        J => 11,
        K => 12, // C2
        O => 13,
        L => 14,
        P => 15,
        Semicolon => 16,
        _ => return None
    })
}

fn draw_octave(ctx: &mut Context, playing: impl Iterator<Item=u8>) {
    graphics::set_color(ctx, Color::from_rgb(255, 255, 255)).unwrap();

    let playing: Vec<_> = playing.collect();

    // This is horrible... But it works!!!!!! IT WORKS!
    fn i_to_pos(i: u8) -> u8 {
        let x = i % 12;
        let y = i / 12;
        (if x <= 4 {
            x
        } else {
            x + 1
        } + (y * 12 + y * 2))
        // match i {
        //     0 => 0,
        //     1 => 1, // -
        //     2 => 2,
        //     3 => 3, // -
        //     4 => 4,
        //     5 => 6,
        //     6 => 7, // -
        //     7 => 8,
        //     8 => 9, // -
        //     9 => 10,
        //     10 => 11, // -
        //     11 => 12,
        //     _ => unreachable!()
        // }
        //     12 => 14 !!
    }

    for i in 0..17 {
        let pos = i_to_pos(i);
        if playing.contains(&(i + 60)) {
            graphics::set_color(ctx, Color::from_rgb(100, 255, 100)).unwrap();
        } else {
            graphics::set_color(ctx, Color::from_rgb(255, 255, 255)).unwrap();
        }
        if pos % 2 == 0 {
            let pos = pos / 2;
            graphics::rectangle(ctx, DrawMode::Fill, graphics::Rect::new(pos as f32 * 62., 0., 60., 400.)).unwrap();
        }
    }

    for i in 0..17 {
        let pos = i_to_pos(i);
        graphics::set_color(ctx, Color::from_rgb(0, 0, 0)).unwrap();
        if pos % 2 != 0 {
            let pos = (pos - 1) / 2;
            graphics::rectangle(ctx, DrawMode::Fill, graphics::Rect::new(pos as f32 * 62. + 47.0, 0., 30., 270.)).unwrap();

            if playing.contains(&(i + 60)) {
                graphics::set_color(ctx, Color::from_rgb(100, 255, 100)).unwrap();
                graphics::rectangle(ctx, DrawMode::Fill, graphics::Rect::new(pos as f32 * 62. + 48.0, 0., 28., 269.)).unwrap();
            }
        }
    }
}

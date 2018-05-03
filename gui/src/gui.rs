use ggez::conf;
use ggez::graphics::{self, DrawMode, Color};
use ggez::event::{self, Keycode, Mod};
use ggez::{Context, GameResult};

pub fn run_gui<F: FnMut(u8, bool)>(input_handler: F)
{
    let mut c = conf::Conf::new();
    c.window_setup.title = "Retro keyboard".into();
    let ctx = &mut Context::load_from_conf("input_test", "ggez", c).unwrap();

    let state = &mut GuiState { input_handler };
    event::run(ctx, state).unwrap();
}

struct GuiState<F: FnMut(u8, bool)> {
    input_handler: F
}

impl<F: FnMut(u8, bool)> event::EventHandler for GuiState<F> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        draw_octave(ctx);
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if repeat {
            return;
        }

        if let Some(tone) = key_to_midi_tone(keycode) {
            (self.input_handler)(tone, true);
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if repeat {
            return;
        }

        if let Some(tone) = key_to_midi_tone(keycode) {
            (self.input_handler)(tone, false);
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

fn draw_octave(ctx: &mut Context) {
    graphics::set_color(ctx, Color::from_rgb(255, 255, 255)).unwrap();
    for i in 0..8 {
        graphics::rectangle(ctx, DrawMode::Fill, graphics::Rect::new(i as f32 * 62., 0., 60., 500.)).unwrap();
    }

    graphics::set_color(ctx, Color::from_rgb(0, 0, 0)).unwrap();

    for i in 0..2 {
        graphics::rectangle(ctx, DrawMode::Fill, graphics::Rect::new(i as f32 * 62. + 47.0, 0., 30., 300.)).unwrap();
    }

    for i in 0..3 {
        graphics::rectangle(ctx, DrawMode::Fill, graphics::Rect::new(i as f32 * 62. + (4. * 62. - 15.0), 0., 30., 300.)).unwrap();
    }
}

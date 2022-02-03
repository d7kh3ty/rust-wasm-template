use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::Window;
use std::process;

// define any data that is required by the main loop
pub struct State {
    sdl_context: sdl2::Sdl,
    canvas: sdl2::render::Canvas<Window>,
    rect: sdl2::rect::Rect,
}

impl State {
    pub fn new() -> Result<Self, anyhow::Error> {
        let sdl_context = sdl2::init().unwrap();

        let window = sdl_context
            .video()
            .unwrap()
            .window("rust_to_js", 640, 400)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        let rect = Rect::new(10, 10, 10, 10);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        Ok(Self {
            sdl_context,
            canvas,
            rect,
        })
    }
}

fn event_loop(state: &mut State) {
    let rect = &mut state.rect;
    let mut events = state.sdl_context.event_pump().unwrap();
    for event in events.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                process::exit(1);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                rect.x -= 10;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                rect.x += 10;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                rect.y -= 10;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                rect.y += 10;
            }
            _ => {}
        }
    }

    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);

    let canvas = &mut state.canvas;

    let _ = canvas.set_draw_color(black);
    let _ = canvas.clear();
    let _ = canvas.set_draw_color(white);
    let _ = canvas.fill_rect(*rect);

    let _ = canvas.present();
}

impl emscripten_main_loop::MainLoop for State {
    fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
        event_loop(self);
        emscripten_main_loop::MainLoopEvent::Continue
    }
}

fn main() {
    let mut state = State::new().unwrap();

    #[cfg(target_os = "emscripten")]
    emscripten_main_loop::run(state);

    #[cfg(not(target_os = "emscripten"))]
    loop {
        event_loop(&mut state);
    }
}

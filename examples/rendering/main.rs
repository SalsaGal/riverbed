use riverbed::{Game, WindowSize};
use riverbed::audio::{AudioHandler, Sink};
use riverbed::input::InputState;
use riverbed::layer::*;
use riverbed::math::{Color, Point, Rect};

fn main() {
	let game = Game::new("Window", WindowSize::Windowed(Point::new(640, 480)), Box::new(Display::new()))
		.render_frame_limit(60)
		.update_frame_limit(60);

	game.run();
}

struct Display {
	down: bool,
	mouse_pos: Point<u16>,
	sound: Option<Sink>,

	angle: f32,
}

impl Display {
	pub fn new() -> Self {
		Self {
			down: false,
			mouse_pos: Point::default(),
			sound: None,

			angle: 0.0,
		}
	}
}

impl Layer for Display {
	fn init(&mut self, data: &mut LayerData) {
		data.texture_cache.push(data.graphics.texture_from_file("examples/rendering/test.png").unwrap());
		data.texture_cache.push(data.graphics.texture_from_file("examples/rendering/spring.png").unwrap());
		self.sound = Some(data.audio.new_sink());
		self.sound.as_ref().unwrap().append(AudioHandler::load_wav("examples/rendering/test.wav"));
		self.sound.as_ref().unwrap().play();
	}

	fn update(&mut self, data: &mut LayerData) {
		self.down = data.input.key_is(49, InputState::Pressed);
		self.mouse_pos = data.input.mouse_pos;
		self.angle += std::f32::consts::TAU / 360.0;
	}

	fn render(&mut self, data: &mut LayerData) {
		let mut canvas = data.graphics.canvas();
		canvas.clear(Color::GREEN);
		canvas.draw_texture(Rect::new(256, 128, 64, 64), self.angle, data.texture_cache.get(0).unwrap());
		canvas.draw_texture(Rect::new(330, 128, 64, 24), self.angle, data.texture_cache.get(1).unwrap());
	}
}

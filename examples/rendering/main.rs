use riverbed::{Game, WindowSize};
use riverbed::input::{InputState, MouseButton};
use riverbed::math::{Color, Point, Rect};
use riverbed::layer::*;

fn main() {
	let game = Game::new("Window", WindowSize::Windowed(Point::new(640, 480)), Box::new(Display::new()))
		.render_frame_limit(60)
		.update_frame_limit(60);

	game.run();
}

struct Display {
	down: bool,
	mouse_pos: Point<u16>,
}

impl Display {
	pub fn new() -> Self {
		Self {
			down: false,
			mouse_pos: Point::origin(),
		}
	}
}

impl Layer for Display {
	fn init(&mut self, data: &mut LayerData) {
		data.texture_cache.push(data.graphics.texture_from_file("examples/rendering/test.png"));
	}

	fn update(&mut self, data: &mut LayerData) {
		self.down = data.input.key_is(49, InputState::Pressed);
		self.mouse_pos = data.input.mouse_pos;
		if data.input.button_is(MouseButton::Left, InputState::Down) {
			println!("Clicked");
		}
	}

	fn render(&mut self, data: &mut LayerData) {
		let mut canvas = data.graphics.canvas();
		canvas.fill_rect(Rect::new(self.mouse_pos.x as i32, self.mouse_pos.y as i32, 128, 192), Color::RED);
		if self.down {
			canvas.draw_texture(Rect::new(0, 0, 192, 64), data.texture_cache.get(0).unwrap());
		}
	}
}

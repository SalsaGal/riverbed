use riverbed::{Game, WindowSize};
use riverbed::graphics::{GraphicsHandler, TextureCache};
use riverbed::input::{InputHandler, InputState};
use riverbed::math::{Color, Point, Rect};
use riverbed::layer::Layer;

fn main() {
	let game = Game::new("Window", WindowSize::Windowed(Point::new(640, 480)), Box::new(Display::new()));
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
	fn init(&mut self, graphics: &mut GraphicsHandler, texture_cache: &mut TextureCache) {
		texture_cache.push(graphics.texture_from_file("examples/rendering/test.png"));
	}

	fn update(&mut self, input: &InputHandler) {
		self.down = input.key_is(49, InputState::Pressed);
		self.mouse_pos = input.mouse_pos;
	}

	fn render(&mut self, graphics: &mut GraphicsHandler, texture_cache: &mut TextureCache) {
		let mut canvas = graphics.canvas();
		canvas.fill_rect(Rect::new(self.mouse_pos.x as i32, self.mouse_pos.y as i32, 128, 192), Color::RED);
		if self.down {
			canvas.draw_texture(Rect::new(0, 0, 192, 64), texture_cache.get(0).unwrap());
		}
	}
}

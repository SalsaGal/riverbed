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
}

impl Display {
	pub fn new() -> Self {
		Self {
			down: false,
		}
	}
}

impl Layer for Display {
	fn update(&mut self, input: &InputHandler) {
		self.down = input.key_is(49, InputState::Pressed);
	}

	fn render(&mut self, graphics: &mut GraphicsHandler, texture_cache: &mut TextureCache) {
		let texture = texture_cache.push(graphics.texture_from_file("examples/rendering/test.png"));
		let mut canvas = graphics.canvas();
		canvas.fill_rect(Rect::new(64, 64, 128, 192), Color::RED);
		if self.down {
			canvas.draw_texture(Rect::new(0, 0, 192, 64), texture_cache.get(texture).unwrap());
		}
	}
}

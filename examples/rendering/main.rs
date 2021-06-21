use riverbed::{Game, WindowSize};
use riverbed::graphics::{GraphicsHandler, TextureCache};
use riverbed::math::{Color, Point, Rect};
use riverbed::layer::Layer;

fn main() {
	let game = Game::new("Window", WindowSize::Windowed(Point::new(640, 480)), Box::new(Display));
	game.run();
}

struct Display;

impl Layer for Display {
	fn render(&mut self, graphics: &mut GraphicsHandler, _: &mut TextureCache) {
		let texture = graphics.texture_from_file("examples/rendering/test.png");
		let mut canvas = graphics.canvas();
		canvas.fill_rect(Rect::new(64, 64, 128, 192), Color::RED);
		canvas.draw_texture(Rect::new(0, 0, 192, 64), &texture);
	}
}

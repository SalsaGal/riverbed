use riverbed::{Game, WindowSize};
use riverbed::graphics::Canvas;
use riverbed::math::{Color, Point, Rect};
use riverbed::layer::Layer;

fn main() {
	let game = Game::new("Window", WindowSize::Windowed(Point::new(640, 480)), Box::new(Display));
	game.run();
}

struct Display;

impl Layer for Display {
	fn render(&mut self, canvas: &mut Canvas) {
		canvas.fill_rect(Rect::new(64, 64, 128, 192), Color::RED);
	}
}

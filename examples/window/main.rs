use riverbed::{Game, WindowSize};
use riverbed::layer::EmptyLayer;

fn main() {
	let game = Game::new("Window", WindowSize::Fullscreen, Box::new(EmptyLayer));
	game.run();
}

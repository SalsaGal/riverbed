use riverbed::{Game, WindowSize};
use riverbed::scene::EmptyScene;

fn main() {
	let game = Game::new("Window", WindowSize::Fullscreen, Box::new(EmptyScene));
	game.run();
}

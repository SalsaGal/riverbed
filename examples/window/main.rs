use riverbed::{Game, WindowSize};

fn main() {
	let game = Game::new("Window", WindowSize::Fullscreen);
	game.run();
}

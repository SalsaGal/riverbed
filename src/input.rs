use std::collections::HashMap;

use winit::event::{ElementState, KeyboardInput, ScanCode};

pub struct InputHandler {
	keys: HashMap<ScanCode, InputState>,
}

impl InputHandler {
	pub fn new() -> Self {
		Self {
			keys: HashMap::new(),
		}
	}

	pub fn handle(&mut self, event: KeyboardInput) {
		self.keys.insert(event.scancode, match event.state {
			ElementState::Pressed => InputState::Pressed,
			ElementState::Released => InputState::Released,
		});
	}

	pub fn update(&mut self) {
		for (_, state) in self.keys.iter_mut() {
			match *state {
				InputState::Pressed => *state = InputState::Down,
				InputState::Released => *state = InputState::Up,
				_ => {},
			}
		}
	}

	pub fn key_is(&self, key: ScanCode, state: InputState) -> bool {
		*self.keys.get(&key).unwrap_or(&InputState::Up) == state
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum InputState {
	Up,
	Pressed,
	Down,
	Released,
}

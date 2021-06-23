use crate::math::Point;

use std::collections::HashMap;

use winit::event::{ElementState, KeyboardInput, ScanCode};
pub use winit::event::MouseButton;

pub struct InputHandler {
	keys: HashMap<ScanCode, InputState>,
	buttons: HashMap<MouseButton, InputState>,

	pub mouse_pos: Point<u16>,
}

impl InputHandler {
	pub fn new() -> Self {
		Self {
			keys: HashMap::new(),
			buttons: HashMap::new(),

			mouse_pos: Point::origin(),
		}
	}

	pub fn handle_key(&mut self, event: KeyboardInput) {
		self.keys.insert(event.scancode, match event.state {
			ElementState::Pressed => if !matches!(self.keys.get(&event.scancode), Some(InputState::Down)) {
				InputState::Pressed
			} else {
				InputState::Down
			},
			ElementState::Released => InputState::Released,
		});
	}

	pub fn handle_button(&mut self, button: MouseButton, state: ElementState) {
		self.buttons.insert(button, match state {
			ElementState::Pressed => if !matches!(self.buttons.get(&button), Some(InputState::Down)) {
				InputState::Pressed
			} else {
				InputState::Down
			},
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
		for (_, state) in self.buttons.iter_mut() {
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

	pub fn button_is(&self, button: MouseButton, state: InputState) -> bool {
		*self.buttons.get(&button).unwrap_or(&InputState::Up) == state
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum InputState {
	Up,
	Pressed,
	Down,
	Released,
}

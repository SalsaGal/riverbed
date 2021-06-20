pub mod math;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub struct Game {
	event_loop: EventLoop<()>,
	window: Window,
}

impl Game {
	pub fn new(title: &str) -> Self {
		let event_loop = EventLoop::new();
		let window = WindowBuilder::new().with_title(title);

		let window = window.build(&event_loop).unwrap();

		Self {
			event_loop,
			window,
		}
	}

	pub fn run(self) {
		self.event_loop.run(move |event, _, control_flow| {
			match event {
				Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
					*control_flow = ControlFlow::Exit;
				},
				_ => {},
			}
		});
	}
}

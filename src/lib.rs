pub mod math;

use crate::math::Point;

use winit::dpi::{LogicalSize, Size};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window, WindowBuilder};

pub struct Game {
	event_loop: EventLoop<()>,
	window: Window,
}

impl Game {
	pub fn new(title: &str, size: WindowSize) -> Self {
		let event_loop = EventLoop::new();
		let mut window = WindowBuilder::new().with_title(title).with_resizable(false);
		window = match size {
			WindowSize::Fullscreen => window.with_fullscreen(Some(Fullscreen::Borderless(None))),
			WindowSize::Windowed(bounds) => window.with_inner_size(Size::Logical(LogicalSize {width: bounds.x as f64, height: bounds.y as f64})),
		};

		let window = window.build(&event_loop).unwrap();

		Self {
			event_loop,
			window,
		}
	}

	pub fn run(self) {
		let window = self.window;
		self.event_loop.run(move |event, _, control_flow| {
			match event {
				Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
					*control_flow = ControlFlow::Exit;
				},
				Event::WindowEvent { event: WindowEvent::Resized(_), .. } => {
					window.set_fullscreen(Some(Fullscreen::Borderless(None)));
				},
				_ => {},
			}
		});
	}
}

pub enum WindowSize {
	Fullscreen,
	Windowed(Point<u16>),
}

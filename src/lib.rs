mod graphics;

pub mod math;
pub mod scene;

use crate::graphics::GraphicsHandler;
use crate::math::Point;
use crate::scene::Scene;

use winit::dpi::{LogicalSize, Size};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window, WindowBuilder};

pub struct Game {
	event_loop: EventLoop<()>,
	window: Window,

	scenes: Vec<Box<dyn Scene>>,
}

impl Game {
	pub fn new(title: &str, size: WindowSize, scene: Box<dyn Scene>) -> Self {
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

			scenes: vec![scene],
		}
	}

	pub fn run(self) {
		let window = self.window;
		let mut graphics = GraphicsHandler::new(&window);
		let mut scenes = self.scenes;

		self.event_loop.run(move |event, _, control_flow| {
			match event {
				Event::MainEventsCleared => {
					graphics.update();

					// Update scenes
					for scene in scenes.iter_mut() {
						scene.update();
					}

					// Render scenes
					for scene in scenes.iter_mut() {
						scene.render();
					}
				},
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

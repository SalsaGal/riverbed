pub mod input;
pub mod graphics;
pub mod math;
pub mod layer;

use crate::graphics::{GraphicsHandler, TextureCache};
use crate::input::InputHandler;
use crate::math::Point;
use crate::layer::Layer;

use winit::dpi::{LogicalSize, Size};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window, WindowBuilder};

pub struct Game {
	event_loop: EventLoop<()>,
	window: Window,

	scenes: Vec<Box<dyn Layer>>,
}

impl Game {
	pub fn new(title: &str, size: WindowSize, scene: Box<dyn Layer>) -> Self {
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
		let mut input = InputHandler::new();
		let mut texture_cache = TextureCache::new();
		let mut scenes = self.scenes;

		for scene in scenes.iter_mut() {
			scene.init(&mut graphics, &mut texture_cache);
		}

		self.event_loop.run(move |event, _, control_flow| {
			match event {
				Event::MainEventsCleared => {
					// Update scenes
					for scene in scenes.iter_mut() {
						scene.update(&input);
					}

					// Render scenes
					for scene in scenes.iter_mut() {
						scene.render(&mut graphics, &mut texture_cache);
					}

					// Update engine
					input.update();
				},
				Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
					*control_flow = ControlFlow::Exit;
				},
				Event::WindowEvent { event: WindowEvent::Resized(_), .. } => {
					window.set_fullscreen(Some(Fullscreen::Borderless(None)));
				},
				Event::WindowEvent { event: WindowEvent::KeyboardInput { input: event, .. }, .. } => {
					input.handle_key(event);
				},
				Event::WindowEvent { event: WindowEvent::CursorMoved { position, .. }, ..} => {
					input.mouse_pos = Point::new(position.x as u16, position.y as u16);
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

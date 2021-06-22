pub mod input;
pub mod graphics;
pub mod math;
pub mod layer;

use crate::graphics::{GraphicsHandler, TextureCache};
use crate::input::InputHandler;
use crate::math::Point;
use crate::layer::*;

use std::time::{Duration, Instant};

use winit::dpi::{LogicalSize, Size};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Fullscreen, Window, WindowBuilder};

pub struct Game {
	event_loop: EventLoop<()>,
	window: Window,

	scenes: Vec<Box<dyn Layer>>,

	render_frame_limit: Option<Duration>,
	update_frame_limit: Option<Duration>,

	next_render: Instant,
	next_update: Instant,
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

			render_frame_limit: None,
			update_frame_limit: None,

			next_render: Instant::now(),
			next_update: Instant::now(),
		}
	}

	pub fn render_frame_limit(mut self, framerate: u16) -> Self {
		self.render_frame_limit = Some(Duration::from_millis(1000 / framerate as u64));
		self
	}

	pub fn update_frame_limit(mut self, framerate: u16) -> Self {
		self.update_frame_limit = Some(Duration::from_millis(1000 / framerate as u64));
		self
	}

	pub fn run(self) {
		let window = self.window;
		let mut graphics = GraphicsHandler::new(&window);
		let mut input = InputHandler::new();
		let mut texture_cache = TextureCache::new();
		let mut scenes = self.scenes;
		let render_frame_limit = self.render_frame_limit;
		let update_frame_limit = self.update_frame_limit;
		let mut next_render = self.next_render;
		let mut next_update = self.next_update;

		for scene in scenes.iter_mut() {
			scene.init(&mut LayerData {
				graphics: &mut graphics,
				input: &input,
				texture_cache: &mut texture_cache,
			});
		}

		self.event_loop.run(move |event, _, control_flow| {
			match event {
				Event::MainEventsCleared => {
					let mut layer_data = LayerData {
						graphics: &mut graphics,
						input: &input,
						texture_cache: &mut texture_cache,
					};
					
					if matches!(Instant::now().checked_duration_since(next_update), Some(_)) {
						// Update scenes
						for scene in scenes.iter_mut() {
							scene.update(&mut layer_data);
						}
						next_update += update_frame_limit.unwrap_or_default();
					}

					if matches!(Instant::now().checked_duration_since(next_render), Some(_)) {
						// Render scenes
						for scene in scenes.iter_mut() {
							scene.render(&mut layer_data);
						}
						next_render += render_frame_limit.unwrap_or_default();
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

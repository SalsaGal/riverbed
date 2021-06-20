use polystrip::{Renderer, RenderSize, WindowTarget};
use polystrip::gon::GonPipeline;
use polystrip::math::Color;

use winit::window::Window;

pub struct GraphicsHandler {
	renderer: WindowTarget,
	pipeline: GonPipeline,
}

impl GraphicsHandler {
	pub fn new(window: &Window) -> Self {
		let window_size = RenderSize::new(window.inner_size().width, window.inner_size().height).wrap();
		let renderer = WindowTarget::new(Renderer::new().wrap(), window, &window_size, 3);
		let pipeline = GonPipeline::new(&renderer, &renderer);

		Self {
			renderer,
			pipeline,
		}
	}

	pub fn update(&mut self) {
		let mut frame = self.renderer.next_frame().render_with(&mut self.pipeline);
	}
}

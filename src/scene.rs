pub trait Scene {
	fn update(&mut self) {}
	fn render(&mut self) {}
}

pub struct EmptyScene;
impl Scene for EmptyScene {}
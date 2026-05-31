#[derive(Clone)]
pub struct Context {
    pub viewing_distance: f32,
    pub width: i32,
    pub height: i32,
}

impl Default for Context {
    fn default() -> Context {
        Context {
            viewing_distance: 25.0,
            width: 640,
            height: 480,
        }
    }
}

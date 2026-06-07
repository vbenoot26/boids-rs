#[derive(Clone)]
pub struct Context {
    pub viewing_distance: f32,
    pub close_distance: f32,
    pub width: i32,
    pub height: i32,

    pub avoid_factor: f32,
    pub centering_factor: f32,
    pub matching_factor: f32,

    pub boid_amount: usize,

    pub min_speed: f32,
    pub max_speed: f32,
}

impl Default for Context {
    fn default() -> Context {
        Context {
            viewing_distance: 20.0,
            close_distance: 5.0,
            width: 1000,
            height: 1000,
            avoid_factor: 0.05,
            centering_factor: 0.0005,
            matching_factor: 0.05,
            boid_amount: 8000,
            min_speed: 5.0,
            max_speed: 10.0,
        }
    }
}

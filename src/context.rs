pub struct Context {
    pub viewing_distance: i32,
    pub width: i32,
    pub height: i32,
}

pub fn new() -> Context {
    return Context {
        viewing_distance: 25,
        width: 640,
        height: 480,
    };
}

pub struct Node {
    pub x: i32,
}

pub enum FooBar {
    struct Node,
    struct Node2,
}

struct Node2 {
    y: u32,
    j: i64,
}

trait FooZap {
    fn is_dead(x: u32) -> i32;
}

#[derive(Debug)]
enum Key {
    Up,
    Right,
    Down,
    Left,
    Enter,
}

#[derive(Debug)]
enum KeyState {
    Pressed(bool), // Auto-repeat flag
    Released,
}

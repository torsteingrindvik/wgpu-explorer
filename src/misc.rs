#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn displace(direction: Direction, amount: f32) -> (f32, f32) {
    assert!(amount > 0.0);

    match direction {
        Direction::Up => (0.0, amount),
        Direction::Down => (0.0, -amount),
        Direction::Left => (-amount, 0.0),
        Direction::Right => (amount, 0.0),
    }
}

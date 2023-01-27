use std::ops;
use std::cmp::PartialEq;
#[macro_use] extern crate impl_ops;

pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

impl Direction{
    fn delta(&self) -> Position{
        match self{
            Self::Up    => Position {x: 0, y: 1},
            Self::Down  => Position {x: 0, y:-1},
            Self::Left  => Position {x:-1, y: 0},
            Self::Right => Position {x: 1, y: 0}
        }
    }
}

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Position{
    pub x: i8,
    pub y: i8
}

impl Position{
    pub fn new() -> Self{
        Self{
            x: 0,
            y: 0
        }
    }
}

impl_op_ex!(+|a: &Position, b: &Position| -> Position {
    Position {x: a.x + b.x, y: a.y + b.y}
});

impl_op_ex!(-|a: &Position, b: &Position| -> Position {
    Position {x: a.x - b.x, y: a.y - b.y}
});

pub struct Snake{
    pub head: Position,
    pub tail: Position
}

impl Snake{
    /// Make a new snake at (0,0),(0,0)
    /// # Examples
    /// ```
    /// 
    /// use advent_of_code_2022_09::Snake;
    /// use advent_of_code_2022_09::Position;
    /// let snake = Snake::new();
    ///
    /// assert_eq!(snake.head, Position{x:0, y:0});
    /// assert_eq!(snake.tail, Position{x:0, y:0});
    /// ```
    pub fn new() -> Self{
        Self{
            head: Position::new(),
            tail: Position::new()
        }
    }

    /// Move the snake head, and the tail if necessary
    /// # Examples
    /// ```
    /// use advent_of_code_2022_09::Snake;
    /// use advent_of_code_2022_09::Direction;
    /// use advent_of_code_2022_09::Position;
    /// let mut snake = Snake::new();
    ///
    /// snake.step(Direction::Up);
    /// assert_eq!(snake.head, Position{x:0, y:1});
    /// assert_eq!(snake.tail, Position{x:0, y:0});
    ///
    /// snake.step(Direction::Right);
    /// assert_eq!(snake.head, Position{x:1, y:1});
    /// assert_eq!(snake.tail, Position{x:0, y:0});
    ///
    /// snake.step(Direction::Right);
    /// assert_eq!(snake.head, Position{x:2, y:1});
    /// assert_eq!(snake.tail, Position{x:1, y:1});
    ///
    /// snake.step(Direction::Right);
    /// assert_eq!(snake.head, Position{x:3, y:1});
    /// assert_eq!(snake.tail, Position{x:2, y:1});
    ///
    /// snake.step(Direction::Down);
    /// assert_eq!(snake.head, Position{x:3, y:0});
    /// assert_eq!(snake.tail, Position{x:2, y:1});
    ///
    /// snake.step(Direction::Left);
    /// assert_eq!(snake.head, Position{x:2, y:0});
    /// assert_eq!(snake.tail, Position{x:2, y:1});
    /// ```
    pub fn step(&mut self, dir: Direction){
        self.head = &self.head + dir.delta();
        let trail = &self.head - &self.tail;
        if trail.x.abs() > 1 || trail.y.abs() > 1{
            self.tail = self.head - dir.delta();
        }
    }
}

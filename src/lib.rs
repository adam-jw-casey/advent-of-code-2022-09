use std::ops;
use std::cmp::PartialEq;
#[macro_use] extern crate impl_ops;
use sscanf::sscanf;
use std::collections::HashSet;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position{
    pub x: i32,
    pub y: i32
}

impl Position{
    pub fn new() -> Self{
        Self{
            x: 0,
            y: 0
        }
    }

    pub fn unit(&self) -> Self{
        Position{
            x: self.x.signum(),
            y: self.y.signum()
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
    body: Vec<Position>
}

impl Snake{
    /// Make a new snake at (0,0),(0,0)
    /// # Examples
    /// ```
    /// 
    /// use advent_of_code_2022_09::Snake;
    /// use advent_of_code_2022_09::Position;
    /// let snake = Snake::new(2);
    ///
    /// assert_eq!(*snake.head(), Position{x:0, y:0});
    /// assert_eq!(*snake.tail(), Position{x:0, y:0});
    /// ```
    pub fn new(length: u8) -> Self{
        assert!(length > 0);
        Self{
            body: (0..length).map(|_| Position::new()).collect()
        }
    }

    pub fn head(&self) -> &Position{
        &self.body.first().unwrap()
    }

    pub fn tail(&self) -> &Position{
        &self.body.last().unwrap()
    }

    /// Move the snake head, and the tail if necessary
    /// # Examples
    /// ```
    /// use advent_of_code_2022_09::Snake;
    /// use advent_of_code_2022_09::Direction;
    /// use advent_of_code_2022_09::Position;
    /// let mut snake = Snake::new(2);
    ///
    /// snake.step(&Direction::Up);
    /// assert_eq!(*snake.head(), Position{x:0, y:1});
    /// assert_eq!(*snake.tail(), Position{x:0, y:0});
    ///
    /// snake.step(&Direction::Right);
    /// assert_eq!(*snake.head(), Position{x:1, y:1});
    /// assert_eq!(*snake.tail(), Position{x:0, y:0});
    ///
    /// snake.step(&Direction::Right);
    /// assert_eq!(*snake.head(), Position{x:2, y:1});
    /// assert_eq!(*snake.tail(), Position{x:1, y:1});
    ///
    /// snake.step(&Direction::Right);
    /// assert_eq!(*snake.head(), Position{x:3, y:1});
    /// assert_eq!(*snake.tail(), Position{x:2, y:1});
    ///
    /// snake.step(&Direction::Down);
    /// assert_eq!(*snake.head(), Position{x:3, y:0});
    /// assert_eq!(*snake.tail(), Position{x:2, y:1});
    ///
    /// snake.step(&Direction::Left);
    /// assert_eq!(*snake.head(), Position{x:2, y:0});
    /// assert_eq!(*snake.tail(), Position{x:2, y:1});
    /// ```
    pub fn step(&mut self, dir: &Direction){
        self.body[0] = self.body[0] + dir.delta();

        for i in 1..self.body.len(){
            let trail = &self.body[i-1] - &self.body[i];
            if trail.x.abs() > 1 || trail.y.abs() > 1{
                self.body[i] = self.body[i] + trail.unit();
            }
        }
    }
}

/// Counts the number of unique positions the snake's tail occupies
/// while following the commands in input_moves
/// # Examples
/// ```
/// use advent_of_code_2022_09::count_tail_positions;
/// assert_eq!(
///     13,
///     count_tail_positions(concat!(
///             "R 4\n",
///             "U 4\n",
///             "L 3\n",
///             "D 1\n",
///             "R 4\n",
///             "D 1\n",
///             "L 5\n",
///             "R 2"
///         ),
///         2
///     )
/// );
///
/// assert_eq!(
///     1,
///     count_tail_positions(concat!(
///             "R 4\n",
///             "U 4\n",
///             "L 3\n",
///             "D 1\n",
///             "R 4\n",
///             "D 1\n",
///             "L 5\n",
///             "R 2"
///         ),
///         10
///     )
/// );
///
/// assert_eq!(
///     36,
///     count_tail_positions(concat!(
///             "R 5\n",
///             "U 8\n",
///             "L 8\n",
///             "D 3\n",
///             "R 17\n",
///             "D 10\n",
///             "L 25\n",
///             "U 20"
///         ),
///         10
///     )
/// );
/// ```
pub fn count_tail_positions(input_moves: &str, snake_size: u8) -> usize{
    let mut snake = Snake::new(snake_size);
    let mut tail_positions = HashSet::new();
    let mut dir: Direction;
    let mut n: u8;
    let mut parsed: (char, u8);

    tail_positions.insert(*snake.tail());
    for line in input_moves.lines(){
        parsed = sscanf!(line, "{char} {u8}")
            .expect("Input should be properly formatted");

        dir = match parsed.0{
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _   => panic!("Invalid input!")
        };
        n = parsed.1;

        for _i in 0..n{
            snake.step(&dir);
            tail_positions.insert(*snake.tail());
        }
    }

    tail_positions.len()
}

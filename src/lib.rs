//! Rust Api for mms (micromouse simulator)

use std::{
    io::{BufRead, StdinLock, StdoutLock, Write},
    num::{NonZeroU32, ParseFloatError, ParseIntError},
    str::FromStr,
};

#[derive(thiserror::Error, Debug)]
pub enum MmsError {
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("ParseFloatError: {0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[error("InvalidAck: {0}")]
    InvalidAck(String),
    #[error("InvalidColorString: {0}")]
    InvalidColorString(String),
    #[error("InvalidDirectionString: {0}")]
    InvalidDirectionString(String),
}

/// Which stat to query
pub enum StatQuery {
    TotalDistance,
    TotalTurns,
    BestRunDistance,
    BestRunTurns,
    CurrentRunDistance,
    CurrentRunTurns,
    TotalEffectiveDistance,
    BestRunEffectiveDistance,
    CurrentRunEffectiveDistance,
    Score,
}

impl StatQuery {
    fn get_string(&self) -> &'static str {
        match self {
            StatQuery::TotalDistance => "total-distance",
            StatQuery::TotalTurns => "total-turns",
            StatQuery::BestRunDistance => "best-run-distance",
            StatQuery::BestRunTurns => "best-run-turns",
            StatQuery::CurrentRunDistance => "current-run-distance",
            StatQuery::CurrentRunTurns => "current-run-turns",
            StatQuery::TotalEffectiveDistance => "total-effective-distance",
            StatQuery::BestRunEffectiveDistance => "best-run-effective-distance",
            StatQuery::CurrentRunEffectiveDistance => "current-run-effective-distance",
            StatQuery::Score => "score",
        }
    }
}

/// The stat that was requested
pub enum Stat {
    TotalDistance(i32),
    TotalTurns(i32),
    BestRunDistance(i32),
    BestRunTurns(i32),
    CurrentRunDistance(i32),
    CurrentRunTurns(i32),
    TotalEffectiveDistance(f32),
    BestRunEffectiveDistance(f32),
    CurrentRunEffectiveDistance(f32),
    Score(f32),
}

/// The direction for the wall
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl FromStr for Direction {
    type Err = MmsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::{East, North, South, West};
        match s {
            "n" => Ok(North),
            "e" => Ok(East),
            "s" => Ok(South),
            "w" => Ok(West),
            _ => Err(MmsError::InvalidDirectionString(s.to_string())),
        }
    }
}

impl Direction {
    fn get_string(&self) -> char {
        use Direction::{East, North, South, West};
        match self {
            North => 'n',
            East => 'e',
            South => 's',
            West => 'w',
        }
    }
}

/// The cell color
pub enum CellColor {
    Black,
    Blue,
    Gray,
    Cyan,
    Green,
    Orange,
    Red,
    White,
    Yellow,
    DarkBlue,
    DarkCyan,
    DarkGray,
    DarkGreen,
    DarkRed,
    DarkYellow,
}

impl FromStr for CellColor {
    type Err = MmsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CellColor::{
            Black, Blue, Cyan, DarkBlue, DarkCyan, DarkGray, DarkGreen, DarkRed, DarkYellow, Gray,
            Green, Orange, Red, White, Yellow,
        };
        match s {
            "k" => Ok(Black),
            "b" => Ok(Blue),
            "a" => Ok(Gray),
            "c" => Ok(Cyan),
            "g" => Ok(Green),
            "o" => Ok(Orange),
            "r" => Ok(Red),
            "w" => Ok(White),
            "y" => Ok(Yellow),
            "B" => Ok(DarkBlue),
            "C" => Ok(DarkCyan),
            "A" => Ok(DarkGray),
            "G" => Ok(DarkGreen),
            "R" => Ok(DarkRed),
            "Y" => Ok(DarkYellow),
            _ => Err(MmsError::InvalidColorString(s.to_string())),
        }
    }
}

impl CellColor {
    fn get_char(&self) -> char {
        use CellColor::{
            Black, Blue, Cyan, DarkBlue, DarkCyan, DarkGray, DarkGreen, DarkRed, DarkYellow, Gray,
            Green, Orange, Red, White, Yellow,
        };
        match self {
            Black => 'k',
            Blue => 'b',
            Gray => 'a',
            Cyan => 'c',
            Green => 'g',
            Orange => 'o',
            Red => 'r',
            White => 'w',
            Yellow => 'y',
            DarkBlue => 'B',
            DarkCyan => 'C',
            DarkGray => 'A',
            DarkGreen => 'G',
            DarkRed => 'R',
            DarkYellow => 'Y',
        }
    }
}

/// The main wrapper around the mms api. Holds locks to `stdin` and `stdout` to allow for fast and
/// exclusive access for the api.
pub struct MmsApi<'a> {
    cout: StdoutLock<'a>,
    cin: StdinLock<'a>,
}

#[cfg(feature = "use_results")]
type ResultType<T> = Result<T, MmsError>;

#[cfg(not(feature = "use_results"))]
type ResultType<T> = T;

#[cfg(feature = "use_results")]
macro_rules! writeln_and_flush {
    ($dst:expr, $($arg:tt)*) => {
        writeln!($dst, $($arg)*)?;
        $dst.flush()?;
    };
}

#[cfg(not(feature = "use_results"))]
macro_rules! writeln_and_flush {
    ($dst:expr, $($arg:tt)*) => {
        writeln!($dst, $($arg)*).unwrap();
        $dst.flush().unwrap();
    };
}

#[cfg(feature = "use_results")]
macro_rules! handle_result {
    ($e: expr) => {
        $e?
    };
}
#[cfg(not(feature = "use_results"))]
macro_rules! handle_result {
    ($e: expr) => {
        $e.unwrap()
    };
}

#[cfg(feature = "use_results")]
macro_rules! return_result {
    ($e: expr) => {
        return Ok($e);
    };
}
#[cfg(not(feature = "use_results"))]
macro_rules! return_result {
    (()) => {
        ()
    };
    ($e: expr) => {
        return $e;
    };
}

macro_rules! ack {
    ($this: expr) => {
        return $this.read_ack();
    };
}

impl<'a> MmsApi<'a> {
    /// Create a new api for mms
    #[must_use]
    pub fn new(cin: StdinLock<'a>, cout: StdoutLock<'a>) -> Self {
        Self { cout, cin }
    }

    /// Returns the width of the maze
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn maze_width(&mut self) -> ResultType<i32> {
        writeln_and_flush!(self.cout, "mazeWidth");
        let mut response = String::new();
        handle_result!(self.cin.read_line(&mut response));
        return_result!(handle_result!(response.trim().parse()));
    }

    /// Returns the height of the maze
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn maze_height(&mut self) -> ResultType<i32> {
        writeln_and_flush!(self.cout, "mazeHeight");
        let mut response = String::new();
        handle_result!(self.cin.read_line(&mut response));
        return_result!(handle_result!(response.trim().parse()));
    }

    /// Returns `true` if there is a wall in front of the robot, else `false`
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn wall_front(&mut self) -> ResultType<bool> {
        writeln_and_flush!(self.cout, "wallFront");
        let mut response = String::new();
        handle_result!(self.cin.read_line(&mut response));
        return_result!(response.trim() == "true");
    }

    /// Returns `true` if there is a wall to the right of the robot, else `false`
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn wall_right(&mut self) -> ResultType<bool> {
        writeln_and_flush!(self.cout, "wallRight");
        let mut response = String::new();
        handle_result!(self.cin.read_line(&mut response));
        return_result!(response.trim() == "true");
    }

    /// Returns `true` if there is a wall to the left of the robot, else `false`
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn wall_left(&mut self) -> ResultType<bool> {
        writeln_and_flush!(self.cout, "wallLeft");
        let mut response = String::new();
        handle_result!(self.cin.read_line(&mut response));
        return_result!(response.trim() == "true");
    }

    /// Move the robot forward the specified number of cells
    ///
    /// Args:
    /// - `distance`: The optional non-zero number of cells to move forward. Default = 1
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// `InvalidAck`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn move_forward(&mut self, distance: Option<NonZeroU32>) -> ResultType<()> {
        writeln_and_flush!(
            self.cout,
            "moveForward {}",
            distance.map_or_else(String::new, |d| d.to_string())
        );
        ack!(self);
    }

    /// Turn the robot ninety degrees to the right
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// `InvalidAck`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn turn_right(&mut self) -> ResultType<()> {
        writeln_and_flush!(self.cout, "turnRight");
        ack!(self);
    }

    /// Turn the robot ninety degrees to the left
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// `InvalidAck`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn turn_left(&mut self) -> ResultType<()> {
        writeln_and_flush!(self.cout, "turnLeft");
        ack!(self);
    }

    /// Display a wall at the given position
    ///
    /// Args:
    /// - `x`: The X coordinate of the cell
    /// - `y`: The Y coordinate of the cell
    /// - `direction`: The direction of the wall
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn set_wall(&mut self, x: u32, y: u32, direction: &Direction) -> ResultType<()> {
        writeln_and_flush!(self.cout, "setWall {x} {y} {}", direction.get_string());
        return_result!(());
    }

    /// Clear the wall at the given position
    ///
    /// Args:
    /// - `x`: The X coordinate of the cell
    /// - `y`: The Y coordinate of the cell
    /// - `direction`: The direction of the wall
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn clear_wall(&mut self, x: u32, y: u32, direction: &Direction) -> ResultType<()> {
        writeln_and_flush!(self.cout, "clearWall {x} {y} {}", direction.get_string());
        return_result!(());
    }

    /// Set the color of the cell at the given position
    ///
    /// Args:
    /// - `x`: The X coordinate of the cell
    /// - `y`: The Y coordinate of the cell
    /// - `color`: The color of the cell
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn set_color(&mut self, x: u32, y: u32, color: &CellColor) -> ResultType<()> {
        writeln_and_flush!(self.cout, "setColor {x} {y} {}", color.get_char());
        return_result!(());
    }

    /// Clear the color of the cell at the given position
    ///
    /// Args:
    /// - `x`: The X coordinate of the cell
    /// - `y`: The Y coordinate of the cell
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn clear_color(&mut self, x: u32, y: u32) -> ResultType<()> {
        writeln_and_flush!(self.cout, "clearColor {x} {y}");
        return_result!(());
    }

    /// Clear the color of all cells
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn clear_all_collor(&mut self) -> ResultType<()> {
        writeln_and_flush!(self.cout, "clearAllColor");
        return_result!(());
    }

    /// Set the text of the cell at the given position
    ///
    /// Args:
    /// - `x`: The X coordinate of the cell
    /// - `y`: The Y coordinate of the cell
    /// - `text`: The desired text, max length 10
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn set_text(&mut self, x: u32, y: u32, text: &str) -> ResultType<()> {
        writeln_and_flush!(self.cout, "setText {x} {y} {text}");
        return_result!(());
    }

    /// Clear the text of the cell at the given position
    ///
    /// Args:
    /// - `x`: The X coordinate of the cell
    /// - `y`: The Y coordinate of the cell
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn clear_text(&mut self, x: u32, y: u32) -> ResultType<()> {
        writeln_and_flush!(self.cout, "clearText {x} {y}");
        return_result!(());
    }

    /// Clear the text of all cells
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn clear_all_text(&mut self) -> ResultType<()> {
        writeln_and_flush!(self.cout, "clearAllText");
        return_result!(());
    }

    /// Returns `true` if the reset button was pressed, else `false`
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn was_reset(&mut self) -> ResultType<bool> {
        writeln_and_flush!(self.cout, "wasReset");
        let mut response = String::new();
        handle_result!(self.cin.read_line(&mut response));
        return_result!(response.trim() == "true");
    }

    /// Allow the mouse to be moved back to the start of the maze
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// `InvalidAck`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn ack_reset(&mut self) -> ResultType<()> {
        writeln_and_flush!(self.cout, "ackReset");
        ack!(self);
    }

    /// The value of the stat, or `-1` if no value exists yet.
    ///
    /// # Errors
    /// `IoError`
    /// `ParseIntError`
    /// `ParseFloatError`
    /// # Panics
    /// this panics when `use_results` is disabled
    pub fn get_stat(&mut self, query: &StatQuery) -> ResultType<Stat> {
        writeln_and_flush!(self.cout, "{}", query.get_string());
        let mut response = String::new();
        handle_result!(self.cin.read_line(&mut response));
        let response = response.trim();
        let result = match query {
            StatQuery::TotalDistance => Stat::TotalDistance(handle_result!(response.parse())),
            StatQuery::TotalTurns => Stat::TotalTurns(handle_result!(response.parse())),
            StatQuery::BestRunDistance => Stat::BestRunDistance(handle_result!(response.parse())),
            StatQuery::BestRunTurns => Stat::BestRunTurns(handle_result!(response.parse())),
            StatQuery::CurrentRunDistance => {
                Stat::CurrentRunDistance(handle_result!(response.parse()))
            }
            StatQuery::CurrentRunTurns => Stat::CurrentRunTurns(handle_result!(response.parse())),
            StatQuery::TotalEffectiveDistance => {
                Stat::TotalEffectiveDistance(handle_result!(response.parse()))
            }
            StatQuery::BestRunEffectiveDistance => {
                Stat::BestRunEffectiveDistance(handle_result!(response.parse()))
            }
            StatQuery::CurrentRunEffectiveDistance => {
                Stat::CurrentRunEffectiveDistance(handle_result!(response.parse()))
            }
            StatQuery::Score => Stat::Score(handle_result!(response.parse())),
        };
        return_result!(result);
    }

    fn read_ack(&mut self) -> ResultType<()> {
        let mut response = String::new();
        handle_result!(self.cin.read_line(&mut response));
        let ack = response.trim();
        #[cfg(feature = "use_results")]
        if ack == "ack" {
            Ok(())
        } else {
            Err(MmsError::InvalidAck(response))
        }
        #[cfg(not(feature = "use_results"))]
        assert!(ack == "ack", "{response}");
    }
}

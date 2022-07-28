use std::fmt;
use std::hash::Hash;
use std::iter;
use std::str;
use std::usize;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Player {
    Nought,
    Cross,
}

impl Player {
    pub fn toggle(self) -> Player {
        match self {
            Player::Nought => Player::Cross,
            Player::Cross => Player::Nought,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::Nought => write!(f, "O"),
            Player::Cross => write!(f, "X"),
        }
    }
}

impl str::FromStr for Player {
    type Err = ParsePlayerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "O" => Ok(Player::Nought),
            "X" => Ok(Player::Cross),
            _ => Err(ParsePlayerError {}),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParsePlayerError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
    Occupied(Player),
    Vacant,
}

impl Cell {
    fn is_occupied(self) -> bool {
        !self.is_vacant()
    }

    fn is_vacant(self) -> bool {
        match self {
            Cell::Occupied(_) => false,
            Cell::Vacant => true,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Occupied(player) => write!(f, "{}", player),
            Cell::Vacant => write!(f, " "),
        }
    }
}

// a position on the board
// 1 2 3
// 4 5 6
// 7 8 9
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pos {
    pos: usize,
}

impl Pos {
    pub fn new(pos: usize) -> Option<Pos> {
        if (1..=Board::SIZE).contains(&pos) {
            Some(Pos { pos })
        } else {
            None
        }
    }
    pub fn get(self) -> usize {
        self.pos
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

pub struct Board {
    // row-major layer
    cells: [Cell; Board::SIZE],
}

impl Board {
    pub const WIDTH: usize = 3;
    pub const SIZE: usize = Board::WIDTH * Board::WIDTH;

    pub fn new() -> Board {
        Board {
            cells: [Cell::Vacant; Board::SIZE],
        }
    }

    pub fn place(&mut self, pos: Pos, player: Player) -> Result<(), PlaceError> {
        let cell = &mut self.cells[pos.get() - 1];
        match *cell {
            Cell::Occupied(player) => Err(PlaceError {
                pos,
                occupied_by: player,
            }),
            Cell::Vacant => {
                *cell = Cell::Occupied(player);
                Ok(())
            }
        }
    }

    pub fn wins(&self, player: Player) -> bool {
        self.rows().any(|row| occupied_by(row, player))
            || self.columns().any(|column| occupied_by(column, player))
            || self
                .diagonals()
                .any(|diagonal| occupied_by(diagonal, player))
    }

    pub fn is_draw(&self) -> bool {
        self.is_complete() && !self.wins(Player::Nought) && !self.wins(Player::Cross)
    }

    fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &Cell>> {
        self.cells.chunks(Board::WIDTH).map(|chunk| chunk.iter())
    }

    fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &Cell>> {
        (0..Board::WIDTH).map(move |n| self.cells.iter().skip(n).step_by(Board::WIDTH))
    }

    fn diagonals(&self) -> impl Iterator<Item = impl Iterator<Item = &Cell>> {
        // major and minor have the same type
        let major = iter::once(
            self.cells
                .iter()
                .skip(0)
                .step_by(Board::WIDTH + 1)
                .take(Board::WIDTH),
        );
        let minor = iter::once(
            self.cells
                .iter()
                .skip(Board::WIDTH - 1)
                .step_by(Board::WIDTH - 1)
                .take(Board::WIDTH),
        );
        major.chain(minor)
    }

    fn is_complete(&self) -> bool {
        self.cells.iter().all(|cell| cell.is_occupied())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "+{}+", ["---"; Board::WIDTH].join("+"))?;

        for row in self.rows() {
            writeln!(f, "| {} |", row.format(" | "))?;
            writeln!(f, "+{}+", ["---"; Board::WIDTH].join("+"))?;
        }

        Ok(())
    }
}

fn occupied_by<'a, I: Iterator<Item = &'a Cell>>(mut cells: I, player: Player) -> bool {
    cells.all(|cell| *cell == Cell::Occupied(player))
}

#[derive(Debug, Eq, PartialEq)]
pub struct PlaceError {
    pub pos: Pos,
    pub occupied_by: Player,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_toggle() {
        assert_eq!(Player::Nought, Player::Cross.toggle());
        assert_eq!(Player::Cross, Player::Nought.toggle());
    }

    #[test]
    fn player_display() {
        assert_eq!("O", format!("{}", Player::Nought));
        assert_eq!("X", format!("{}", Player::Cross));
    }

    #[test]
    fn player_parse() {
        assert_eq!(Ok(Player::Nought), "O".parse());
        assert_eq!(Ok(Player::Cross), "X".parse());

        assert!("".parse::<Player>().is_err());
        assert!("a".parse::<Player>().is_err());
        assert!("o".parse::<Player>().is_err());
        assert!("XXX".parse::<Player>().is_err());
    }

    #[test]
    fn cell() {
        assert!(Cell::Occupied(Player::Nought).is_occupied());
        assert!(Cell::Occupied(Player::Cross).is_occupied());
        assert!(!Cell::Vacant.is_occupied());

        assert!(!Cell::Occupied(Player::Nought).is_vacant());
        assert!(!Cell::Occupied(Player::Cross).is_vacant());
        assert!(Cell::Vacant.is_vacant());
    }

    #[test]
    fn cell_display() {
        assert_eq!("O", format!("{}", Cell::Occupied(Player::Nought)));
        assert_eq!("X", format!("{}", Cell::Occupied(Player::Cross)));
        assert_eq!(" ", format!("{}", Cell::Vacant));
    }

    #[test]
    fn pos() {
        assert_eq!(1, Pos::new(1).unwrap().get());
        assert_eq!(4, Pos::new(4).unwrap().get());
        assert_eq!(9, Pos::new(9).unwrap().get());

        assert!(Pos::new(0).is_none());
        assert!(Pos::new(10).is_none());
        assert!(Pos::new(usize::MAX).is_none());
    }

    #[test]
    fn board_new() {
        let board = Board::new();
        assert_eq!([Cell::Vacant; 9], board.cells);
    }

    #[test]
    fn board_place() {
        let mut board = Board::new();

        board.place(Pos::new(1).unwrap(), Player::Nought).unwrap();
        assert_eq!(
            [
                Cell::Occupied(Player::Nought),
                Cell::Vacant,
                Cell::Vacant,
                Cell::Vacant,
                Cell::Vacant,
                Cell::Vacant,
                Cell::Vacant,
                Cell::Vacant,
                Cell::Vacant,
            ],
            board.cells
        );
        board.place(Pos::new(5).unwrap(), Player::Cross).unwrap();
        board.place(Pos::new(9).unwrap(), Player::Nought).unwrap();
        assert_eq!(
            [
                Cell::Occupied(Player::Nought),
                Cell::Vacant,
                Cell::Vacant,
                Cell::Vacant,
                Cell::Occupied(Player::Cross),
                Cell::Vacant,
                Cell::Vacant,
                Cell::Vacant,
                Cell::Occupied(Player::Nought),
            ],
            board.cells
        );

        assert_eq!(
            PlaceError {
                pos: Pos::new(1).unwrap(),
                occupied_by: Player::Nought,
            },
            board
                .place(Pos::new(1).unwrap(), Player::Cross)
                .unwrap_err()
        );
    }

    #[test]
    fn board_display() {
        assert_eq!(
            "\
            +---+---+---+\n\
            |   |   |   |\n\
            +---+---+---+\n\
            |   |   |   |\n\
            +---+---+---+\n\
            |   |   |   |\n\
            +---+---+---+\n\
            ",
            format!("{}", Board::new()),
        );
    }

    #[test]
    fn board_rows() {
        let board = Board {
            cells: [
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
                Cell::Vacant,
                Cell::Occupied(Player::Cross),
                Cell::Vacant,
                Cell::Occupied(Player::Nought),
                Cell::Vacant,
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
            ],
        };

        let mut rows = board.rows();

        let mut row = rows.next().unwrap();
        assert_eq!(Cell::Occupied(Player::Nought), *row.next().unwrap());
        assert_eq!(Cell::Occupied(Player::Cross), *row.next().unwrap());
        assert_eq!(Cell::Vacant, *row.next().unwrap());
        assert!(row.next().is_none());

        let mut row = rows.next().unwrap();
        assert_eq!(Cell::Occupied(Player::Cross), *row.next().unwrap());
        assert_eq!(Cell::Vacant, *row.next().unwrap());
        assert_eq!(Cell::Occupied(Player::Nought), *row.next().unwrap());
        assert!(row.next().is_none());

        let mut row = rows.next().unwrap();
        assert_eq!(Cell::Vacant, *row.next().unwrap());
        assert_eq!(Cell::Occupied(Player::Nought), *row.next().unwrap());
        assert_eq!(Cell::Occupied(Player::Cross), *row.next().unwrap());
        assert!(row.next().is_none());

        assert!(rows.next().is_none());
    }

    #[test]
    fn board_columns() {
        let board = Board {
            cells: [
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
                Cell::Vacant,
                Cell::Occupied(Player::Cross),
                Cell::Vacant,
                Cell::Occupied(Player::Nought),
                Cell::Vacant,
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
            ],
        };

        let mut columns = board.columns();

        let mut column = columns.next().unwrap();
        assert_eq!(Cell::Occupied(Player::Nought), *column.next().unwrap());
        assert_eq!(Cell::Occupied(Player::Cross), *column.next().unwrap());
        assert_eq!(Cell::Vacant, *column.next().unwrap());
        assert!(column.next().is_none());

        let mut column = columns.next().unwrap();
        assert_eq!(Cell::Occupied(Player::Cross), *column.next().unwrap());
        assert_eq!(Cell::Vacant, *column.next().unwrap());
        assert_eq!(Cell::Occupied(Player::Nought), *column.next().unwrap());
        assert!(column.next().is_none());

        let mut column = columns.next().unwrap();
        assert_eq!(Cell::Vacant, *column.next().unwrap());
        assert_eq!(Cell::Occupied(Player::Nought), *column.next().unwrap());
        assert_eq!(Cell::Occupied(Player::Cross), *column.next().unwrap());
        assert!(column.next().is_none());

        assert!(columns.next().is_none());
    }

    #[test]
    fn board_diagonals() {
        let board = Board {
            cells: [
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
                Cell::Vacant,
                Cell::Occupied(Player::Cross),
                Cell::Vacant,
                Cell::Occupied(Player::Nought),
                Cell::Vacant,
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
            ],
        };

        let mut diagonals = board.diagonals();

        let mut diagonal = diagonals.next().unwrap();
        assert_eq!(Cell::Occupied(Player::Nought), *diagonal.next().unwrap());
        assert_eq!(Cell::Vacant, *diagonal.next().unwrap());
        assert_eq!(Cell::Occupied(Player::Cross), *diagonal.next().unwrap());
        assert!(diagonal.next().is_none());

        let mut diagonal = diagonals.next().unwrap();
        assert_eq!(Cell::Vacant, *diagonal.next().unwrap());
        assert_eq!(Cell::Vacant, *diagonal.next().unwrap());
        assert_eq!(Cell::Vacant, *diagonal.next().unwrap());
        assert!(diagonal.next().is_none());

        assert!(diagonals.next().is_none());
    }

    #[test]
    fn board_is_complete() {
        let board = Board {
            cells: [Cell::Occupied(Player::Cross); 9],
        };
        assert!(board.is_complete());

        let board = Board {
            cells: [Cell::Vacant; 9],
        };
        assert!(!board.is_complete());

        let board = Board {
            cells: [
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Nought),
                Cell::Vacant,
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
            ],
        };
        assert!(!board.is_complete());
    }

    #[test]
    fn board_wins() {
        let board = Board {
            cells: [
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
                Cell::Vacant,
                Cell::Occupied(Player::Cross),
                Cell::Vacant,
                Cell::Occupied(Player::Nought),
                Cell::Vacant,
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
            ],
        };
        assert!(!board.wins(Player::Nought));
        assert!(!board.wins(Player::Cross));

        let board = Board {
            cells: [
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Nought),
            ],
        };
        assert!(board.wins(Player::Nought));
        assert!(!board.wins(Player::Cross));
    }

    #[test]
    fn board_is_draw() {
        let board = Board {
            cells: [
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
                Cell::Vacant,
                Cell::Occupied(Player::Cross),
                Cell::Vacant,
                Cell::Occupied(Player::Nought),
                Cell::Vacant,
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
            ],
        };
        assert!(!board.is_draw());

        let board = Board {
            cells: [
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Nought),
            ],
        };
        assert!(!board.is_draw());

        let board = Board {
            cells: [
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
                Cell::Occupied(Player::Nought),
                Cell::Occupied(Player::Cross),
            ],
        };
        eprintln!("{}", board);
        assert!(board.is_draw());
    }
}

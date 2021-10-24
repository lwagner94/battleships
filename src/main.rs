
use rand::Rng;
use regex::Regex;
use crate::FireResult::Hit;
use std::io::{BufRead, Write};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[derive(Clone, PartialEq)]
enum CellStatus {
    Free,
    FreeAndHit,
    Ship(usize),
    ShipAndHit(usize)
}

impl Default for CellStatus {
    fn default() -> Self {
        CellStatus::Free
    }
}

#[derive(Clone)]
struct Cell {
    status: CellStatus,
}

enum FireResult {
    Miss,
    Hit,
    Sink
}

struct Ship {
    hp: u8,
    max_hp: u8
}

struct Field {
    cells: Vec<Cell>,
    ships: Vec<Ship>,
    width: usize,
    height: usize,
    alive_ships: usize,
}

impl Field {
    fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![Cell {
                status: CellStatus::Free
            }; width * height],
            ships: Vec::new(),
            width,
            height,
            alive_ships: 0
        }
    }

    fn draw(&self) {
        println!("Ships alive: {}", self.alive_ships);

        print!("    ");
        for row_nr in 0..self.width {
            print!(" {} ", ('A' as usize + row_nr) as u8 as char);
        }

        print!("\n");

        for line_nr in 0..self.height {
            print!("{:2}  ", line_nr + 1);

            for row_nr in 0..self.width {
                let index = line_nr * self.width + row_nr;

                match self.cells[index].status {
                    CellStatus::Free => {
                        print!("-|-")
                    },
                    CellStatus::FreeAndHit => {
                        print!("-o-")
                    },
                    CellStatus::Ship(id) => {
                        print!("-&-")
                    },
                    CellStatus::ShipAndHit(id) => {
                        print!("-x-")
                    }
                }

            }
            print!("\n\n");
        }
    }

    pub fn add_ships(&mut self, number_of_ships: usize) {
        let mut rng = rand::thread_rng();
        let mut placed = 0;

        while placed != number_of_ships {
            let index = rng.gen_range(0..(self.width * self.height - 2));

            if (index + 1) % self.width == 0 || (index + 2) % self.width == 0 {
                continue;
            }

            if self.cells[index].status == CellStatus::Free &&
                self.cells[index + 1].status == CellStatus::Free &&
                self.cells[index + 2].status == CellStatus::Free {

                self.cells[index].status = CellStatus::Ship(placed);
                self.cells[index + 1].status = CellStatus::Ship(placed);
                self.cells[index + 2].status = CellStatus::Ship(placed);

                self.ships.push(Ship {
                    hp: 3,
                    max_hp: 3
                });

                placed += 1;
            }

        }

        self.alive_ships = number_of_ships;
    }

    pub fn coordinates_to_index(&self, coords: &str) -> Option<usize> {
        // TODO: Lazy static?
        let re = Regex::new(r"^([A-Za-z])(\d+)$").unwrap();
        let caps = re.captures(coords)?;
        let letter = caps.get(1)?.as_str();
        let digit = caps.get(2)?.as_str();

        let col: usize = letter.to_uppercase().chars().next().unwrap() as usize - 65;
        let row: usize = digit.parse::<usize>().unwrap() - 1;


        if col >= 0 && col < self.width && row >= 0 && row < self.height {
            Some(row * self.width + col)
        } else {
            None
        }
    }

    pub fn fire(&mut self, coords: &str) -> Option<FireResult> {
        let index = self.coordinates_to_index(coords)?;

        Some(match self.cells.get(index).unwrap().status {
            (CellStatus::Free | CellStatus::FreeAndHit) => {
                self.cells.get_mut(index).unwrap().status = CellStatus::FreeAndHit;
                FireResult::Miss
            },
            CellStatus::Ship(id) => {
                self.ships.get_mut(id).unwrap().hp -= 1;
                self.cells.get_mut(index).unwrap().status = CellStatus::ShipAndHit(id);

                if self.ships.get(id).unwrap().hp == 0 {
                    self.alive_ships -= 1;
                    FireResult::Sink
                } else {
                    FireResult::Hit
                }
            },
            CellStatus::ShipAndHit(id) => {
                FireResult::Hit
            }
        })
    }

    pub fn is_game_over(&self) -> bool {
        self.alive_ships == 0
    }
}




fn main() {
    let stdin = std::io::stdin();
    let mut input_lines = stdin.lock().lines();

    let mut field = Field::new(WIDTH, HEIGHT);
    field.add_ships(2);

    while !field.is_game_over() {
        field.draw();
        print!("Next shot: ");
        std::io::stdout().flush();
        let next_input = input_lines.next().unwrap().unwrap();
        // if let Some(result) = field.fire(&next_input) {
        //
        // }

        match field.fire(&next_input) {
            Some(FireResult::Hit) => println!("Hit"),
            Some(FireResult::Miss) => println!("Missed"),
            Some(FireResult::Sink) => println!("Ship sunk!"),
            None => println!("Invalid input")
        }

    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_coordinates_to_index() {
        let field = Field::new(10, 10);
        assert_eq!(field.coordinates_to_index("A1"), Some(0));
        assert_eq!(field.coordinates_to_index("A2"), Some(10));
        assert_eq!(field.coordinates_to_index("B1"), Some(1));
        assert_eq!(field.coordinates_to_index("B4"), Some(31));
    }


}
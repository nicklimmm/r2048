use std::io::{self, Write};

use self::grid::{Action, Grid};

mod grid;

pub struct Game {
    grid: Grid,
}

impl Game {
    pub fn new() -> Game {
        Game { grid: Grid::new() }
    }

    pub fn run(&mut self) {
        println!("~~~~Welcome to r2048!~~~~");
        while !self.grid.game_over() {
            println!("\nCurrent state:");
            println!("{}", self.grid);

            let action = Self::get_input_action();

            // Clear output and put cursor at first row and column
            print!("\x1B[2J\x1B[1;1H");

            match action {
                Some(action) => {
                    self.grid.update(action);
                    self.grid.insert_random_cell();
                }
                None => {
                    break;
                }
            }
        }

        println!("\n~~~~Game Over!~~~~");
        println!("\nFinal state:");
        println!("{}", self.grid);
    }

    const MAX_INPUT_TRIES: u8 = 3;
    fn get_input_action() -> Option<Action> {
        for tries in 1..=Self::MAX_INPUT_TRIES {
            println!("\nAction options: 1) Up, 2) Right, 3) Down, 4) Left");
            print!("Enter your action: ");

            io::stdout().flush().unwrap();
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();

            match buf.trim() {
                "1" => return Some(Action::Up),
                "2" => return Some(Action::Right),
                "3" => return Some(Action::Down),
                "4" => return Some(Action::Left),
                _ => {
                    println!("\n~~~~Invalid input~~~~");
                    println!("{}/{} tries", tries, Self::MAX_INPUT_TRIES);
                }
            };
        }

        return None;
    }
}

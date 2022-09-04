use std::fmt;
use std::io;

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = if self == &Player::O { "O" } else { "X" };
        write!(f, "{}", symbol)?;
        Ok(())
    }
}

pub struct Game {
    pub board: [[Option<Player>; 3]; 3],
    pub current_player: Player,
}

fn input_to_usize(prompt: &str) -> usize {
    let output: usize;

    loop {
        let mut input = String::new();
        println!("{}", prompt);

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        match input.trim().parse::<usize>() {
            Ok(n) => {
                output = n;
                break;
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
    }

    output
}

impl Game {
    fn new(p: Player) {
        let game = Game {
            board: [[None; 3]; 3],
            current_player: p,
        };

        game.place();
    }

    fn place(mut self) {
        loop {
            println!("{}", self);

            let y = input_to_usize("Row:");

            if self.board.get(y) == None {
                println!("Out of bounds.");
                continue;
            }

            let x = input_to_usize("Column:");

            match self.board[y].get(x) {
                None => {
                    println!("Out of bounds.");
                    continue;
                }
                Some(None) => {
                    self.board[y][x] = Some(self.current_player);
                    break;
                }
                Some(_) => {
                    println!("Square already filled.");
                    continue;
                }
            }
        }

        self.check_win();
    }

    fn check_matches(&self, y: usize, x: usize, dy: isize, dx: usize) -> bool {
        const NEEDED: usize = 3;

        let player = self.board[y][x];

        if player == None {
            return false;
        }

        (1..NEEDED).fold(true, |won, i| {
            // TODO: can I fix this?
            if self.board[((y as isize) + (dy * (i as isize))) as usize][x + dx * i] != player {
                false
            } else {
                won
            }
        })
    }

    fn check_win(mut self) {
        const NEEDED: usize = 3;

        let mut won = false;

        for (y, row) in self.board.into_iter().enumerate() {
            for (x, cell) in row.into_iter().enumerate() {
                if cell != Some(self.current_player) {
                    break;
                }

                // match horizontal
                // TODO: move overflow checks into check_matches()
                if x <= row.len() - NEEDED {
                    if self.check_matches(y, x, 0, 1) {
                        won = true;
                        break;
                    }
                }

                // match vertical
                if y <= self.board.len() - NEEDED {
                    if self.check_matches(y, x, 1, 0) {
                        won = true;
                        break;
                    }
                }

                // match diagonal down
                if x <= row.len() - NEEDED && y <= self.board.len() - NEEDED {
                    if self.check_matches(y, x, 1, 1) {
                        won = true;
                        break;
                    }
                }

                // match diagonal up
                if x <= row.len() - NEEDED && y >= NEEDED {
                    if self.check_matches(y, x, -1, 1) {
                        won = true;
                        break;
                    }
                }
            }
        }

        // if X won, O starts

        if won {
            println!("{} won!", self.current_player);
        }

        self.current_player = match self.current_player {
            Player::O => Player::X,
            _ => Player::O,
        };

        if won {
            Game::new(self.current_player);
        } else {
            self.place();
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.board {
            for cell in row {
                let symbol: String = match &cell {
                    None => String::from(" "),
                    Some(p) => p.to_string(),
                };
                write!(f, "[{}]", symbol)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "It's {}'s turn.\n", &self.current_player.to_string())?;

        Ok(())
    }
}

pub fn main() {
    Game::new(Player::X);
}

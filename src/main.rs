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
        let player = self.board[y][x];

        // TODO: or do this check in check_win()?
        if player == None {
            return false;
        }

        const NEEDED: usize = 3;

        // TODO: do without type conversions?
        let max_y = y as isize + ((NEEDED as isize - 1) * dy);
        let max_x = x + (NEEDED - 1) * dx;

        // prevent out of bounds
        if max_y >= NEEDED as isize || max_y < 0 || max_x >= NEEDED {
            return false;
        }

        (1..NEEDED).find(|&i| {
            self.board[((y as isize) + (dy * (i as isize))) as usize][x + dx * i] != player
        }) == None
    }

    fn check_win(mut self) {
        let mut won = false;

        for (y, row) in self.board.into_iter().enumerate() {
            for (x, _) in row.into_iter().enumerate() {
                won = [
                    // horizontal
                    self.check_matches(y, x, 0, 1),
                    // vertical
                    self.check_matches(y, x, 1, 0),
                    // diagonal down
                    self.check_matches(y, x, 1, 1),
                    // diagonal up
                    self.check_matches(y, x, -1, 1),
                ]
                .into_iter()
                .find(|&x| x == true)
                    != None;
            }
        }

        if won {
            println!("{} won!", self.current_player);
        }

        // if X won, O starts
        self.current_player = if self.current_player == Player::O {
            Player::X
        } else {
            Player::O
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

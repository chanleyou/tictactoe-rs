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
        let g = Game {
            board: [[None; 3]; 3],
            current_player: p,
        };

        g.place();
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

    fn check_win(mut self) {
        // TODO: victory logic
        for row in &self.board {
            for cell in row {
                // match horizontal
                // skip if last two cols
                // match vertical
                // skip if last two rows
                // match diagonal up
                // skip if first two rows or last two cols
                // match diagonal down
                // skip if last two rows or last two cols
            }
        }

        // if X won, O starts

        self.current_player = match self.current_player {
            Player::O => Player::X,
            _ => Player::O,
        };

        self.place();
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

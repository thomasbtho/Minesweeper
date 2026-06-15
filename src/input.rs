use std::{
    io::{self, Write},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
pub enum PlayerAction {
    Reveal { x: usize, y: usize },
    Flag { x: usize, y: usize },
    Quit,
}

impl FromStr for PlayerAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action: Vec<&str> = s.trim().split_whitespace().collect();

        if action.len() == 1 && action[0].eq_ignore_ascii_case("quit")
            || action[0].eq_ignore_ascii_case("exit")
        {
            return Ok(PlayerAction::Quit);
        }

        if action.len() != 3 {
            return Err("Error: wrong input format! Expected 'x y cmd'.".to_string());
        }

        let x = action[0]
            .parse::<usize>()
            .map_err(|_| "x is not a number")?
            - 1;
        let y = action[1]
            .parse::<usize>()
            .map_err(|_| "y is not a number")?
            - 1;

        match action[2].to_lowercase().as_str() {
            "free" | "reveal" => Ok(PlayerAction::Reveal { x, y }),
            "mine" | "flag" => Ok(PlayerAction::Flag { x, y }),
            _ => Err(format!("Unknown command '{}'", action[2])),
        }
    }
}

pub fn read_usize(prompt: &str) -> Result<usize, String> {
    print!("{}", prompt);
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    input
        .trim()
        .parse::<usize>()
        .map_err(|_| "Error: please enter a valid positive whole number!".to_string())
}

pub fn read_usize_in_range(prompt: &str, min: usize, max: usize) -> usize {
    loop {
        match read_usize(&format!("{} [{}, {}]? ", prompt, min, max)) {
            Ok(val) if (min..=max).contains(&val) => return val,
            Ok(_) => println!("Error: value must be between {} and {}!", min, max),
            Err(err) => println!("{}", err),
        }
    }
}

pub fn get_player_action() -> Result<PlayerAction, String> {
    print!("Flag/unflag mines or claim a cell as free: ");

    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    input.parse::<PlayerAction>()
}

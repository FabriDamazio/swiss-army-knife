use crate::cli::TimerAction;

pub fn handle(action: TimerAction) {
    println!("Action: {:?}", action);

    match action {
        TimerAction::Start { duration } => {
            println!("Duration match!");
            let total_seconds = parse_duration(&duration);
            println!("Timer set for {} seconds", total_seconds);
        }
    }
}

fn parse_duration(input: &str) -> u64 {
    println!("Input: {:?}", input);
    let num: Result<u64, _> = input.parse();
    match num {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid duration input");
            0
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seconds() {
        assert_eq!(parse_duration("10"), 10);
    }
}

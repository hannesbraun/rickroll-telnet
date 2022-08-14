use std::io::Write;

fn sleep(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis));
}

fn parse_delay(token: &str) -> Option<u64> {
    let stripped = token
        .trim_matches(|c| c == ' ' || c == '-')
        .strip_prefix('{')?
        .strip_suffix('}')?;
    if stripped.chars().all(|c| c.is_ascii_digit()) {
        stripped.parse::<u64>().ok()
    } else {
        None
    }
}

#[derive(PartialEq)]
enum LastOutput {
    Delay,
    Text,
}

const DELAY_WORD: u64 = 200;
const DELAY_LINE: u64 = 1000;

fn main() {
    if let Ok(lyrics) = std::fs::read_to_string("lyrics.dat") {
        let mut last_output = LastOutput::Text;
        let lines: Vec<&str> = lyrics.split('\n').collect();
        for line in lines {
            let tokens: Vec<&str> = line.split_inclusive(&[' ', '-']).collect();
            for (i, token) in tokens.iter().enumerate() {
                if let Some(delay) = parse_delay(token) {
                    sleep(delay);
                    last_output = LastOutput::Delay;
                } else {
                    let mut cutted = token.strip_suffix(' ').unwrap_or(token);
                    if i >= tokens.len() {
                        cutted = cutted.strip_suffix(' ').unwrap_or(cutted);
                    }
                    print!("{}", cutted);
                    let _ = std::io::stdout().flush();
                    sleep(DELAY_WORD);

                    last_output = LastOutput::Text;
                }
            }

            if last_output != LastOutput::Delay {
                sleep(DELAY_LINE);
            }
            println!();
        }
    }
}

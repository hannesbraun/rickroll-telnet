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

pub fn record() -> Vec<RickAction> {
    let mut recording = Vec::new();
    if let Ok(lyrics) = std::fs::read_to_string("lyrics.dat") {
        let mut last_output = LastOutput::Text;
        let lines: Vec<&str> = lyrics.split('\n').collect();
        for line in lines {
            let tokens: Vec<&str> = line.trim().split_inclusive(&[' ', '-']).collect();
            for token in tokens {
                if let Some(delay) = parse_delay(token) {
                    recording.push(RickAction::Delay(delay));
                    last_output = LastOutput::Delay;
                } else {
                    recording.push(RickAction::Write(
                        token.strip_suffix('-').unwrap_or(token).to_string(),
                    ));
                    recording.push(RickAction::Delay(DELAY_WORD));

                    last_output = LastOutput::Text;
                }
            }

            if last_output != LastOutput::Delay {
                recording.push(RickAction::Delay(DELAY_LINE));
            }
            recording.push(RickAction::Write(String::from("\n")));
        }
    }

    recording
}

pub enum RickAction {
    Delay(u64),
    Write(String),
}

pub fn replay(mut out: Box<dyn Write>, recording: &Vec<RickAction>) -> std::io::Result<()> {
    for action in recording {
        match action {
            RickAction::Delay(millis) => {
                sleep(*millis);
            }
            RickAction::Write(str) => {
                write!(&mut out, "{}", str)?;
                let _ = std::io::stdout().flush();
            }
        }
    }

    Ok(())
}

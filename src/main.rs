use std::io::Write;

fn sleep(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis));
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
        let delay_regex = regex::Regex::new(r"\{(\d+)\}").unwrap();
        let lines: Vec<&str> = lyrics.split('\n').collect();
        for line in lines {
            let tokens: Vec<&str> = line.split_inclusive(&[' ', '-']).collect();
            for (i, token) in tokens.iter().enumerate() {
                if delay_regex.is_match(token) {
                    let delay = delay_regex
                        .captures_iter(token)
                        .next()
                        .map_or(0, |c| c[1].parse::<u64>().unwrap_or(0));
                    sleep(delay);
                    last_output = LastOutput::Delay;
                } else {
                    let mut cutted = token.strip_suffix("-").unwrap_or(token);
                    if i >= tokens.len() {
                        cutted = cutted.strip_suffix(" ").unwrap_or(cutted);
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
            print!("\n");
        }
    }
}

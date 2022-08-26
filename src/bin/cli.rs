use rickroll_telnet::{record, replay};
use std::io;

fn main() -> io::Result<()> {
    replay(Box::new(io::stdout()), &record())?;
    Ok(())
}

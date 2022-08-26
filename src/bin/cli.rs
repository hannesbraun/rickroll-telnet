use rickroll_telnet::do_not_give_me_up;
use std::io;

fn main() -> io::Result<()> {
    do_not_give_me_up(Box::new(io::stdout()))?;
    Ok(())
}

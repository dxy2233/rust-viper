use std::io::{self, stdout};
use viper::game;

fn main() -> io::Result<()> {
    let mut stdout = stdout();

    game::start(&mut stdout)
}

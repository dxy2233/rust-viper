use crossterm::{
    cursor::{self, MoveTo},
    event, execute, queue,
    style::{self, Color, Print, Stylize},
    terminal,
};
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

const CREAM_COLOR: Color = Color::Rgb {
    r: 255,
    g: 253,
    b: 208,
};

pub fn start(stdout: &mut impl Write) -> io::Result<()> {
    let t_size = terminal::size()?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    execute!(stdout, cursor::Hide)?;
    draw_border(stdout, t_size)?;

    terminal::enable_raw_mode()?;

    // execute!(stdout, cursor::MoveTo(10, 5))?;
    // execute!(stdout, Print("Hello, world!"))?;

    let mut time = Instant::now();
    let mut sit = (10, 5);

    loop {
        // if event::poll(Duration::from_millis(500))? {
        //
        // }

        if time.elapsed() >= Duration::from_millis(100) {
            if sit.0 == t_size.0 {
                break;
            }
            execute!(stdout, MoveTo(sit.0, sit.1), Print(" "))?;
            sit.0 += 1;
            execute!(
                stdout,
                MoveTo(sit.0, sit.1),
                style::PrintStyledContent("█".with(CREAM_COLOR))
            )?;
            time = Instant::now();
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}

fn draw_border(stdout: &mut impl Write, t_size: (u16, u16)) -> io::Result<()> {
    let (width, height) = t_size;
    for y in 0..=height {
        for x in 0..=width {
            if (y == 0 || y == height - 1) || (x == 0 || x == width - 1) {
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent("█".magenta())
                )?;
            }
        }
    }
    stdout.flush()?;
    Ok(())
}

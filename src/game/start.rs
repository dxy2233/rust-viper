use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode},
    execute, queue,
    style::{self, Color, Print, Stylize},
    terminal,
};
use rand::Rng;
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn set_direction(&mut self, new_direction: &Direction) {
        if (*self == Direction::Up && new_direction != &Direction::Down)
            || (*self == Direction::Down && new_direction != &Direction::Up)
            || (*self == Direction::Left && new_direction != &Direction::Right)
            || (*self == Direction::Right && new_direction != &Direction::Left)
        {
            *self = *new_direction;
        }
    }
}

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

    let mut rng = rand::thread_rng();
    let mut time = Instant::now();
    let mut direction = Direction::Down;
    let mut btn_direction = Direction::Down;
    let mut snake = vec![(10, 5), (10, 6), (10, 7)];
    let mut food = (1, 1);

    for i in snake.iter() {
        execute!(
            stdout,
            MoveTo(i.0, i.1),
            style::PrintStyledContent("█".with(CREAM_COLOR))
        )?
    }

    loop {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Up => btn_direction = Direction::Up,
                    KeyCode::Down => btn_direction = Direction::Down,
                    KeyCode::Left => btn_direction = Direction::Left,
                    KeyCode::Right => btn_direction = Direction::Right,
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        if time.elapsed() >= Duration::from_millis(500) {
            execute!(stdout, MoveTo(snake[0].0, snake[0].1), Print(" "))?;

            direction.set_direction(&btn_direction);
            snake_move(&mut snake, &direction);

            food.0 = rng.gen_range(1..t_size.0 - 1);
            food.1 = rng.gen_range(1..t_size.1 - 1);
            execute!(
                stdout,
                MoveTo(food.0, food.1),
                style::PrintStyledContent("█".cyan())
            )?;

            let head = snake.last().unwrap();
            execute!(
                stdout,
                MoveTo(head.0, head.1),
                style::PrintStyledContent("█".with(CREAM_COLOR))
            )?;

            if head.0 == t_size.0 - 1 || head.0 == 0 || head.1 == t_size.1 - 1 || head.1 == 0 {
                break;
            }
            time = Instant::now();
        }
    }

    terminal::disable_raw_mode()?;
    Ok(())
}

fn draw_border(stdout: &mut impl Write, t_size: (u16, u16)) -> io::Result<()> {
    let (width, height) = t_size;
    for y in 0..height {
        for x in 0..width {
            if y == 0 || y == height - 1 || x == 0 || x == width - 1 {
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

fn snake_move(snake: &mut Vec<(u16, u16)>, direction: &Direction) {
    let now_head = snake.last().unwrap();
    let new_head = match direction {
        Direction::Up => (now_head.0, now_head.1 - 1),
        Direction::Down => (now_head.0, now_head.1 + 1),
        Direction::Left => (now_head.0 - 1, now_head.1),
        Direction::Right => (now_head.0 + 1, now_head.1),
    };
    snake.push(new_head);
    snake.remove(0);
}

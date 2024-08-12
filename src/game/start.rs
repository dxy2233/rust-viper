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
    let mut food: Option<(u16, u16)> = Some((
        rng.gen_range(1..t_size.0 - 1),
        rng.gen_range(1..t_size.1 - 1),
    ));

    if let Some((x, y)) = food {
        execute!(stdout, MoveTo(x, y), style::PrintStyledContent("█".cyan()))?;
    }
    for i in snake.iter() {
        execute!(
            stdout,
            MoveTo(i.0, i.1),
            style::PrintStyledContent("█".with(CREAM_COLOR))
        )?
    }

    loop {
        if event::poll(Duration::from_millis(100))? {
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

        if time.elapsed() >= Duration::from_millis(100) {
            direction.set_direction(&btn_direction);
            let head = snake_next_head(&mut snake, &direction);

            match food {
                Some((x, y)) if head.0 == x && head.1 == y => {
                    let new_food = (
                        rng.gen_range(1..t_size.0 - 1),
                        rng.gen_range(1..t_size.1 - 1),
                    );
                    food.replace(new_food);
                    execute!(
                        stdout,
                        MoveTo(new_food.0, new_food.1),
                        style::PrintStyledContent("█".cyan())
                    )?;
                }
                _ => {
                    execute!(stdout, MoveTo(snake[0].0, snake[0].1), Print(" "))?;
                    snake.remove(0);
                }
            }
            snake.push(head);

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

fn snake_next_head(snake: &mut [(u16, u16)], direction: &Direction) -> (u16, u16) {
    let now_head = snake.last().unwrap();
    match direction {
        Direction::Up => (now_head.0, now_head.1 - 1),
        Direction::Down => (now_head.0, now_head.1 + 1),
        Direction::Left => (now_head.0 - 1, now_head.1),
        Direction::Right => (now_head.0 + 1, now_head.1),
    }
}

// fn create_food(food: &Option<(u16, u16)>) {}

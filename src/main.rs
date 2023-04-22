use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    Result, ExecutableCommand,
};
use std::io::{stdout, Write};
use std::time::Duration;

struct Paddle {
    x: u16,
    y: u16,
    height: u16,
    width: u16,
}

struct Ball {
    x: u16,
    y: u16,
    dx: i16,
    dy: i16,
    height: u16,
    width: u16,
}

struct GameState {
    left_score: u32,
    right_score: u32,
}

fn main() -> Result<()> {
    let mut left_paddle = Paddle {
        x: 4,
        y: 10,
        height: 5,
        width: 1,
    };
    let mut right_paddle = Paddle {
        x: 75,
        y: 10,
        height: 5,
        width: 1,
    };
    let mut ball = Ball {
        x: 40,
        y: 12,
        dx: 1,
        dy: 1,
        height: 1,
        width: 2,
    };

    impl Ball {
        fn reset_position(&mut self) {
            self.x = 40;
            self.y = 12;
            self.dx = -self.dx;
            self.dy = 1;
        }
    }

    let mut game_state = GameState {
        left_score: 0,
        right_score: 0,
    };

    // Hide cursor
    stdout().execute(Hide)?;

    enable_raw_mode()?;

    loop {
        // Handle input
        if poll(Duration::from_millis(200))? {
            let event = read()?;
            match event {
                Event::Key(key_event) => match key_event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => {
                        if right_paddle.y > 0 {
                            right_paddle.y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if right_paddle.y < 20 {
                            right_paddle.y += 1;
                        }
                    }
                    KeyCode::Char('w') => {
                        if left_paddle.y > 0 {
                            left_paddle.y -= 1;
                        }
                    }
                    KeyCode::Char('s') => {
                        if left_paddle.y < 20 {
                            left_paddle.y += 1;
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }

        // Update ball position
        ball.x = (ball.x as i16 + ball.dx as i16) as u16;
        ball.y = (ball.y as i16 + ball.dy as i16) as u16;

        // Update ball position and check for collisions
        ball.x = ((ball.x as i16) + ball.dx) as u16;
        ball.y = ((ball.y as i16) + ball.dy) as u16;

        // Check ball collisions with top and bottom walls
        if ball.y == 0 || ball.y == 23 - ball.height {
            ball.dy = -ball.dy;
        }

        // Check ball collisions with left and right paddles
        let left_paddle_collision = ball.x == left_paddle.x + left_paddle.width
            && ball.y >= left_paddle.y
            && ball.y <= left_paddle.y + left_paddle.height - ball.height;

        let right_paddle_collision = ball.x + ball.width == right_paddle.x
            && ball.y >= right_paddle.y
            && ball.y <= right_paddle.y + right_paddle.height - ball.height;

        if left_paddle_collision || right_paddle_collision {
            ball.dx = -ball.dx;
        }

        // Check ball collisions with left and right walls (score)
        if ball.x == 0 {
            game_state.right_score += 1;
            ball.reset_position();
        } else if ball.x >= 79 - ball.width {
            game_state.left_score += 1;
            ball.reset_position();
        }

        //draw paddles, ball, and score
        render(&left_paddle, &right_paddle, &ball, &game_state)?;
    }

    stdout().execute(Show)?;

    disable_raw_mode()?;

    Ok(())
}

fn render(paddle_left: &Paddle, paddle_right: &Paddle, ball: &Ball, game_state: &GameState) -> Result<()> {
    let mut stdout = stdout();

    // Clear screen
    stdout.execute(Clear(ClearType::All))?;

    //Scores
    stdout.execute(MoveTo(35, 1))?;
    stdout.execute(SetForegroundColor(Color::Green))?;
    write!(stdout, "{} - {}", game_state.left_score, game_state.right_score)?;
    stdout.execute(ResetColor)?;

    // Draw paddles
    for i in 0..paddle_left.height {
        stdout.execute(MoveTo(paddle_left.x, paddle_left.y + i))?;
        stdout.execute(SetBackgroundColor(Color::White))?;
        stdout.execute(Print(" "))?;
        stdout.execute(ResetColor)?;

        stdout.execute(MoveTo(paddle_right.x, paddle_right.y + i))?;
        stdout.execute(SetBackgroundColor(Color::White))?;
        stdout.execute(Print(" "))?;
        stdout.execute(ResetColor)?;
    }

    // Draw ball
    stdout.execute(MoveTo(ball.x, ball.y))?;
    stdout.execute(SetForegroundColor(Color::Yellow))?;
    stdout.execute(Print("■"))?;
    stdout.execute(ResetColor)?;

    stdout.flush()?;

    Ok(())
}
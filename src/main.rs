// these are the things that I need
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent, UpdateEvent,
};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics,
    left_score: isize,
    right_score: isize,
}

impl App {
    fn render(
        &mut self,
        args: &RenderArgs,
        left_paddle: &Paddle,
        right_paddle: &Paddle,
        ball: &Ball,
    ) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
        const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let left = rectangle::square(0.0, 0.0, 100.0);
        let right = rectangle::square(0.0, 0.0, 100.0);

        let ball_blob = rectangle::square(0.0, 0.0, 10.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            rectangle(
                FOREGROUND,
                left,
                c.transform.trans(-90.0, left_paddle.pos as f64),
                gl,
            );
            rectangle(
                FOREGROUND,
                right,
                c.transform
                    .trans((args.width - 10) as f64, right_paddle.pos as f64),
                gl,
            );

            rectangle(
                FOREGROUND,
                ball_blob,
                c.transform.trans(ball.x as f64, ball.y as f64),
                gl,
            );
        });
    }

    fn update_score(
        self,
        ball: &Ball,
        window: &Window,
        left_paddle: &Paddle,
        right_paddle: &Paddle,
    ) -> App {
        match ball.x {
            x if x < 1
                && (ball.y < left_paddle.pos || ball.y > left_paddle.pos + left_paddle.height) =>
            {
                App {
                    right_score: self.right_score + 1,
                    ..self
                }
            }
            x if x >= window.width
                && (ball.y < right_paddle.pos
                    || ball.y > right_paddle.pos + right_paddle.height) =>
            {
                App {
                    left_score: self.left_score + 1,
                    ..self
                }
            }
            _ => self,
        }
    }

    fn press_up_down(&self, args: &Button, right_paddle: Paddle) -> Paddle {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => match right_paddle.vel {
                    vel if vel > 0 => {
                        right_paddle.update_vel(-vel).set_should_move(true)
                    }
                    _ => right_paddle.set_should_move(true),
                },
                Key::Down => match right_paddle.vel {
                    vel if vel < 0 => {
                        right_paddle.update_vel(-vel).set_should_move(true)
                    }
                    _ => right_paddle.set_should_move(true),
                },
                _ => right_paddle,
            }
        } else {
            right_paddle
        }
    }

    fn release_up_down(&self, args: &Button, right_paddle: Paddle) -> Paddle {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up | Key::Down => right_paddle.set_should_move(false),
                _ => right_paddle,
            }
        } else {
            right_paddle
        }
    }

    fn press_w_s(&self, args: &Button, left_paddle: Paddle) -> Paddle {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::W => match left_paddle.vel {
                    vel if vel > 0 => left_paddle.update_vel(-vel).set_should_move(true),
                    _ => left_paddle.set_should_move(true),
                },
                Key::S => match left_paddle.vel {
                    vel if vel < 0 => left_paddle.update_vel(-vel).set_should_move(true),
                    _ => left_paddle.set_should_move(true),
                },
                _ => left_paddle,
            }
        } else {
            left_paddle
        }
    }

    fn release_w_s(&self, args: &Button, left_paddle: Paddle) -> Paddle {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::W | Key::S => left_paddle.set_should_move(false),
                _ => left_paddle,
            }
        } else {
            left_paddle
        }
    }
}

struct Window {
    width: isize,
    height: isize,
}

struct Ball {
    x: isize,
    y: isize,
    vel_x: isize,
    vel_y: isize,
}

impl Ball {
    fn update(&self, window: &Window, left_paddle: &Paddle, right_paddle: &Paddle) -> Ball {
        let mut ball = Ball { ..*self };
        match self.x {
            x if x >= window.width - 10
                && (self.y >= right_paddle.pos
                    && self.y <= right_paddle.pos + right_paddle.height) =>
            {
                ball.vel_x = -self.vel_x;
                ball.x = self.x + ball.vel_x;
            }
            x if x <= 10
                && (self.y >= left_paddle.pos
                    && self.y <= left_paddle.pos + left_paddle.height) =>
            {
                ball.vel_x = -self.vel_x;
                ball.x = self.x + ball.vel_x;
            }
            x if (x > window.width) || (x < 1) => {
                ball.x = window.width / 2;
                ball.y = window.height / 2;
            }
            x => ball.x = x + self.vel_x,
        };

        match self.y {
            y if (y > window.height) || (y < 0) => {
                ball.vel_y = -self.vel_y;
                ball.y = self.y + ball.vel_y;
            }
            y => ball.y = y + self.vel_y,
        };

        ball
    }
}

struct Paddle {
    pos: isize,
    height: isize,
    should_move: bool,
    vel: isize,
}

impl Paddle {
    fn new_with_default_vel(start_pos: isize, height: isize) -> Paddle {
        Paddle {
            pos: start_pos,
            height,
            should_move: false,
            vel: 2,
        }
    }

    fn update_vel(self, new_vel: isize) -> Paddle {
        Paddle {
            vel: new_vel,
            ..self
        }
    }

    fn set_should_move(self, should_move: bool) -> Paddle {
        Paddle {
            should_move,
            ..self
        }
    }

    fn update_pos(self, window: &Window) -> Paddle {
        let max_window_height = window.height - self.height;

        let new_pos = self.pos + self.vel;
        if self.should_move && new_pos >= 0 && new_pos <= max_window_height {

            Paddle {
                pos: new_pos,
                ..self
            }
        } else {

            self
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Pong", [512, 342])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let window_frame = Window {
        width: 512,
        height: 342,
    };

    let mut app = App {
        gl: GlGraphics::new(opengl),
        left_score: 0,
        right_score: 0,
    };

    let mut events = Events::new(EventSettings::new());

    let mut left_paddle: Paddle = Paddle::new_with_default_vel(0, 100);
    let mut right_paddle: Paddle = Paddle::new_with_default_vel(0, 100);
    let mut ball: Ball = Ball {
        x: 0,
        y: 0,
        vel_x: 1,
        vel_y: 1,
    };

    while let Some(e) = events.next(&mut window) {
        match e.render_args() {
            Some(r) => app.render(&r, &left_paddle, &right_paddle, &ball),
            None => (),
        }

        match e.update_args() {
            Some(_u) => {
                ball = ball.update(&window_frame, &left_paddle, &right_paddle);
                left_paddle = left_paddle.update_pos(&window_frame);
                right_paddle = right_paddle.update_pos(&window_frame);
                app = app.update_score(&ball, &window_frame, &left_paddle, &right_paddle);
            }
            None => {}
        };

        match e.press_args() {
            Some(b) => {
    
                left_paddle = app.press_w_s(&b, left_paddle);
                right_paddle = app.press_up_down(&b, right_paddle);
            }
            None => (),
        }

        match e.release_args() {
            Some(b) => {
    
                left_paddle = app.release_w_s(&b, left_paddle);
                right_paddle = app.release_up_down(&b, right_paddle);
            }
            None => (),
        }
    }
}

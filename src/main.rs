use macroquad::prelude::*;

const BAT_SIZE: Vec2 = vec2(10f32, 150f32);
const BAT_SPEED: f32 = 800f32;
const BALL_RADIUS: f32 = 15f32;
const BALL_SPEED: f32 = 700f32;

struct Bat {
    rect: Rect,
    velocity: f32,
    controls: (KeyCode, KeyCode),
}

impl Bat {
    fn new(xpos: f32, controls: (KeyCode, KeyCode)) -> Self {
        Self {rect: Rect::new(xpos, (screen_height() - BAT_SIZE.y) / 2f32, 1f32, BAT_SIZE.y), velocity: 0f32, controls}
    }

    fn update(&mut self, dt: f32) {
        self.rect.y += self.velocity * dt;

        if self.rect.y < 0f32 {self.rect.y = 0f32}
        if self.rect.y + self.rect.h > screen_height() {self.rect.y = screen_height() - self.rect.h}
    }

    fn input(&mut self) {
        match (is_key_down(self.controls.0), is_key_down(self.controls.1)) {
            (true, false) => self.velocity = -BAT_SPEED,
            (false, true) => self.velocity = BAT_SPEED,
            _ => self.velocity = 0f32,
        }
    }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, BAT_SIZE.x, self.rect.h, WHITE);
    }
    
}

struct Ball {
    rect: Rect,
    velocity: Vec2,
}

impl Ball {
    fn new() -> Self {
        Self {
            rect: Rect::new(screen_width() / 2f32 - BALL_RADIUS, screen_height() / 2f32 - BALL_RADIUS, BALL_RADIUS * 2f32, BALL_RADIUS * 2f32),
            velocity: vec2(rand::gen_range(-1f32, 1f32), rand::gen_range(-1f32, 1f32)).normalize() * BALL_SPEED
        }
    }

    fn set_angle(&mut self, bat: &Bat) {
        let dist_from_centre = (bat.rect.y + BAT_SIZE.y / 2f32) - (self.rect.y + BALL_RADIUS);
        let yflip = if dist_from_centre > 0f32 {-1f32} else {1f32};
        let xflip = if self.velocity.x < 0f32 {1f32} else {-1f32};
        let angle = dist_from_centre.abs() / (BAT_SIZE.y / 2f32) * 60f32.to_radians();
        let length = self.velocity.length().abs();
        self.velocity = vec2(length * angle.cos() * xflip, length * angle.sin() * yflip);
    }

    fn update(&mut self, dt: f32, p1: &Bat, p2: &Bat) -> bool {
        self.rect.x += self.velocity.x * dt;
        self.rect.y += self.velocity.y * dt;

        if self.rect.x < 0f32 || self.rect.x + BALL_RADIUS * 2f32 > screen_width() {
            return true;
        }

        if self.rect.y < 0f32 {
            self.velocity.y = -self.velocity.y;
            self.rect.y = 0f32;
        }
        else if self.rect.y + BALL_RADIUS * 2f32 > screen_height() {
            self.velocity.y = -self.velocity.y;
            self.rect.y = screen_height() - BALL_RADIUS * 2f32;
        }

        if self.rect.overlaps(&p1.rect) && self.velocity.x < 0f32 {
            self.set_angle(p1);
        }
        else if self.rect.overlaps(&p2.rect) && self.velocity.x > 0f32 {
            self.set_angle(p2)
        }

        return false;
    }

    fn draw(&self) {
        draw_circle(self.rect.x + BALL_RADIUS, self.rect.y + BALL_RADIUS, BALL_RADIUS, WHITE)
    }
}

#[macroquad::main("Pong!")]
async fn main() {
    let mut p1 = Bat::new(50f32, (KeyCode::W, KeyCode::S));
    let mut p2 = Bat::new(screen_width()-50f32-BAT_SIZE.x, (KeyCode::P, KeyCode::Semicolon));

    let mut ball = Ball::new();

    loop {
        clear_background(BLACK);
        draw_rectangle(screen_width() / 2f32 - 5f32, 0f32, 10f32, screen_height(), WHITE);

        p1.input();
        p2.input();

        p1.update(get_frame_time());
        p2.update(get_frame_time());
        let scored = ball.update(get_frame_time(), &p1, &p2);

        if scored {
            ball = Ball::new();
        }

        p1.draw();
        p2.draw();
        ball.draw();

        next_frame().await
    }
}
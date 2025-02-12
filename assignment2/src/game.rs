use piston_window::*;

use rand::{thread_rng, Rng};
use snake::{Direction, Snake};
use utils::{draw_block, draw_rectange};

const MOVING_SPEED: f64 = 0.1;
const NUM_FOODS: usize = 3;
const FOOD_DECAY_SPEED: f32 = 0.002;
const INIT_FOOD_LIFE: f32 = 1.0;

pub struct Food {
    pub x: i32,
    pub y: i32,
    pub time: f32,
}

pub struct Game {
    pub snake: Snake,

    pub foods: Vec<Food>,

    width: i32,
    height: i32,

    pub is_game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(w: i32, h: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            foods: Vec::new(),
            width: w,
            height: h,
            is_game_over: false,
        }
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.is_game_over = false;
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.is_game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if let Some(d) = dir {
            if d == self.snake.head_direction().opposite() {
                return;
            }
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        for food in &self.foods {
            draw_block(
                [
                    0.67 * 1.0 / food.time,
                    0.48 * food.time,
                    0.34 * food.time,
                    food.time,
                ],
                food.x,
                food.y,
                con,
                g,
            );
        }

        let border_color = [0.0, 0.0, 0.0, 1.0];

        draw_rectange(border_color, 0, 0, self.width, 1, con, g);
        draw_rectange(border_color, 0, self.height - 1, self.width, 1, con, g);
        draw_rectange(border_color, 0, 0, 1, self.height, con, g);
        draw_rectange(border_color, self.width - 1, 0, 1, self.height, con, g);

        if self.is_game_over {
            draw_rectange(
                [0.91, 0.30, 0.24, 0.5],
                0,
                0,
                self.width,
                self.height,
                con,
                g,
            );
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        self.update_food_expired();
        self.update_food_life();

        if self.is_game_over {
            if self.waiting_time > 1.0 {
                self.restart();
            }
            return;
        }

        self.update_food();

        if self.waiting_time > MOVING_SPEED {
            self.update_snake(None);
        }
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_snake_alive() {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.is_game_over = true;
        }
        self.waiting_time = 0.0;
    }

    /// Remove expired foods
    pub fn update_food_expired(&mut self) {
        let mut new_foods: Vec<Food> = Vec::new();
        for food in &self.foods {
            if food.time >= 0.1 {
                let new_food = Food {
                    x: food.x,
                    y: food.y,
                    time: food.time,
                };
                new_foods.push(new_food);
            }
        }
        self.foods = new_foods;
    }

    /// Subtract food time parameter
    fn update_food_life(&mut self) {
        let mut new_foods = Vec::new();
        for food in &self.foods {
            let new_food = Food {
                x: food.x,
                y: food.y,
                time: food.time - FOOD_DECAY_SPEED,
            };
            new_foods.push(new_food);
        }
        self.foods = new_foods;
    }

    /// Check if the snake is eating any food
    fn check_eating(&mut self) {
        let mut point = (0, 0);
        for food in self.foods.iter() {
            if self.snake.head_position() == (food.x, food.y) {
                point = (food.x, food.y);
                self.snake.increase_length();
            }
        }
        self.remove_food(point.0, point.1);
    }

    fn remove_food(&mut self, x: i32, y: i32) {
        let mut new_foods: Vec<Food> = Vec::new();
        for food in self.foods.iter() {
            if food.x != x && food.y != y {
                let new_food = Food {
                    x: food.x,
                    y: food.y,
                    time: food.time,
                };
                new_foods.push(new_food);
            }
        }
        self.foods = new_foods;
    }

    /// Check if the snake is alive
    fn check_snake_alive(&self) -> bool {
        let mut newpos = self.snake.head_position();
        let dir = self.snake.head_direction();

        if dir == Direction::Left {
            newpos = (newpos.0 - 1, newpos.1);
        } else if dir == Direction::Right {
            newpos = (newpos.0 + 1, newpos.1);
        } else if dir == Direction::Up {
            newpos = (newpos.0, newpos.1 + 1);
        } else {
            newpos = (newpos.0, newpos.1 - 1);
        }
        if self.overlaps(newpos) || self.is_oob(newpos) {
            return false;
        }
        true
    }

    fn is_oob(&self, pos: (i32, i32)) -> bool {
        if pos.0 > 0 && pos.0 < self.width - 1 && pos.1 > 0 && pos.1 < self.height - 1 {
            return false;
        }
        true
    }

    fn overlaps(&self, pos: (i32, i32)) -> bool {
        for block in self.snake.get_body() {
            if block.x == pos.0 && block.y == pos.1 {
                return true;
            }
        }
        false
    }

    /// Add food at NUM_FOODS number of places
    fn update_food(&mut self) {
        let mut num_loops = NUM_FOODS - self.foods.len();
        let mut all_foods: Vec<Food> = Vec::new();
        let mut new_foods: Vec<Food> = Vec::new();

        for food in self.foods.iter() {
            let f = Food {
                x: food.x,
                y: food.y,
                time: food.time,
            };
            all_foods.push(f);
        }

        while num_loops > 0 {
            let mut thr = thread_rng();
            let newx: i32 = thr.gen();
            let newy: i32 = thr.gen();

            let new_food = Food {
                x: newx,
                y: newy,
                time: INIT_FOOD_LIFE,
            };
            new_foods.push(new_food);
            num_loops -= 1;
        }
        all_foods.append(&mut new_foods);
        self.foods = all_foods;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wall_collision() {
        let mut game = Game::new(20, 20);

        for _ in 0..14 {
            game.update(1.0);
            assert_eq!(game.is_game_over, false);
        }

        game.update(1.0);
        assert_eq!(game.is_game_over, true);
    }

    #[test]
    fn test_food_collect() {
        let mut game = Game::new(20, 20);

        game.foods.push(Food {
            x: 6,
            y: 2,
            time: 5.0,
        });

        for _ in 0..2 {
            let f = &game.foods[0];
            assert!(f.x == 6 && f.y == 2);

            game.update(1.0);
        }

        game.update(1.0);

        let f = &game.foods[0];
        assert!(!(f.x == 6 && f.y == 2));
    }

    #[test]
    fn test_food_regenerate() {
        let mut game = Game::new(20, 20);
        assert_eq!(game.foods.len(), 0);

        game.update(1.0);
        assert_eq!(game.foods.len(), NUM_FOODS);

        game.foods.pop();
        game.foods.pop();
        assert_eq!(game.foods.len(), NUM_FOODS - 2);

        game.update(1.0);
        assert_eq!(game.foods.len(), NUM_FOODS);
    }

    #[test]
    fn test_remove_expired_food() {
        let mut game = Game::new(20, 20);

        game.update(1.0);
        assert_eq!(game.foods.len(), NUM_FOODS);

        for food in game.foods.iter_mut() {
            food.time = 0.0;
        }

        game.update_food_expired();
        assert_eq!(game.foods.len(), 0);

        game.update(1.0);
        assert_eq!(game.foods.len(), NUM_FOODS);
    }

    #[test]
    fn test_decay_food() {
        let mut game = Game::new(20, 20);

        game.update(1.0);
        assert_eq!(game.foods.len(), NUM_FOODS);

        for food in game.foods.iter() {
            assert_eq!(food.time as f32, INIT_FOOD_LIFE);
        }

        game.update(1.0);

        for food in game.foods.iter() {
            assert_eq!(food.time as f32, INIT_FOOD_LIFE - FOOD_DECAY_SPEED);
        }

        game.update(1.0);
    }
}

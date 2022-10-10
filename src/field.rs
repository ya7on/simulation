use crate::bot::{Action, Bot};
use crate::{HEIGHT, WIDTH};
use rand::Rng;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::io::{BufWriter, StdoutLock, Write};

#[derive(Clone, Debug)]
pub enum Cell {
    Empty,
    Bot(Bot),
}

pub enum Color {
    None,
    Green,
    Yellow,
    Red,
}

#[derive(Debug)]
pub struct OtherBot {
    pub id: usize,
    pub energy: isize,
}

pub struct Field {
    cells: HashMap<usize, HashMap<usize, Cell>>,
    colors: HashMap<String, Color>,
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            colors: HashMap::new(),
            cells: (0..width)
                .map(|x| {
                    (
                        x,
                        (0..height)
                            .map(|y| {
                                (
                                    y,
                                    if rand::thread_rng().gen_range(0..100) < 5 {
                                        Cell::Bot(Bot::new())
                                    } else {
                                        Cell::Empty
                                    },
                                )
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }

    fn get_cell(&self, x: isize, y: isize) -> (i8, Option<OtherBot>) {
        if x >= 0 && y >= 0 && x < WIDTH as isize - 1 && y < HEIGHT as isize - 1 {
            match self
                .cells
                .get(&(x as usize))
                .unwrap_or(&HashMap::new())
                .get(&(y as usize))
                .unwrap_or(&Cell::Empty)
            {
                Cell::Bot(bot) => (
                    1,
                    Some(OtherBot {
                        id: bot.id as usize,
                        energy: bot.energy,
                    }),
                ),
                _ => (0, None),
            }
        } else {
            (-1, None)
        }
    }

    fn get_new_coordinates(x: usize, y: usize, angle: usize) -> (usize, usize) {
        match angle {
            0 => (x - 1, y - 1),
            1 => (x, y - 1),
            2 => (x + 1, y - 1),
            3 => (x - 1, y),
            4 => (x + 1, y),
            5 => (x - 1, y + 1),
            6 => (x, y + 1),
            7 => (x + 1, y + 1),
            _ => (x, y),
        }
    }

    pub fn step(&mut self) {
        self.colors.clear();

        let mut actions = Vec::<(usize, usize, usize, Action)>::new();
        for (x, line) in self.cells.iter() {
            for (y, cell) in line.iter() {
                if let Cell::Bot(bot) = cell {
                    let x = x.clone() as isize;
                    let y = y.clone() as isize;
                    let (angle, action) = bot.step(vec![
                        self.get_cell(x - 1, y - 1),
                        self.get_cell(x, y - 1),
                        self.get_cell(x + 1, y - 1),
                        self.get_cell(x - 1, y),
                        self.get_cell(x + 1, y),
                        self.get_cell(x - 1, y + 1),
                        self.get_cell(x, y + 1),
                        self.get_cell(x + 1, y + 1),
                    ]);
                    actions.push((
                        x.clone() as usize,
                        y.clone() as usize,
                        angle as usize,
                        action,
                    ));
                }
            }
        }

        for (x, y, angle, action) in actions {
            match action {
                Action::Move => {
                    let bot = self.cells.get(&x).unwrap().get(&y).unwrap().clone();
                    self.cells.get_mut(&x).unwrap().remove(&y);
                    self.cells
                        .get_mut(&x)
                        .unwrap()
                        .insert(y.clone(), Cell::Empty);
                    let (new_x, new_y) = Self::get_new_coordinates(x, y, angle);
                    self.cells.get_mut(&new_x).unwrap().remove(&new_y);
                    self.cells
                        .get_mut(&new_x)
                        .unwrap()
                        .insert(new_y.clone(), bot);
                    self.colors
                        .insert(format!("{}:{}", new_x, new_y), Color::Yellow);

                    if let Cell::Bot(bot) =
                        self.cells.get_mut(&new_x).unwrap().get_mut(&new_y).unwrap()
                    {
                        bot.energy -= 1;
                    }

                    if let Cell::Bot(bot) = self.cells.get(&new_x).unwrap().get(&new_y).unwrap() {
                        if bot.energy <= 0 {
                            self.cells.get_mut(&new_x).unwrap().remove(&new_y);
                            self.cells
                                .get_mut(&new_x)
                                .unwrap()
                                .insert(new_y.clone(), Cell::Empty);
                        }
                    }
                }
                Action::Reproduction => {
                    let bot = self.cells.get(&x).unwrap().get(&y).unwrap().clone();
                    let (new_x, new_y) = Self::get_new_coordinates(x, y, angle);
                    self.cells.get_mut(&new_x).unwrap().remove(&new_y);
                    if let Cell::Bot(bot) = bot {
                        self.cells
                            .get_mut(&new_x)
                            .unwrap()
                            .insert(new_y.clone(), Cell::Bot(bot.mutate()));
                    }
                    self.colors
                        .insert(format!("{}:{}", new_x, new_y), Color::Green);

                    if let Cell::Bot(bot) = self.cells.get_mut(&x).unwrap().get_mut(&y).unwrap() {
                        bot.energy -= 1;
                    };

                    if let Cell::Bot(bot) = self.cells.get(&x).unwrap().get(&y).unwrap() {
                        if bot.energy <= 1 {
                            self.cells.get_mut(&x).unwrap().remove(&y);
                            self.cells
                                .get_mut(&x)
                                .unwrap()
                                .insert(y.clone(), Cell::Empty);
                        }
                    }
                }
                Action::Attack => {
                    let bot = self.cells.get(&x).unwrap().get(&y).unwrap().clone();
                    self.cells.get_mut(&x).unwrap().remove(&y);
                    self.cells
                        .get_mut(&x)
                        .unwrap()
                        .insert(y.clone(), Cell::Empty);
                    let (new_x, new_y) = Self::get_new_coordinates(x, y, angle);
                    self.cells.get_mut(&new_x).unwrap().remove(&new_y);
                    self.cells
                        .get_mut(&new_x)
                        .unwrap()
                        .insert(new_y.clone(), bot);
                    self.colors
                        .insert(format!("{}:{}", new_x, new_y), Color::Red);

                    if let Cell::Bot(bot) =
                        self.cells.get_mut(&new_x).unwrap().get_mut(&new_y).unwrap()
                    {
                        bot.energy += 1;
                    }

                    if let Cell::Bot(bot) = self.cells.get(&x).unwrap().get(&y).unwrap() {
                        if bot.energy <= 0 {
                            self.cells.get_mut(&x).unwrap().remove(&y);
                            self.cells
                                .get_mut(&x)
                                .unwrap()
                                .insert(y.clone(), Cell::Empty);
                        }
                    }
                }
                Action::Heal => {
                    if let Cell::Bot(bot) = self.cells.get_mut(&x).unwrap().get_mut(&y).unwrap() {
                        bot.energy -= 0;
                    }
                    if let Cell::Bot(bot) = self.cells.get(&x).unwrap().get(&y).unwrap() {
                        if bot.energy <= 0 {
                            self.cells.get_mut(&x).unwrap().remove(&y);
                            self.cells
                                .get_mut(&x)
                                .unwrap()
                                .insert(y.clone(), Cell::Empty);
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) {
        let max_x = self.cells.len();
        for x in 0..max_x {
            let line = self.cells.get(&x).unwrap();
            let max_y = line.len();
            for y in 0..max_y {
                let cell = line.get(&y).unwrap();

                let draw_x = x as i32 * 5;
                let draw_y = y as i32 * 5;
                let draw_w = 5;
                let draw_h = 5;

                let rect = Rect::new(draw_x, draw_y, draw_w, draw_h);

                match cell {
                    Cell::Empty => {}
                    Cell::Bot(bot) => {
                        canvas.set_draw_color(sdl2::pixels::Color::RGB(
                            bot.color.0,
                            bot.color.1,
                            bot.color.2,
                        ));
                        canvas.fill_rect(rect).unwrap();
                        match self
                            .colors
                            .get(&format!("{}:{}", x, y))
                            .unwrap_or(&Color::None)
                        {
                            Color::None => {
                                canvas.set_draw_color(sdl2::pixels::Color::WHITE);
                            }
                            Color::Green => {
                                canvas.set_draw_color(sdl2::pixels::Color::GREEN);
                            }
                            Color::Yellow => {
                                canvas.set_draw_color(sdl2::pixels::Color::YELLOW);
                            }
                            Color::Red => {
                                canvas.set_draw_color(sdl2::pixels::Color::RED);
                            }
                        };
                        canvas.draw_rect(rect).unwrap();
                    }
                };
            }
        }
    }
}

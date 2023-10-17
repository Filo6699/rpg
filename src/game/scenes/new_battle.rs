#![allow(unused)]
use std::ops::Index;

use super::{stats::StatisticsScene, Scene, SharedData};
use crate::game::{battle::Entity, message_queue::MessageQueue, utils::get_full_size_rect};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use rand::{rngs::ThreadRng, Rng};
use ratatui::{
    prelude::*,
    text::Line,
    widgets::{Block, Paragraph},
};

const SCENE_ID: i32 = 69;

enum MoveDirection {
    Left,
    Right,
    Up,
}

#[derive(Copy, Clone)]
struct EntityPos {
    x: f64,
    y: f64,
    xa: f64,
    ya: f64,
}

impl From<EntityPos> for Rect {
    fn from(val: EntityPos) -> Self {
        Rect {
            x: val.x as u16,
            y: val.y as u16,
            width: 1,
            height: 1,
        }
    }
}

struct EntityState {
    pos: EntityPos,
    entity: Entity,
}

#[derive(Clone)]
enum Tile {
    Empty,
    Full,
}

struct GridSize {
    pub width: usize,
    pub height: usize,
}

pub struct NBattleScene {
    grid_size: GridSize,
    grid: Vec<Vec<Tile>>,

    player_state: EntityState,
    enemy_state: EntityState,

    pressed_keys: Vec<KeyCode>,
    rng_thread: ThreadRng,
    message_queue: MessageQueue,
}

impl NBattleScene {
    pub fn new(data: &SharedData) -> Self {
        let mut scene = NBattleScene {
            grid_size: GridSize {
                width: 120,
                height: 33,
            },
            grid: vec![vec![Tile::Empty; 120]; 33],

            player_state: EntityState {
                pos: EntityPos {
                    x: 3.0,
                    y: 20.0,
                    xa: 0.0,
                    ya: 0.0,
                },
                entity: data.player_data.to_entity(),
            },
            enemy_state: EntityState {
                pos: EntityPos {
                    x: 100.0,
                    y: 20.0,
                    xa: 0.0,
                    ya: 0.0,
                },
                entity: Entity::default(),
            },

            pressed_keys: vec![],
            rng_thread: rand::thread_rng(),
            message_queue: MessageQueue::default(),
        };
        scene.generate_grid();
        scene
    }

    fn set_cell(&mut self, x: usize, y: usize, tile: Tile) {
        if x >= self.grid_size.width || y >= self.grid_size.height {
            return;
        }
        self.grid[y][x] = tile;
    }

    fn generate_grid(&mut self) {
        let row = 27;
        for column in 0..self.grid_size.width {
            self.grid[row][column] = Tile::Full;
        }

        for x in 0..self.grid_size.width {
            for y in 0..7 {
                if self.rng_thread.gen_range(0..50) == 0 {
                    for offset in -3..4 {
                        let offset: i16 = offset;
                        if offset.is_negative() && offset.unsigned_abs() as usize > x {
                            continue;
                        }
                        let x = if offset.is_negative() {
                            x - offset.unsigned_abs() as usize
                        } else {
                            x + offset as usize
                        };
                        self.set_cell(x, y * 4, Tile::Full)
                    }
                }
            }
        }
    }

    fn render_grid(&self, frame: &mut crate::Frame) {
        let mut rows: Vec<Line> = vec![];
        for row in &self.grid {
            let mut rend = String::from("");
            for cell in row {
                let ch = match cell {
                    Tile::Empty => ' ',
                    Tile::Full => '■',
                };
                rend.push(ch);
            }
            rows.push(Line::from(rend))
        }
        let p = Paragraph::new(rows);
        let area = Rect {
            x: (frame.size().width - self.grid_size.width as u16) / 2,
            y: (frame.size().height - self.grid_size.height as u16) / 2,
            width: self.grid_size.width as u16,
            height: self.grid_size.height as u16,
        };
        frame.render_widget(p, area)
    }

    fn on_ground(&self, entity: &EntityState) -> bool {
        let pos = &entity.pos;
        if pos.ya >= 0.0 {
            if pos.y as usize == self.grid_size.height - 1 {
                return true;
            }
            if let Tile::Full = self.grid[(pos.y + 1.0) as usize][pos.x as usize] {
                return true;
            }
        }
        false
    }

    fn render_entities(&self, frame: &mut crate::Frame) {
        let block = Block::default().title("X").fg(Color::LightRed).bold();
        let area: Rect = self.enemy_state.pos.into();
        frame.render_widget(block, area);

        let block = Block::default().title("O").fg(Color::LightGreen).bold();
        let area: Rect = self.player_state.pos.into();
        frame.render_widget(block, area);
    }

    fn jump(&mut self) {
        if self.on_ground(&self.player_state) {
            self.player_state.pos.ya -= 0.2;
        } else {
            self.player_state.pos.ya -= 0.01;
        }
    }

    fn can_move(&self, direction: &MoveDirection, entity: &EntityState) -> bool {
        let pos = &entity.pos;
        match direction {
            MoveDirection::Left => {
                if pos.x as usize <= 0 {
                    return false;
                }
                if let Tile::Full = self.grid[pos.y as usize][(pos.x - 1.0) as usize] {
                    return false;
                }
                true
            }
            MoveDirection::Right => {
                if pos.x as usize >= self.grid_size.width - 1 {
                    return false;
                }
                if let Tile::Full = self.grid[pos.y as usize][(pos.x + 1.0) as usize] {
                    return false;
                }
                true
            }
            MoveDirection::Up => {
                if pos.y as usize <= 0 {
                    return false;
                }
                if let Tile::Full = self.grid[(pos.y - 1.0) as usize][pos.x as usize] {
                    return false;
                }
                true
            }
        }
    }

    fn step(&mut self, direction: MoveDirection) {
        if !self.can_move(&direction, &self.player_state) {
            return;
        }
        let mut xaa: f64 = match direction {
            MoveDirection::Left => -0.1,
            MoveDirection::Right => 0.1,
            _ => panic!("Wrong move direction you idiot"),
        };
        if self.on_ground(&self.player_state) {
            xaa *= 6.0;
        }
        self.player_state.pos.xa += xaa;
    }

    fn tick(&mut self, data: &mut SharedData) {
        if self.pressed_keys.contains(&KeyCode::Char('d')) {
            self.step(MoveDirection::Right)
        }
        if self.pressed_keys.contains(&KeyCode::Char('a')) {
            self.step(MoveDirection::Left)
        }
        if self.pressed_keys.contains(&KeyCode::Char(' ')) && self.player_state.pos.ya <= 0.0 {
            self.jump()
        }

        self.player_state.pos.xa = self.player_state.pos.xa.clamp(-1.0, 1.0);
        self.player_state.pos.ya = self.player_state.pos.ya.clamp(-1.0, 1.0);

        if self.player_state.pos.xa > 0.0 {
            if self.can_move(&MoveDirection::Right, &self.player_state) {
                self.player_state.pos.x += self.player_state.pos.xa;
            } else {
                self.player_state.pos.xa = 0.0;
            }
        }
        if self.player_state.pos.xa < 0.0 {
            if self.can_move(&MoveDirection::Left, &self.player_state) {
                self.player_state.pos.x += self.player_state.pos.xa;
            } else {
                self.player_state.pos.xa = 0.0;
            }
        }
        if self.player_state.pos.ya < 0.0 {
            if self.can_move(&MoveDirection::Up, &self.player_state) {
                self.player_state.pos.y += self.player_state.pos.ya;
            } else {
                self.player_state.pos.ya = 0.0;
            }
        }
        self.player_state.pos.y += self.player_state.pos.ya;

        self.enemy_state.pos.x += self.enemy_state.pos.xa;
        self.enemy_state.pos.y += self.enemy_state.pos.ya;

        self.player_state.pos.x = self
            .player_state
            .pos
            .x
            .clamp(0.0, (self.grid_size.width - 1) as f64);
        self.player_state.pos.y = self
            .player_state
            .pos
            .y
            .clamp(0.0, (self.grid_size.height - 1) as f64);

        self.enemy_state.pos.x = self
            .enemy_state
            .pos
            .x
            .clamp(0.0, (self.grid_size.width - 1) as f64);
        self.enemy_state.pos.y = self
            .enemy_state
            .pos
            .y
            .clamp(0.0, (self.grid_size.height - 1) as f64);

        if !self.on_ground(&self.player_state) {
            self.player_state.pos.ya += 0.02;
            self.player_state.pos.xa *= 0.9;
        } else {
            self.player_state.pos.xa /= 20.0;
            if self.player_state.pos.ya > 0.0 {
                self.player_state.pos.ya = 0.0;
                self.player_state.pos.y = (self.player_state.pos.y as u16) as f64 + 0.950;
            }
        }
        if !self.on_ground(&self.enemy_state) {
            self.enemy_state.pos.ya += 0.04;
        } else {
            self.enemy_state.pos.xa /= 20.0;
            self.enemy_state.pos.ya = 0.0;
        }
    }

    fn key_down(&mut self, key: KeyCode) {
        self.pressed_keys.push(key);
    }

    fn key_up(&mut self, key: KeyCode) {
        self.pressed_keys.retain(|&x| x != key)
    }

    pub fn scene_id() -> i32 {
        SCENE_ID
    }
}

impl Scene for NBattleScene {
    fn scene_id(&self) -> i32 {
        SCENE_ID
    }

    fn set_message_queue(&mut self, queue: MessageQueue) {
        self.message_queue = queue;
    }

    fn handle_input(&mut self, key: KeyEvent, data: &mut SharedData) {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Enter => data.current_scene = StatisticsScene::scene_id(),
                KeyCode::Char(pressed_key) => self.key_down(key.code),
                _ => (),
            }
        } else if let KeyCode::Char(ch) = key.code {
            self.key_up(key.code)
        }
    }

    fn render(&self, frame: &mut crate::Frame, _: &SharedData) {
        if frame.size().width < self.grid_size.width as u16
            || frame.size().height < self.grid_size.height as u16
        {
            let p = Paragraph::new("Terminal is too small");
            let area = get_full_size_rect(frame);
            frame.render_widget(p, area);
            return;
        }
        self.render_grid(frame);
        self.render_entities(frame);
    }

    fn update(&mut self, data: &mut SharedData) {
        self.tick(data);
    }
}

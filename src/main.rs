use noise::{NoiseFn, Perlin};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    prelude::*,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::*,
};
use std::collections::HashSet;
use std::io::{self, stdout};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc, Mutex,
};
use std::thread;
use std::time::Duration;

const MAX_INVENTORY: u32 = 5;
const MAX_LOGS: usize = 10;

fn log_event(logs: &Arc<Mutex<Vec<String>>>, msg: &str) {
    let mut logs = logs.lock().unwrap();
    logs.push(msg.to_string());
    if logs.len() > MAX_LOGS {
        logs.remove(0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RobotType {
    Explorer,
    Miner,
}

struct Map {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
    base_x: usize,
    base_y: usize,
}

impl Map {
    fn new(width: usize, height: usize, _seed: u32) -> Self {
        let mut data = vec![vec!['.'; width]; height];
        let base_x = width / 2;
        let base_y = height / 2;
        data[base_y][base_x] = 'S';
        Self {
            width,
            height,
            data,
            base_x,
            base_y,
        }
    }

    pub fn clone_map(&self) -> Map {
        Map {
            width: self.width,
            height: self.height,
            data: self.data.clone(),
            base_x: self.base_x,
            base_y: self.base_y,
        }
    }
}

#[derive(Clone)]
struct Robot {
    id: usize,
    x: usize,
    y: usize,
    robot_type: RobotType,
    inventory: u32,                 // Nombre de ressources collectées
    target: Option<(usize, usize)>, // Pour les collecteurs : la cible (ressource)
    paused: bool,                   // Indique si le robot Miner (id 2) est en pause
}

impl Robot {
    fn new(id: usize, x: usize, y: usize, robot_type: RobotType) -> Self {
        Self {
            id,
            x,
            y,
            robot_type,
            inventory: 0,
            target: None,
            paused: false,
        }
    }

    fn move_randomly(&mut self, _width: usize, _height: usize, _map: &Map) {
    }

    fn move_towards(&mut self, _target: (usize, usize), _map: &Map) {
    }

    fn perform_task(
        &mut self,
        _map: &mut Map,
        _reported_resources: &Arc<Mutex<HashSet<(usize, usize)>>>,
        _logs: &Arc<Mutex<Vec<String>>>,
    ) {
    }
}

fn render_ui(
    _rx: mpsc::Receiver<Map>,
    _robots: Arc<Mutex<Vec<Robot>>>,
    running: Arc<AtomicBool>,
    _logs: Arc<Mutex<Vec<String>>>,
) -> io::Result<()> {
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }
    terminal.clear()?;
    Ok(())
}

fn main() -> io::Result<()> {
    println!("Hello, world!");
    Ok(())
}

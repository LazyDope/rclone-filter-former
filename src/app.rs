use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader},
    path::PathBuf,
};

use clap::Parser;
use ratatui::DefaultTerminal;
use ropey::Rope;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct AppInput {
    path: PathBuf,
}

pub struct App {
    path: PathBuf,
    buffer: Rope,
    mode: AppMode,
    line: usize,
}

enum AppMode {
    View,
    Edit,
    ManualTest,
    DirTest,
}

impl App {
    pub fn init() -> Result<Self, io::Error> {
        let input = AppInput::parse();
        let file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(&input.path)?;
        let buffer = Rope::from_reader(BufReader::new(file))?;
        Ok(App {
            path: input.path,
            buffer,
            mode: AppMode::View,
            line: 0,
        })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), io::Error> {
        self.buffer.insert(0, "+ ~/test/*\n");
        let file = File::create(&self.path)?;
        self.buffer.write_to(file)
    }
}

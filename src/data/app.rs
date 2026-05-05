use std::{
    io::BufReader,
    path::Path,
    time::{Duration, Instant},
};

use lofty::{AudioFile, Probe};
use ratatui::widgets::ListState;
use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct Song {
    pub title: String,
    pub path: String,
}

fn load_songs_form_dir(dir: &str) -> Vec<Song> {
    let mut songs = Vec::new();

    let entries = std::fs::read_dir(dir).expect("读取目录失败");
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && is_song(&path) {
            let title = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let path = path.to_string_lossy().to_string();
            songs.push(Song::new(title, path));
        }
    }
    songs
}

fn is_song(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => {
            matches!(ext.to_lowercase().as_str(), "mp3")
        }
        None => false,
    }
}

impl Song {
    pub fn new(title: String, path: String) -> Self {
        Self { title, path }
    }
}

pub struct PlayerStae {
    stream: OutputStream,
    handle: OutputStreamHandle,
    sink: Sink,
    current: Option<String>,
    duration_song: Option<String>,
    pub duration: Option<Duration>,
    start_time: Option<Instant>,
    pased_at: Option<Duration>,
    pub is_playing: bool,
}

fn get_duration(path: &str) -> Option<Duration> {
    let tagged_file = Probe::open(Path::new(path)).ok()?.read().ok()?;

    let properties = tagged_file.properties();

    Some(properties.duration())
}

impl PlayerStae {
    pub fn new() -> Self {
        let (steam, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        Self {
            stream: steam,
            handle,
            sink,
            current: None,
            duration_song: None,
            duration: None,
            start_time: None,
            pased_at: None,
            is_playing: false,
        }
    }

    pub fn play(&mut self, path: String) {
        if self.current.as_deref() == Some(&path) {
            self.sink.play();
            return;
        }
        self.current = Some(path.clone());
        self.sink.stop();
        self.sink = Sink::try_new(&self.handle).unwrap();
        let file = std::fs::File::open(path.clone()).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        self.duration = get_duration(&path);
        self.sink.append(source);
        self.sink.play();
        self.is_playing = true;
        self.start_time = Some(Instant::now());
        self.pased_at = None;
        self.duration_song = Some(path);
    }

    pub fn pause(&mut self) {
        self.sink.pause();
        self.is_playing = false;
        if let Some(start_time) = self.start_time {
            self.pased_at = Some(start_time.elapsed())
        }
    }

    pub fn resume(&mut self) {
        self.sink.play();
        self.is_playing = true;
        if let Some(paused) = self.pased_at {
            self.start_time = Some(Instant::now() - paused)
        }
    }

    pub fn is_finished(&self) -> bool {
        self.sink.empty() && self.is_playing
    }
}

impl Default for PlayerStae {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq)]
pub enum Tab {
    Home,
    Settings,
}

impl Tab {
    pub fn all() -> Vec<Tab> {
        vec![Tab::Home, Tab::Settings]
    }

    pub fn next(&mut self) {
        *self = match self {
            Tab::Home => Tab::Settings,
            Tab::Settings => Tab::Home,
        };
    }

    pub fn title(&self) -> &str {
        match self {
            Tab::Home => "首页",
            Tab::Settings => "设置",
        }
    }
}

pub struct App {
    pub tab: Tab,
    pub songs: Vec<Song>,
    pub list_state: ListState,
    pub playing_song: Option<usize>,
    pub should_quit: bool,
    pub player: PlayerStae,
}

impl App {
    pub fn new() -> Self {
        let songs = load_songs_form_dir("/home/sheneternity/music/");
        Self {
            tab: Tab::Home,
            songs,
            list_state: ListState::default(),
            playing_song: None,
            should_quit: false,
            player: PlayerStae::new(),
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn next_tab(&mut self) {
        self.tab.next();
    }

    pub fn select_next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.songs.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn select_prev(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.songs.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn play_or_pause(&mut self) {
        if let Some(i) = self.list_state.selected() {
            if self.playing_song == Some(i) {
                if self.player.sink.is_paused() {
                    self.player.resume();
                } else {
                    self.player.pause();
                }
            } else {
                self.playing_song = Some(i);
                self.player.play(self.songs[i].path.clone());
            }
        }
    }

    pub fn get_pragress(&self) -> f64 {
        if let (Some(start), Some(total)) = (self.player.start_time, self.player.duration) {
            let elapsed = if self.player.sink.is_paused() {
                self.player.pased_at.unwrap_or_default()
            } else {
                start.elapsed()
            };

            let progress = elapsed.as_secs_f64() / total.as_secs_f64();

            progress.clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    pub fn auto_next(&mut self) {
        if self.playing_song.is_some() && self.player.is_finished() {
            let current = self.list_state.selected().unwrap_or(0);
            let next = if current >= self.songs.len() - 1 {
                0
            } else {
                current + 1
            };
            self.list_state.select(Some(next));
            self.playing_song = Some(next);
            self.player.play(self.songs[next].path.clone());
        }
    }

    pub fn got() {}
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

use std::{thread, time};
use std::path::Path;
use std::io::{self, Write};

extern crate notify_rust;
extern crate ears;

use ears::{Sound, AudioController};
use notify_rust::Notification;

const BEEP_FILE: &'static str = "/usr/share/sounds/freedesktop/stereo/complete.oga";
const HELP: &'static str = r#"
NAME
    prod - a simple Pomodor timer

DESCRIPTION
    List information about the FILE(s), or the current directory

COMMANDS
    help
        show this help
    work
        start working on a task. Default value is 25 minutes
    break
        take a break. Default value is 5 minutes
"#;


enum Cmd {
    Work,
    Break,
    Unknown,
    Test,
    Help,
}

impl Cmd {
    fn get_time(&self) -> u64 {
        match *self {
            Cmd::Work => 25 * 60,
            Cmd::Break => 5 * 60,
            Cmd::Test => 3,
            _ => 0,
        }
    }

    fn exec(&self) {
        match *self {
            Cmd::Work => {
                wait(self.get_time());
                sound_beep();
                notify("Good job!");
            },
            Cmd::Break => {
                wait(self.get_time());
                sound_beep();
                notify("Break is over.");
            },
            Cmd::Test => {
                wait(self.get_time());
                sound_beep();
                notify("Test");
            },
            Cmd::Help => repl_print(HELP),
            Cmd::Unknown => repl_println("Unknown command"),
        }

    }

    fn parse(line: &String) -> Option<Self> {
        match line.to_lowercase().trim() {
            "help" => Some(Cmd::Help),
            "work" => Some(Cmd::Work),
            "break" => Some(Cmd::Break),
            "test" => Some(Cmd::Test),
            _ => Some(Cmd::Unknown),
        }
    }

}

fn repl_print(msg: &str) {
    io::stdout().write(msg.as_bytes()).expect("Unable to write to stdout");
    io::stdout().flush().expect("Unable to flush to stdout");
}

fn repl_println(msg: &str) {
    repl_print(msg);
    repl_print("\n");
}

fn wait(seconds: u64) {
    for i in (1..seconds).rev() {
        let line = &*format!("\rRemaining time: {:02}:{:02}", i / 60, i % 60);
        repl_print(line);
        thread::sleep(time::Duration::from_secs(1));
    }
    repl_print("\n");
}

fn notify(msg: &str) {
    Notification::new()
        .summary("prod")
        .body(msg)
        .show().unwrap();
}

// TODO: this crate is strangely increasing CPU usage. I should use another one...
fn play_file(file: &str) {
    let mut snd = Sound::new(file).unwrap();
    snd.play();
    while snd.is_playing() {}
}

fn sound_beep() {
    if Path::new(BEEP_FILE).is_file() {
        thread::spawn(move || {
            play_file(BEEP_FILE);
        });
    }
    
}

fn main() {
    loop {
        let mut cmd = String::new();
        repl_print(">>> ");
        match io::stdin().read_line(&mut cmd) {
            Ok(i) if i > 0 => {
                match Cmd::parse(&cmd) {
                    Some(c) => c.exec(),
                    None => continue,
                }
            },
            Ok(..) => return,
            Err(error) => repl_println(&*format!("error parsing command: {}", error))
        }
    }
}

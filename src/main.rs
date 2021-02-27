extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session};

struct Ferris;

impl Ferris {
    fn new() -> Ferris {
        Ferris {}
    }
    // comment lines in buffer out
    fn commentline(&self, nums: Vec<i64>) -> i64 {
        nums.iter().sum::<i64>()
    }
}

enum Messages {
    CommentLine,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "commentline" => Messages::CommentLine,
            _ => Messages::Unknown(event),
        }
    }
}

struct EventHandler {
    nvim: Neovim,
    ferris: Ferris,
}

impl EventHandler {
    fn new() -> EventHandler {
        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let ferris = Ferris::new();

        EventHandler { nvim, ferris }
    }

    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, values) in receiver {
            match Messages::from(event) {
                // Handle 'Add'
                Messages::CommentLine => {
                    let nums = values
                        .iter()
                        .map(|v| v.as_i64().unwrap())
                        .collect::<Vec<i64>>();

                    let sum = self.ferris.commentline(nums);
                    self.nvim
                        .command(&format!("echo \"Sum: {}\"", sum.to_string()))
                        .unwrap();
                }
                // Handle anything else
                Messages::Unknown(event) => {
                    self.nvim
                        .command(&format!("echo \"Unknown command: {}\"", event))
                        .unwrap();
                }
            }
        }
    }
}

fn main() {
    let mut event_handler = EventHandler::new();
    event_handler.recv();
}

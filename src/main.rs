extern crate neovim_lib;
use neovim_lib::{Neovim,NeovimApi,Session};

fn main() {
    println!("Hello, world!");
    let mut event_handler = EventHandler::new();
    event_handler.handle_events();
}

enum Messages {
    Comment,
    Unknown(String),
}
impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "comment" => Messages::Comment,
            _ => Messages::Unknown(event),
        }
    }
}


struct Ferris;
impl Ferris {
    fn new() -> Ferris {
        Ferris {}
        }
        // Comment a line out. 
        fn comment(&self, p: i64) -> i64 {
            p 
        }
}

pub struct EventHandler {
    nvim: Neovim,
    ferris: Ferris,
}

impl EventHandler {
    fn new() -> EventHandler {
        let mut session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let ferris = Ferris::new();
    EventHandler {nvim, ferris}
    }

    pub fn handle_events(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();
        for(event,values) in receiver {
            match Messages::from(event) {

                Messages::Comment => {
                    let mut nums = values.iter();
                let p = nums.next().unwrap().as_i64().unwrap();

                let product = self.ferris.comment(p);
                self.nvim // <-- Echo response to Nvim
                    .command(&format!("echo \"Product: {}\"", product.to_string()))
                    .unwrap();
                }

                Messages::Unknown(event) => {
                    self.nvim // <-- Echo unknown command
                    .command(&format!("echo \"Unknown command: {}\"", event))
                    .unwrap();
                }
            }
        }
        // todo
    }
}



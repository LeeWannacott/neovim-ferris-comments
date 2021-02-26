extern crate neovim_lib;


use neovim_lib::{Neovim,NeovimApi,Session};

fn main() {
    println!("Hello, world!");
    let mut event_handler = EventHandler::new();
    event_handler.recv();
}

struct Commenter;

impl Commenter {
    fn new() -> Commenter {
        Commenter {}
        }
        // Comment a line out. 
        fn comment(&self, p: i64, q: i64) -> i64 {
            p * q
        }
}

struct EventHandler {
    nvim: Neovim,
    commenter: Commenter,
}

impl EventHandler {
    fn new() -> EventHandler {
        let mut session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let commenter = Commenter::new();
    
    EventHandler {nvim, commenter}
    }
        // handle eventes
    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();
        for(event,values) in receiver {
            match Messages::from(event) {
                Messages::CommentLine => {
                    let mut nums = values.iter();
                let p = nums.next().unwrap().as_i64().unwrap();
                let q = nums.next().unwrap().as_i64().unwrap();

                let product = self.commenter.comment(q,p);
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

enum Messages {
    CommentLine,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "CommentLine" => Messages::CommentLine,
            _ => Messages::Unknown(event),
        }
    }
}

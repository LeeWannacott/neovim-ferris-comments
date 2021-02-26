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
        fn add(&self, nums: Vec<i64>) -> i64 {
            nums.iter().sum::<i64>()
          }
          
          // Multiply two numbers
          fn multiply(&self, p: i64, q: i64) -> i64 {
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
                Messages::Add => {
                    let nums = values
                    .iter()
                    .map(|v| v.as_i64().unwrap())
                    .collect::<Vec<i64>>();

                let sum = self.commenter.add(nums);
                self.nvim // <-- Echo response to Nvim
                    .command(&format!("echo \"Sum: {}\"", sum.to_string()))
                    .unwrap();
                    
                }
                Messages::Multiply => {
                    let mut nums = values.iter();
                let p = nums.next().unwrap().as_i64().unwrap();
                let q = nums.next().unwrap().as_i64().unwrap();

                let product = self.commenter.multiply(p, q);
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
    Add,
    Multiply,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            "add" => Messages::Add,
            "multiply" => Messages::Multiply,
            _ => Messages::Unknown(event),
        }
    }
}

use std;

use ansi_term::Colour::{Green, Red};
use rustyline;

use device;


fn print(result: Result<String, String>) {
    // TODO handle IO errors
    // TODO handle command errors
    println!("{:?}", result);
}

pub fn run_loop(device_name: &str) {
    let (send, recv) = match device::open(device_name) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", Red.paint(e));
            return;
        }
    };
    let prompt = Green.paint(">> at").to_string();
    let mut rl = rustyline::Editor::<()>::new();
    rl.set_history_max_len(1024);

    let eval = |cmd: String| {
        // TODO there must be a better way
        let at = String::from("at") + &cmd;
        send.send(at);
        // TODO maybe display spinner (poll with recv_timeout)
        return recv.recv().unwrap();
    };

    loop {
        match rl.readline(&prompt) {
            Ok(line) => {rl.add_history_entry(&line);
                         print(eval(line))},
            Err(_)   => break,
        }
    }
}

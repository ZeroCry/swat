use ansi_term::Colour::{Green, Red};
use rustyline;

use device;


fn print(result: Result<String, String>) {
    match result {
        Ok(msg) => println!("{}", msg),
        Err(msg) => eprintln!("{}", Red.paint(msg))
    }
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

    let eval = |cmd: &str| {
        send.send(["at", cmd].join("")).unwrap();
        // TODO maybe display spinner (poll with recv_timeout)
        return recv.recv().unwrap();
    };

    loop {
        match rl.readline(&prompt) {
            Ok(line) => {rl.add_history_entry(&line);
                         print(eval(&line))},
            Err(_)   => break,
        }
    }
}

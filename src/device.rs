use std::io::{self, BufRead};
use std::thread;

use std::sync::mpsc::{channel, Receiver, Sender};

use serial;

type CmdChannel = (Sender<String>, Receiver<Result<String, String>>);

fn command_worker(port: serial::SystemPort) -> CmdChannel {
    let (send_cmd, receive_cmd) = channel();
    let (send_result, receive_result) = channel();

    thread::spawn(move || {
        let lines = io::BufReader::new(port).lines();
        for cmd in receive_cmd.iter() {
            // TODO run command
            // send string + \r\n
            // reader.read_line until "OK" or "ERROR"
            send_result.send(Ok(String::from("OK")));
        }
    });

    return (send_cmd, receive_result);
}

fn is_not_final_string(x: &str) -> bool {
    x != "OK" && x != "ERROR"
}

pub fn open(name: &str ) -> Result<CmdChannel, String> {
    match serial::open(name) {
        Ok(port) => Ok(command_worker(port)),
        Err(e) => Err(format!("Could not open {}: {}", name, e))
    }
}

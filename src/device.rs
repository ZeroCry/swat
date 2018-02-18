use std::io::{self, BufRead, Write};
use std::thread;

use std::sync::mpsc::{channel, Receiver, Sender};

use serial;

type CmdChannel = (Sender<String>, Receiver<Result<String, String>>);

fn command_worker(mut port: serial::SystemPort) -> CmdChannel {
    let (send_cmd, receive_cmd) = channel::<String>();
    let (send_result, receive_result) = channel();

    thread::spawn(move || {
        for cmd in receive_cmd.iter() {
            send_result.send(match port.write(&(cmd + "\r\n").into_bytes()) {
                Ok(_) => read_response(&mut port),
                Err(e) => Err(format!("{} while writing", e))
            }).unwrap();
        }
    });

    return (send_cmd, receive_result);
}

fn is_final_string(x: &str) -> bool {
    x == "OK" || x == "ERROR"
}

pub fn open(name: &str ) -> Result<CmdChannel, String> {
    match serial::open(name) {
        Ok(port) => Ok(command_worker(port)),
        Err(e) => Err(format!("Could not open {}: {}", name, e))
    }
}

fn read_response(port: &mut serial::SystemPort) -> Result<String, String> {
    let mut res: Vec<String> = Vec::new();
    for l in io::BufReader::new(port.by_ref()).lines() {
        match l {
            Ok(s) => {
                let is_final = is_final_string(&s);
                res.push(s);
                if is_final {
                    // TODO check for ERROR and return Err(...) instead?
                    return Ok(res.join("\n"));
                }
            },
            Err(e) => return Err(format!("{} while reading", e))
        }
    }
    Err(String::from("EOF while reading response."))
}

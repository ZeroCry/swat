extern crate ansi_term;
#[macro_use]
extern crate clap;
extern crate rustyline;
extern crate serial;

use clap::{Arg, App, SubCommand};

mod device;
mod repl;

fn main() {
    let matches = App::new(crate_name!())
                          .version(crate_version!())
                          .about(crate_description!())
                          .arg(Arg::with_name("device")
                               .short("d")
                               .long("device")
                               .value_name("DEV")
                               .help("Sets the serial device to use")
                               .takes_value(true)
                               .required(true))
                          .subcommand(SubCommand::with_name("repl")
                                      .about("Starts an interactive session."))
                          //.subcommand(SubCommand::with_name("run")
                          //            .about("runs a single command")
                          //            .arg(Arg::with_name("cmd")
                          //                .required(true)))
                          .get_matches();

    let device = matches.value_of("device").unwrap();

    // TODO --retry functionality

    match matches.subcommand() {
        ("repl", Some(_)) => {
            // Repl has no matches (sub-args)
            repl::run_loop(device);
        },
        ("run", Some(run_matches)) => {
            // TODO implement run
            let cmd = run_matches.value_of("cmd").unwrap();
            println!("Run {} on {}", cmd, device);
        },
        ("", None) => println!("No subcommand was used.\n\n{}",
                               matches.usage()),
        _ => unreachable!(),
    }
}

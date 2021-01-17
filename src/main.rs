use log::*;
use simplelog as sl;
use structopt::StructOpt;

mod bridge;  // bridge driver
mod emit;    // emit driver
mod error;   // errors
//mod net;     // networking
mod opt;     // CLI parsing
mod packing; // lan packet (de)serialization
mod recv;    // recv driver
mod util;    // utilities

fn main() {
    let opt = opt::Opt::from_args();

    sl::TermLogger::init(
        opt.log_level,
        Default::default(),
        sl::TerminalMode::Mixed,
    ).or_else(|_| sl::SimpleLogger::init(
        opt.log_level,
        Default::default(),
    )).expect("Unable to initialize logging");

    match opt.cmd {
        opt::Subcommand::Bridge {
        } => todo!(),
        opt::Subcommand::Cache {
            file,
            no_update,
            show,
        } => {
            info!("performing roll call");
            //let devices = net::roll_call(opt.timeout, |_| ()).unwrap();

            //print!("{:?}", devices);
        },
        opt::Subcommand::Emit {
        } => todo!(),
        opt::Subcommand::Recv {
        } => todo!(),
    }
}

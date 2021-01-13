use simplelog as sl;
use structopt::StructOpt;

mod bridge;  // bridge driver
mod error;   // errors
mod opt;     // CLI parsing
mod recv;    // recv driver
mod emit;    // emit driver

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
            no_update,
            show,
        } => todo!(),
        opt::Subcommand::Emit {
        } => todo!(),
        opt::Subcommand::Recv {
        } => todo!(),
    }
}

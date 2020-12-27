use simplelog as sl;
use structopt::StructOpt;

fn parse_log_level(name: &str) -> sl::LevelFilter {
    match name {
        "off" | "Off" | "OFF" => sl::LevelFilter::Off,
        "error" | "Error" | "ERROR" => sl::LevelFilter::Error,
        "warn" | "Warn" | "WARN" => sl::LevelFilter::Warn,
        "info" | "Info" | "INFO" => sl::LevelFilter::Info,
        "debug" | "Debug" | "DEBUG" => sl::LevelFilter::Debug,
        "trace" | "Trace" | "TRACE" => sl::LevelFilter::Trace,
        _ => panic!("Unexpected value \"{}\" for log_level", name),
    }
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Opt {
    /// May be off, error, warn, info, debug, trace
    #[structopt(
        short,
        long,
        default_value = "info", // default provided here bc structopt/clap
                                // provides additional auto documentation
        parse(from_str = parse_log_level),
    )]
    pub log_level: sl::LevelFilter,

    #[structopt(subcommand)]
    pub cmd: Subcommand,
}

#[derive(StructOpt)]
pub enum Subcommand {
    /// Runs this program in bridge mode (TBI)
    ///
    /// Bridge mode acts as a translator between lifx's lan protocol and
    /// JSON. It receives requests and publishes responses on the given (or
    /// default) socket, and translates according to the given (or default)
    /// dictionaries.
    Bridge {
    },

    /// Refreshes the lan device cache (TBI)
    Cache {
        /// Do not update the cache (this option without show is a no-op)
        #[structopt(short, long)]
        no_update: bool,

        /// Display the location and current contents of the device cache
        ///
        /// When omitted, the program only displays new updates.
        #[structopt(short, long)]
        show: bool,
    },

    /// Sends a lifx packet to the network, and displays the response (if
    /// requested) (TBI)
    Emit {
    },

    /// Listens for lifx packets on the local network and displays them (TBI)
    Recv {
    },
}

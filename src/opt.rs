use std::{
    env,
    path::PathBuf,
};
use lazy_static::*;
use simplelog as sl;
use structopt::StructOpt;

lazy_static! {
    static ref TEMP_FILE_DEFAULT: String = temp_file_default();
}

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

fn temp_file_default() -> String {
    let prefix = env::temp_dir().into_os_string();
    let ver = env!("CARGO_PKG_VERSION_MAJOR");
    format!("{}/lifx-tool-cache.v{}.json", prefix.to_string_lossy(), ver)
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

    /// Number of milliseconds to wait for replies
    #[structopt(
        short,
        long,
        default_value = "5000",
    )]
    pub timeout: u64,

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
        /// Specify which file to use as cache database
        #[structopt(
            short,
            long,
            default_value = &TEMP_FILE_DEFAULT,
            parse(from_os_str),
        )]
        file: PathBuf,

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
    /// requested)
    Emit {
    },

    /// Listens for lifx packets on the local network and displays them (TBI)
    Recv {
    },
}

#[derive(StructOpt)]
pub enum EmitMessage {
}

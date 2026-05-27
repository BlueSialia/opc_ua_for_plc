//! Runtime CLI.
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "opc_ua_for_plc-runtime")]
#[command(about = "OPC UA gateway runtime for PLC drivers", long_about = None)]
pub struct Cli {
    /// Runtime configuration file (TOML or YAML).
    #[arg(short, long, value_name = "FILE", default_value = "config/config.toml")]
    pub config: PathBuf,
}

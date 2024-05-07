use clap::{builder::{styling::AnsiColor, Styles}, Parser};

use crate::settings::{ServerSettings, ServiceSettings};

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().bold())
    .usage(AnsiColor::Green.on_default().bold())
    .literal(AnsiColor::Cyan.on_default().bold())
    .placeholder(AnsiColor::Cyan.on_default());

#[derive(Debug, Parser)]
#[command(styles=STYLES)]
pub struct Cli {
    #[arg(short, long, default_value_t = 3000)]
    /// Specifies the port on which the server should listen.
    pub port: u16,

    #[arg(short, long, default_value_t = String::from("./"))]
    /// Sets the directory from which the server will serve files.
    pub directory: String,

    #[arg(short, long)]
    /// Specifies one or more suffixes to append to requested files when necessary.
    pub suffix: Vec<String>,

    #[arg(long, short)]
    /// Specifies the allowed origin for Cross-Origin Resource Sharing (CORS) requests.
    pub allow_origin: Vec<String>,

    #[arg(long, alias = "dc", short = 'c', action, default_value_t = false)]
    /// Disables compression for served files, which can reduce CPU usage but may result in larger file transfers.
    pub disable_compression: bool,

    #[arg(long, short)]
    /// Sets a fallback file to serve when a requested file is not found.
    pub fallback_file: Option<String>,

    /// Prevents automatic appending of "index.html" to directory paths.
    #[arg(long, short = 'n', action, default_value_t = false)]
    pub no_auto_index: bool,
}

impl Cli {
    pub fn into_settings(self) -> (ServerSettings, ServiceSettings) {
        (
            ServerSettings { port: self.port },
            ServiceSettings {
                directory: self.directory,
                suffixes: self.suffix,
                allowed_origins: self.allow_origin,
                compression: !self.disable_compression,
                fallback_file: self.fallback_file,
                append_index_html_on_directories: !self.no_auto_index,
            },
        )
    }
}

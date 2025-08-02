use clap::Parser;

/// A small version of grep that searchs a file.
#[derive(Parser)]
#[command(version)]
pub struct Args {
    /// String to search for
    pub search_string: String,

    /// File to search for `search_string`
    pub file_name: String,

    /// print matches in color (red)
    #[arg(default_value_t = true, long)]
    pub do_print_color: bool,
}
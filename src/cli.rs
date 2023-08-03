use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to the executable to run.
    #[arg(default_value = "")]
    pub executable: String,
}

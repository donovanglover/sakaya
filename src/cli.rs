use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    /// Path to the executable to run.
    #[arg(default_value = "")]
    pub executable: String,
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    nixos_path: String,

    #[arg(long)]
    system_path: String,

    #[arg(long)]
    config: String,

    #[arg(long)]
    config_file: String,

    #[arg(long)]
    flake: String,

    #[arg(long)]
    ensure_unique_name: String,

    #[arg(long)]
    auto_start: String,

    #[arg(long)]
    bridge: String,

    #[arg(long)]
    port: String,

    #[arg(long)]
    host_address: String,

    #[arg(long)]
    local_address: String,
}

fn main() {
    let args = Args::parse();

    println!("{}", args.nixos_path)
}

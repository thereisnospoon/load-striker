use clap::Parser;
use std::sync::mpsc;

mod loader;
mod stats;

#[derive(Parser, Debug)]
struct Args {
    #[clap(parse(from_os_str), short = 'f', long)]
    targets_file: Option<std::path::PathBuf>,
    target: Option<String>,
    #[clap(short = 'u')]
    num_users: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.target.is_none() && args.targets_file.is_none() {
        return Err("Targets are not specified".into());
    }

    let targets: Vec<String> = match &args.target {
        Some(target) => vec![target.clone()],
        _ => {
            let targets_file_path = &args.targets_file.unwrap();
            let contents = std::fs::read_to_string(targets_file_path)
                .expect("could not read file with targets");

            contents
                .lines()
                .into_iter()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect()
        }
    };

    println!("Working on targets: {:?}", targets);

    let (tx, rx) = mpsc::channel();
    stats::consume_stats(rx);
    loader::run_concurrent_requests(args.num_users, targets, tx).await;
    Ok(())
}

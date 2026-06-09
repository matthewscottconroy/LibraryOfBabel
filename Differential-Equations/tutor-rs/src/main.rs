mod adaptive;
mod db;
mod expr;
mod problems;
mod session;
mod ui;

use anyhow::Result;
use clap::{Parser, Subcommand};
use problems::types::Topic;

#[derive(Parser)]
#[command(name = "tutor", about = "Adaptive mathematics problem trainer", version)]
struct Cli {
    /// User profile name
    #[arg(long, short, default_value = "default")]
    profile: String,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Start a practice session (default)
    Practice {
        /// Number of problems
        #[arg(long, short, default_value_t = 10)]
        problems: u32,

        /// Force a specific topic
        #[arg(long, short)]
        topic: Option<String>,
    },
    /// Show mastery dashboard
    Stats,
    /// Reset all progress for this profile
    Reset,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("math-tutor");
    let db_path = data_dir.join(format!("{}.db", cli.profile));
    let db = db::Db::open(&db_path)?;

    match cli.command.unwrap_or(Command::Practice { problems: 10, topic: None }) {
        Command::Practice { problems, topic } => {
            let force_topic = topic.as_deref().and_then(Topic::from_str);
            if topic.is_some() && force_topic.is_none() {
                eprintln!(
                    "Unknown topic '{}'. Valid: analysis, linalg, multivariable, vectors, odes, fourier, pdes, complex_an",
                    topic.unwrap()
                );
                std::process::exit(1);
            }
            session::run(&db, session::SessionConfig {
                profile: cli.profile,
                n_problems: problems,
                force_topic,
            })?;
        }
        Command::Stats => {
            let stats    = db.get_stats(&cli.profile)?;
            let mastery  = db.all_mastery(&cli.profile)?;
            let unlocked = adaptive::unlocked_topics(&mastery);
            let count    = db.session_count(&cli.profile)?;
            ui::stats_table(&stats, &unlocked, count);
        }
        Command::Reset => {
            print!("  Reset ALL progress for profile '{}'? [y/N]: ", cli.profile);
            use std::io::Write;
            std::io::stdout().flush()?;
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf)?;
            if matches!(buf.trim().to_lowercase().as_str(), "y" | "yes") {
                db.reset(&cli.profile)?;
                println!("  Progress reset.");
            } else {
                println!("  Cancelled.");
            }
        }
    }

    Ok(())
}

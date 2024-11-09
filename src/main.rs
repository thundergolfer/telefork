use camino::Utf8PathBuf;

use clap::{Args, Parser, Subcommand};
use tracing::level_filters::LevelFilter;
use tracing_subscriber;
use tracing_subscriber::EnvFilter;

use telefork::cmd;

const NAME: &str = "telefork";


#[derive(Debug, Parser)]
#[clap(name = NAME, version)]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command,
}


#[derive(Debug, Args)]
struct GlobalOpts {
    /// Verbosity level (can be specified multiple times)
    #[clap(long, short, global = true, default_value_t = 0)]
    verbose: usize,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Dump a running process to a file for later restoration.
    Dump {
        /// The pid of the process to dump.
        process_id: i32,
        /// The path to dump to.
        path: Utf8PathBuf,
        /// Restore the process running after dumping.
        #[clap(long)]
        leave_running: bool,
    },
    /// Restore a process from a dumped file.
    Restore {
        /// The dumped file to restore from.
        path: Utf8PathBuf,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = App::parse();

    let level = match cli.global_opts.verbose {
        3.. => LevelFilter::DEBUG.into(),
        2 => LevelFilter::DEBUG.into(),
        1 => LevelFilter::INFO.into(),
        0 => LevelFilter::WARN.into(),
    };

    tracing_subscriber::fmt()
    .with_env_filter(
        EnvFilter::builder()
            .with_default_directive(level)
            .from_env_lossy(),
    )
    .init();

    match cli.command {
        Command::Dump { process_id, path, leave_running } => {
            cmd::dump(process_id, path, leave_running)?;
        }
        Command::Restore { path } => {
            cmd::restore(path)?;
        }
    }
    Ok(())
}


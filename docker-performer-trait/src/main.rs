use eyre::Result;
use structopt::StructOpt;

mod commands;
use commands::{image::*, run::*};

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(subcommand)]
    subcommand: Docker,
}

#[derive(StructOpt, Debug)]
pub enum Docker {
    Image(DockerImage),

    #[structopt(about = "Run a command in a new container")]
    Run(DockerRun),
}

#[allow(dead_code)]
pub struct GlobalOpts {
    debug: bool,
}

trait Performer {
    fn perform(self, opts: GlobalOpts) -> Result<()>;
}

impl Performer for Docker {
    fn perform(self, opts: GlobalOpts) -> Result<()> {
        match self {
            Docker::Image(cmd) => cmd.perform(opts),
            Docker::Run(cmd) => cmd.perform(opts),
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::from_args();
    let opts = GlobalOpts{
        debug: cli.debug,
    };

    if cli.debug { println!("turn on debug.")}
    cli.subcommand.perform(opts)?;
    Ok(())
}
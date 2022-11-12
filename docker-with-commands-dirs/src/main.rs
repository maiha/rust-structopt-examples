use std::error::Error;
use structopt::StructOpt;

mod commands;
use commands::{image::*, run::*};

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(subcommand)]
    subcommand: Docker,
}

#[derive(StructOpt, Debug)]
pub enum Docker {
    Image(DockerImage),

    #[structopt(about = "Run a command in a new container")]
    Run(DockerRunOpts),
}

fn main() -> anyhow::Result<(), Box<dyn Error>> {
    let cli = Cli::from_args();

    if cli.debug { println!("turn on debug.")}

    match cli.subcommand {
        Docker::Image(cmd) => match cmd {
            DockerImage::Inspect(opts) => run_docker_image_inspect(opts),
            DockerImage::List  => run_docker_image_list(cmd),
        },
        Docker::Run(opts)   => run_docker_run(opts),
    }?;
    Ok(())
}
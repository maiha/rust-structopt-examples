use std::{io::Result};
use structopt::StructOpt;

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

fn main() -> Result<()> {
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

#[derive(StructOpt, Debug)]
pub enum DockerImage {
    #[structopt(about = "Display detailed information on one or more images")]
    Inspect(DockerImageOpts),

    #[structopt(name = "ls", about = "List images")]
    List,
}

#[derive(StructOpt, Debug)]
pub struct DockerImageOpts {
    image: String,
}

fn run_docker_image_inspect(opts: DockerImageOpts) -> Result<()> {
    eprintln!("Error: No such image: {:?}", opts.image);
    std::process::exit(-1);
}
    
fn run_docker_image_list(_opts: DockerImage) -> Result<()> {
    println!("REPOSITORY   TAG       IMAGE ID   CREATED   SIZE");
    Ok(())
}
    
#[derive(StructOpt, Debug)]
pub struct DockerRunOpts {
    image: String,
}

fn run_docker_run(opts: DockerRunOpts) -> Result<()> {
    println!("docker run {:?}", opts.image);
    Ok(())
}

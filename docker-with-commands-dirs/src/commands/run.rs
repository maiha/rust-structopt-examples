use anyhow::Result;
use std::error::Error;
use structopt::StructOpt;
use Debug;

#[derive(StructOpt, Debug)]
pub struct DockerRunOpts {
    image: String,
}

pub fn run_docker_run(opts: DockerRunOpts) -> Result<(), Box<dyn Error>> {
    println!("docker run {:?}", opts.image);
    Ok(())
}

use anyhow::Result;
use std::error::Error;
use structopt::StructOpt;
use Debug;

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

pub fn run_docker_image_inspect(opts: DockerImageOpts) -> Result<(), Box<dyn Error>> {
    eprintln!("Error: No such image: {:?}", opts.image);
    std::process::exit(-1);
}

pub fn run_docker_image_list(_opts: DockerImage) -> Result<(), Box<dyn Error>> {
    println!("REPOSITORY   TAG       IMAGE ID   CREATED   SIZE");
    Ok(())
}
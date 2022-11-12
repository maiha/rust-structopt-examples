use eyre::Result;
use structopt::StructOpt;
use Debug;

use crate::{Performer, GlobalOpts};

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

impl Performer for DockerImage {
    fn perform(self, global: GlobalOpts) -> Result<()> {
        match self {
            DockerImage::Inspect(local) => run_inspect(global, local),
            DockerImage::List => run_list(global),
        }
    }
}

#[allow(unused_variables)]
pub fn run_inspect(global: GlobalOpts, local: DockerImageOpts) -> Result<()> {
    eprintln!("Error: No such image: {:?}", local.image);
    std::process::exit(-1);
}

#[allow(unused_variables)]
pub fn run_list(global: GlobalOpts) -> Result<()> {
    println!("REPOSITORY   TAG       IMAGE ID   CREATED   SIZE");
    Ok(())
}
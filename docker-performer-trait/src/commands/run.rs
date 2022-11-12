use eyre::Result;
use structopt::StructOpt;
use Debug;

use crate::{Performer, GlobalOpts};

#[derive(StructOpt, Debug)]
pub struct DockerRun {
    image: String,
}

impl Performer for DockerRun {
    fn perform(self, global: GlobalOpts) -> Result<()> {
        docker_run(global, self)
    }
}

#[allow(unused_variables)]
pub fn docker_run(global: GlobalOpts, local: DockerRun) -> Result<()> {
    println!("docker run {:?}", local.image);
    Ok(())
}

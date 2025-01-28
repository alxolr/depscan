use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Generate {
    #[structopt(short, long, parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

impl Generate {
    pub fn run(&self) {
        println!(
            "Generating the dependency graph for the project at {:?}",
            self.path
        );
    }
}

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "salus")]
struct Opt {
    #[structopt(subcommand)]
    command: Subcommand,
}

#[derive(StructOpt, Debug)]
enum Subcommand {
    #[structopt(name = "create", no_version)]
    Create {
        #[structopt(name = "container-id")]
        id: String,

        #[structopt(name = "path-to-bundle", parse(from_os_str))]
        bundle_path: PathBuf,
    },

    #[structopt(name = "state", no_version)]
    State {
        #[structopt(name = "container-id")]
        id: String,
    },

    #[structopt(name = "start", no_version)]
    Start {
        #[structopt(name = "container-id")]
        id: String,
    },

    #[structopt(name = "kill", no_version)]
    Kill {
        #[structopt(name = "container-id")]
        id: String,

        #[structopt(name = "signal", default_value = "SIGTERM")]
        signal: String,
    },

    #[structopt(name = "delete", no_version)]
    Delete {
        #[structopt(name = "container-id")]
        id: String,
    },
}

fn main() {
    match Opt::from_args().command {
        Subcommand::Create {
            id: _,
            bundle_path: _,
        } => unimplemented!(),
        Subcommand::State { id: _ } => unimplemented!(),
        Subcommand::Start { id: _ } => unimplemented!(),
        Subcommand::Kill { id: _, signal: _ } => unimplemented!(),
        Subcommand::Delete { id: _ } => unimplemented!(),
    }
}

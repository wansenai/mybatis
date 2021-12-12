use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action{

    Add{
        text : String,
    },

    Done{
        position : usize,
    },

    List,

}

#[derive(Debug, StructOpt)]
#[structopt(
name = "zhaowei",
about = "zhaowei file"
)]
pub struct CommandLineArgs{

    #[structopt(subcommand)]
    pub action : Action,

    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
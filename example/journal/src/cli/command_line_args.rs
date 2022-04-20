use std::path::PathBuf;

#[allow(dead_code)]
pub enum Action{
    Add { task : String},
    Done { position : usize},
    List,
}

#[allow(dead_code)]
pub struct CommandLineArgs{
    pub action: Action,
    pub journal_file: Option<PathBuf>,
}
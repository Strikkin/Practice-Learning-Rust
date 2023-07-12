use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about)]
pub struct TodoArgs{
    #[clap(subcommand)]
    command: Option<Action>,
    #[clap(short, long, default_value = "~/.todo")]
    pub path: String,
}

#[derive(Debug, clap::Subcommand)]
pub enum Action {
    /// Add entry
    Add {entry: Option<String>},
    ///Remove entry
    Rem {id: Option<u32>},
    ///Delete a list 
    Del,
    ///Complete a task
    Done {id: Option<u32>},
    /// View a list
    View {id: Option<u32>}
}
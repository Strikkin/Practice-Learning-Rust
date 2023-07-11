use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TodoArgs{
    #[clap(subcommand)]
    pub modify_list: ModifyType,
}

#[derive(Debug, Subcommand)]
pub enum ModifyType {
    /// Add a task
    Add(AddCommand),
    ///Remove a task
    Remove(RemCommand),
    ///Delete a list 
    Delete(DelCommand),
    /// View a list
    View(ViewCommand)
}


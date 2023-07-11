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

#[derive(Debug, Args)]
pub struct AddCommand {
    ///Add task to list
    task_to_add: String,
}

#[derive(Debug, Args)]
pub struct RemCommand {
    ///remove task from list
    task_to_remove: String,
}

#[derive(Debug, Args)]
pub struct DelCommand{
    list_to_del: String,
}

#[derive(Debug, Args)]
pub struct ViewCommand{
    view_list: String,
}

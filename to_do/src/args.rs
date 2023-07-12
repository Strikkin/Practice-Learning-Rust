use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about)]
pub struct TodoArgs{
    #[clap(subcommand)]
    pub modify_list: ModifyType,
    #[clap(short, long, default_value = "$HOME/.todo")]
    path: String,
}

#[derive(Debug, Subcommand)]
pub enum ModifyType {
    /// Add a task
    Add(AddCommand),
    ///Remove a task
    Rem(RemCommand),
    ///Delete a list 
    Del(DelCommand),
    ///Complete a task
    Done(DoneCommand),
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
    task_to_remove: u32,
}

#[derive(Debug, Args)]
pub struct DelCommand{
    list_to_del: String,
}

#[derive(Debug, Args)]
pub struct ViewCommand{
    view_list: String,
}

#[derive(Debug, Args)]
pub struct DoneCommand {
    complete_task: u32,
}

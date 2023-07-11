use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TodoArgs{
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Users
    User(UserCommand),
    /// Views
    View(ViewCommand)
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    ///Create new User
    Create(CreateUser),
    /// Update a User
    Update(UpdateUser),
    /// Delete a User
    Delete(DeleteEntity),
    /// Show all Users
    Show,
}

use clap:: {
    Args,
    Parser,
    Subcommand
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]

pub struct todo_args {
    /// Add to an existing list
    /// Remove from a list
    /// 
}
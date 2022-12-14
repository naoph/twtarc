use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// View and modify user subscriptions
    #[command(arg_required_else_help = true)]
    User {
        #[command(subcommand)]
        command: UserCommands,
    },

    /// Retrieve new data
    #[command(name = "sync")]
    Synch,

    /// Manually archive a tweet
    Insert {
        url: url::Url,
    },
}

#[derive(Debug, Subcommand)]
pub enum UserCommands {
    /// Subscribe to a new user
    Add {
        url: url::Url,
    },

    /// List user subscriptions
    List,
}

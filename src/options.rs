use clap::Clap;

#[derive(Clap, Debug)]
pub struct Opts {
    /// show verbose message
    #[clap(short, long)]
    pub verbose: bool,
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Clap, Debug)]
pub enum Action {
    /// list all drafts.
    List,
    /// create a new draft.
    New { name: String },
    /// post a draft.
    Post {
        name: String,
        #[clap(short, long)]
        update: bool,
    },
}

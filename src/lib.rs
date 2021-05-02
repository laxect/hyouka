#![feature(once_cell, with_options)]
use clap::Clap;

pub use action::take_action;
pub use config::load_config;
pub(crate) use config::{target_dir, working_dir};
pub(crate) use flags::vv;
pub use print::Check;

mod action;
mod config;
#[macro_use]
mod print;
mod flags {
    use std::sync::atomic::{AtomicBool, Ordering};

    /// global flag which can only set once.
    static VV: AtomicBool = AtomicBool::new(false);

    pub(crate) fn set_vv(val: bool) {
        VV.store(val, Ordering::SeqCst);
    }

    pub fn vv() -> bool {
        VV.load(Ordering::Relaxed)
    }
}

#[derive(Clap, Debug)]
struct Opts {
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

pub fn parse_opts() -> anyhow::Result<Action> {
    let Opts { verbose, action } = Opts::parse();
    flags::set_vv(verbose);
    Ok(action)
}

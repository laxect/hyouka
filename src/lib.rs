#![feature(once_cell)]
use clap::Clap;

pub use config::load_config;
pub(crate) use config::{target_dir, working_dir};
pub(crate) use flags::{set_vv, vv};

pub mod print;
mod config;
mod flags {
    use std::lazy::SyncOnceCell;

    /// global flag which can only set once.
    static VV: SyncOnceCell<bool> = SyncOnceCell::new();

    pub fn set_vv(val: bool) {
        VV.set(val).expect("Set twice");
    }

    pub fn vv() -> bool {
        *VV.get().expect("Use before inital")
    }
}

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
    Post { name: String },
}

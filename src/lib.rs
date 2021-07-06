use clap::Clap;

pub use action::take_action;
pub use config::load_config;
pub(crate) use config::{target_dir, working_dir};
pub(crate) use flags::vv;
use options::{Action, Opts};
pub use print::Check;

mod action;
mod config;
#[macro_use]
mod print;
mod options;
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

pub fn parse_opts() -> anyhow::Result<Action> {
    let Opts { verbose, action } = Opts::parse();
    flags::set_vv(verbose);
    Ok(action)
}

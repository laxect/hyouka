use hyouka::{self, Check};

fn run() -> anyhow::Result<()> {
    let action = hyouka::parse_opts().print_err()?;
    hyouka::load_config().print_err()?;
    hyouka::take_action(action).print_err()?;
    Ok(())
}

fn main() {
    run().ok();
}

use crate::{print, print::Check, target_dir, working_dir, Action};
use std::{fs, io::Write};

const TEMPLATE: &[u8] = include_bytes!("../template.txt");

pub fn take_action(action: Action) -> anyhow::Result<()> {
    match action {
        Action::List => {
            print::section("List drafts");
            list()?;
        }
        Action::New { name } => {
            print::section("Create draft");
            new(name)?;
        }
        _ => unimplemented!(),
    }
    Ok(())
}

fn list() -> anyhow::Result<()> {
    let working_dir = working_dir();
    print::vline(format!("working dir: {}", working_dir.to_string_lossy()));
    print::verbose("scan dir");
    let dir = fs::read_dir(working_dir).verbose()?;
    for item in dir {
        let item = item?;
        println!("{}", item.path().file_stem().expect("root file").to_string_lossy());
    }
    Ok(())
}

fn new(mut name: String) -> anyhow::Result<()> {
    let working_dir = working_dir();
    print::vline(format!("working dir: {}", working_dir.to_string_lossy()));
    name.push_str(".md");
    print::verbose(format!("create file {}", &name));
    let path = working_dir.join(name);
    let mut file = fs::File::with_options()
        .create_new(true)
        .write(true)
        .open(path)
        .verbose()?;
    file.write_all(TEMPLATE)?;
    Ok(())
}

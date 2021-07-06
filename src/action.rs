use crate::{line, print, print::Check, target_dir, verbose, working_dir, Action};
use koyomi::Date;
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
        Action::Post { name, update } => {
            print::section("Post draft");
            post(name, update)?;
        }
    }
    Ok(())
}

fn list() -> anyhow::Result<()> {
    let working_dir = working_dir();
    line!("working dir: {}", working_dir.to_string_lossy());
    verbose!("scan dir");
    let dir = fs::read_dir(working_dir).verbose()?;
    for item in dir {
        let item = item?;
        println!("{}", item.path().file_stem().expect("root file").to_string_lossy());
    }
    Ok(())
}

fn new(mut name: String) -> anyhow::Result<()> {
    let working_dir = working_dir();
    line!("working dir: {}", working_dir.to_string_lossy());
    name.push_str(".md");
    verbose!("create file {}", &name);
    let path = working_dir.join(name);
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .verbose()?;
    file.write_all(TEMPLATE)?;
    Ok(())
}

fn post(name: String, update: bool) -> anyhow::Result<()> {
    let working_dir = working_dir();
    line!("working dir: {}", working_dir.to_string_lossy());
    let target_dir = target_dir();
    line!("target dir: {}", target_dir.to_string_lossy());
    let origin_name = [&name, ".md"].concat();
    let origin_path = working_dir.join(&origin_name);
    let target_name = [&today(), "-", &origin_name].concat();
    let target_path = target_dir.join(&target_name);
    if target_path.exists() {
        if update {
            verbose!("remove file {}", target_path.to_string_lossy());
            fs::remove_file(&target_path).verbose()?;
        } else {
            return Err(anyhow::Error::msg("file exist! use --update or manual solve conflict."));
        }
    }
    verbose!("read {}", &origin_name);
    let origin_content = fs::read_to_string(origin_path).verbose()?;
    let koyomi_content: String = origin_content.replacen(
        "\ndate:",
        format!("\ndate: {}", koyomi_day(chrono::Local::today().naive_local())).as_str(),
        1,
    );
    verbose!("post to {}", &target_name);
    fs::write(&target_path, koyomi_content).verbose()?;
    Ok(())
}

fn today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

fn koyomi_day(now: chrono::NaiveDate) -> String {
    let date = Date::from(now);
    format!(
        "{}{}月{}日",
        date.era().expect("Hello John").format(),
        date.month(),
        date.day()
    )
}

#[cfg(test)]
mod test {
    use super::koyomi_day;

    #[test]
    fn today_test() {
        let day = chrono::NaiveDate::from_ymd(2018, 1, 1);
        let day = koyomi_day(day);
        assert_eq!("平成30年1月1日", day);
    }
}

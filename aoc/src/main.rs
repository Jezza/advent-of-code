use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use anyhow::Context;
use structopt::StructOpt;

use parse_display::{Display, FromStr};

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Debug, clap::Parser)]
enum Command {
    Add(AddOpts),
}

#[derive(Debug, clap::Parser)]
struct AddOpts {
    year: u32,
    day: u32,
}

#[derive(Display, FromStr, PartialEq, Debug, Copy, Clone)]
#[display("{year:04}/day-{day:02}")]
struct MemberName {
    year: u32,
    day: u32,
}

macro_rules! format_str {
	($prefix:literal, $value:literal) => {{
		let value = String::from($value);
		format_str!($prefix, value, "")
	}};
	($prefix:literal, $value:literal, $suffix:literal) => {{
		let value = String::from($value);
		format_str!($prefix, value, $suffix)
	}};
	($prefix:literal, $value:expr) => {{
		format_str!($prefix, $value, "")
	}};
	($prefix:literal, $value:expr, $suffix:literal) => {{
		let mut name = toml_edit::Formatted::new($value);
		let decor = name.decor_mut();
		decor.set_prefix($prefix);
		decor.set_suffix($suffix);
		toml_edit::Value::String(name)
	}};
}

fn main() -> anyhow::Result<()> {
    let Args {
        cmd,
    } = clap::Parser::parse();

    match cmd {
        Command::Add(opts) => handle_add(opts),
    }
}

fn handle_add(opts: AddOpts) -> anyhow::Result<()> {
    let AddOpts {
        year,
        day,
    } = opts;

    let cwd = std::env::current_dir()?;

    let (path, doc) = find_path(&cwd, "Cargo.toml", |path| {
        let content = std::fs::read_to_string(path)
            .ok()?;

        let doc = content.parse::<toml_edit::Document>().ok()?;

        let workspace = doc.get("workspace")?;
        let _ = workspace.get("members")?;

        Some(doc)
    }).context("Unable to find Cargo.toml")?;

    let member = MemberName {
        year,
        day,
    };

    add_member(member, doc, &path)?;

    let mut base = path.clone();
    base.pop();

    base.push(format!("{:04}", year));
    base.push(format!("day-{:02}", day));

    std::fs::create_dir_all(&base)?;

    let cargo = base.join("Cargo.toml");
    let main = {
        let mut base = base.join("src");
        std::fs::create_dir_all(&base)?;
        base.push("main.rs");
        base
    };
    let input = {
        let mut base = base.join("input");
        std::fs::create_dir_all(&base)?;
        base.push("input.txt");
        base
    };
    let test_input = {
        let mut base = base.join("input");
        std::fs::create_dir_all(&base)?;
        base.push("test-1.txt");
        base
    };

    // println!("{}", cargo.display());
    // println!("{}", main.display());
    // println!("{}", input.display());

    let mut cargo_content = String::from(include_str!("../template/Cargo.toml"));
    let main_content = include_str!("../template/main.rs");

    const NAME: &str = "{{crate_name}}";
    if let Some(index) = cargo_content.find(NAME) {
        let crate_name = format!("aoc-{:04}-{:02}", year, day);
        cargo_content.replace_range(index..index + NAME.len(), &crate_name);
    }

    std::fs::write(&cargo, cargo_content)?;
    std::fs::write(&main, main_content)?;
    std::fs::write(&input, "")?;
    std::fs::write(&test_input, "")?;

    Ok(())
}

fn add_member(
    member: MemberName,
    mut doc: toml_edit::Document,
    path: &Path,
) -> anyhow::Result<()> {

    // let member_name = format!("{:04}/day-{:02}", year, day);
    // let crate_name = format!("aoc-D{:04}-{:02}", day, year);

    let workspace = doc.get_mut("workspace")
        .expect("Internal error");
    let members = workspace.get_mut("members")
        .expect("Internal error");

    let mut internal = vec![];
    let mut names = vec![];

    if let Some(members) = members.as_array() {
        for member in members {
            if let Some(value) = member.as_str() {
                if let Ok(name) = value.parse::<MemberName>() {
                    names.push(name);
                } else {
                    internal.push(String::from(value));
                }
            }
        }
    } else {
        panic!("Internal Error");
    }

    if names.contains(&member) {
        return Err(anyhow::anyhow!("Member already exists: {}", &member));
    }

    names.push(member);

    names.sort_unstable_by(|l, r| {
        let order = l.year.cmp(&r.year);
        if order != Ordering::Equal {
            order
        } else {
            l.day.cmp(&r.day)
        }
    });

    // println!("{:#?}", internal);
    // println!("{:#?}", names);

    let mut array = toml_edit::Array::new();

    for name in internal {
        array.push_formatted(format_str!("\n\t", name))
    }

    let mut last = None;

    for name in names {
        let value = if let Some(last_year) = last {
            if last_year == name.year {
                format_str!("\n\t", name.to_string())
            } else {
                last = Some(name.year);
                format_str!("\n\n\t", name.to_string())
            }
        } else {
            last = Some(name.year);
            format_str!("\n\n\t", name.to_string())
        };
        array.push_formatted(value);
    }

    array.set_trailing_comma(true);
    array.set_trailing("\n");
    if let Some(members) = members.as_array_mut() {
        *members = array;
    }

    let output = doc.to_string();
    let mut out = path.to_path_buf();
    out.pop();
    out.push("Cargo.toml");

    std::fs::write(&out, &output)?;

    Ok(())
}

fn find_path<T>(
    start: &Path,
    file: impl AsRef<Path>,
    handler: fn(&Path) -> Option<T>,
) -> Option<(PathBuf, T)> {
    let mut path = start.to_path_buf();

    let file_name = file.as_ref();

    loop {
        path.push(file_name);

        if path.exists() {
            if let Some(value) = handler(&path) {
                return Some((path, value));
            }
        }

        // Remove the file name
        if !path.pop() {
            break;
        }
        // Remove the parent directory.
        if !path.pop() {
            break;
        }
    }

    None
}


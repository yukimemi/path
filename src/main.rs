use anyhow::{Context, Result};
use clap::{app_from_crate, App, AppSettings, Arg, ArgMatches};
use std::path::Path;

fn extension(m: &ArgMatches) -> Result<String> {
    let path = m.value_of("PATH").context("<PATH> is required !")?;
    let path = Path::new(&path);
    let extension = path.extension().context("Failed to get extension !")?;
    Ok(extension.to_string_lossy().to_string())
}

fn parent(m: &ArgMatches) -> Result<String> {
    let path = m.value_of("PATH").context("<PATH> is required !")?;
    let path = Path::new(&path);
    let parent = path.parent().context("Failed to get parent !")?;
    Ok(parent.to_string_lossy().to_string())
}

fn main() -> Result<()> {
    let matches = app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandRequired)
        .arg(Arg::from("-n --newline 'Output a newline'").global(true))
        .subcommand(
            App::new("parent")
                .about("Get parent directory")
                .arg("<PATH>          'target path'"),
        )
        .subcommand(
            App::new("extension")
                .about("Get file extension")
                .arg("<PATH>          'target path'"),
        )
        .get_matches();

    let s = match matches.subcommand() {
        Some(("parent", sub_m)) => parent(sub_m)?,
        Some(("extension", sub_m)) => extension(sub_m)?,
        _ => unreachable!(),
    };

    if matches.is_present("newline") {
        println!("{}", s);
    } else {
        print!("{}", s);
    }

    Ok(())
}

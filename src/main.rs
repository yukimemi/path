use anyhow::{Context, Result};
use clap::{app_from_crate, App, AppSettings, ArgMatches};
use std::path::Path;

fn extension(rm: &ArgMatches, m: &ArgMatches) -> Result<String> {
    let path = rm.value_of("PATH").context("<PATH> is required !")?;
    let path = Path::new(&path);
    let extension = path.extension().context("Failed to get extension !")?;
    println!("{:?}", &extension);
    Ok(extension.to_string_lossy().to_string())
}

fn parent(rm: &ArgMatches, m: &ArgMatches) -> Result<String> {
    let path = rm.value_of("PATH").context("<PATH> is required !")?;
    let path = Path::new(&path);
    let parent = path.parent().context("Failed to get parent !")?;
    println!("{:?}", &parent);
    Ok(parent.to_string_lossy().to_string())
}

fn main() -> Result<()> {
    let matches = app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .arg("-n --newline 'Output a newline'")
        .arg("<PATH>          'target path'")
        .subcommand(App::new("parent").about("Get parent directory"))
        .subcommand(App::new("extension").about("Get file extension"))
        .get_matches();

    dbg!(&matches);

    let s = match matches.subcommand() {
        Some(("parent", sub_m)) => parent(&matches, sub_m)?,
        Some(("extension", sub_m)) => extension(&matches, sub_m)?,
        _ => "".to_string(),
    };

    print!("{}", s);

    Ok(())
}

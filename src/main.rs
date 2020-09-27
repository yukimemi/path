use anyhow::{Context, Result};
use argopt::{cmd_group, subcmd};
use std::path::Path;

#[subcmd]
fn extension(path: String) -> Result<()> {
    let path = Path::new(&path);
    println!(
        "{:?}",
        path.extension().context("Failed to get extension !")?
    );
    Ok(())
}

#[subcmd]
fn parent(path: String) -> Result<()> {
    let path = Path::new(&path);
    println!("{:?}", path.parent().context("Failed to get parent !")?);
    Ok(())
}

#[cmd_group(commands = [extension,parent])]
fn main() -> Result<()> {}

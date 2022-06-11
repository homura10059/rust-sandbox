use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_ref().map(|it| it.as_str()) {
        Some("lint") => lint()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:
lint            fmt & fix & clippy
"
    )
}

fn format() -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .arg("fmt")
        .status()?;

    if !status.success() {
        Err("cargo fmt failed")?;
    }

    Ok(())
}

fn fix() -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["fix", "--allow-dirty", "--allow-staged"])
        .status()?;

    if !status.success() {
        Err("cargo fix failed")?;
    }

    Ok(())
}

fn clippy() -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .arg("clippy")
        .status()?;

    if !status.success() {
        Err("cargo clippy failed")?;
    }

    Ok(())
}

fn lint() -> Result<(), DynError> {
    format()?;
    fix()?;
    clippy()?;

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

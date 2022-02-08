use anyhow::{anyhow, Context, Result};

use cargo_manifest::Manifest;
use cargo_ros_install::*;

fn main() {
    let exitcode = match fallible_main().context("Error in cargo-ros-install") {
        Ok(Some(code)) => code,
        Ok(None) => {
            eprintln!("'cargo build' was terminated by signal.");
            1
        }
        Err(e) => {
            eprintln!("{:?}", e);
            1
        }
    };
    // No destructors left to run, so it's fine to exit.
    std::process::exit(exitcode);
}

fn fallible_main() -> Result<Option<i32>> {
    let args = Args::parse()?;
    let mut manifest = Manifest::from_path(&args.manifest_path)?;
    manifest.complete_from_path(&args.manifest_path)?;

    // Unwrap is safe since complete_from_path() has been called
    let is_pure_library = manifest.bin.as_ref().unwrap().is_empty();
    let exitcode = cargo_build(&args.forwarded_args, is_pure_library)?;

    let package_name = &manifest
        .package
        .ok_or(anyhow!("Package has no name."))?
        .name;
    let package_path = args
        .manifest_path
        .parent()
        .ok_or(anyhow!("Manifest path must have a parent."))?;

    // Putting marker file creation after the actual build command means that
    // we create less garbage if the build command failed.
    create_package_marker(&args.install_base, "packages", package_name)?;
    // This marker is used by colcon-ros-cargo when looking for dependencies
    create_package_marker(&args.install_base, "rust_packages", package_name)?;
    install_package(&args.install_base, package_path, package_name)?;
    install_binaries(
        &args.install_base,
        &args.build_base,
        package_name,
        &args.profile,
        // Unwrap is safe since complete_from_path() has been called
        &manifest.bin.unwrap(),
    )?;
    Ok(exitcode)
}

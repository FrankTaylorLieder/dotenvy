use serial_test::serial;
use std::{env, error::Error, path::PathBuf, result::Result};

mod common;

use dotenvy::*;

use crate::common::*;

fn check_missing_fails(
    loader: fn() -> dotenvy::Result<Option<PathBuf>>,
) -> Result<(), Box<dyn Error>> {
    let _ce = CleanEnv::new();
    let dir = tempdir_without_dotenv()?;

    assert!(loader().is_err());

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

fn check_missing_optional(
    loader: fn() -> dotenvy::Result<Option<PathBuf>>,
) -> Result<(), Box<dyn Error>> {
    let _ce = CleanEnv::new();
    let dir = tempdir_without_dotenv()?;

    loader()?;

    println!("TESTKEY: {:?}", env::var("TESTKEY"));

    assert!(env::var("TESTKEY").is_err());
    assert_eq!(env::var("EXISTING")?, "from_env");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

fn check_normal(loader: fn() -> dotenvy::Result<Option<PathBuf>>) -> Result<(), Box<dyn Error>> {
    let _ce = CleanEnv::new();
    let dir = make_test_dotenv()?;

    loader()?;

    assert_eq!(env::var("TESTKEY")?, "test_val");
    assert_eq!(env::var("EXISTING")?, "from_env");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;

    Ok(())
}

fn check_override(loader: fn() -> dotenvy::Result<Option<PathBuf>>) -> Result<(), Box<dyn Error>> {
    let _ce = CleanEnv::new();
    let dir = make_test_dotenv()?;

    loader()?;

    assert_eq!(env::var("TESTKEY")?, "test_val_overridden");
    assert_eq!(env::var("EXISTING")?, "from_file");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;

    Ok(())
}

#[test]
#[serial]
fn test_builder_default() -> Result<(), Box<dyn Error>> {
    check_missing_fails(|| build().load())?;
    check_missing_optional(|| build().optional().load())?;
    check_normal(|| build().load())?;
    check_override(|| build().overryde().load())?;

    Ok(())
}

#[test]
#[serial]
fn test_builder_filename() -> Result<(), Box<dyn Error>> {
    check_missing_fails(|| build().from_filename(".env").load())?;
    check_missing_optional(|| build().from_filename(".env").optional().load())?;
    check_normal(|| build().from_filename(".env").load())?;
    check_override(|| build().from_filename(".env").overryde().load())?;

    Ok(())
}

// TODO: all iter cases, all reader cases, all path cases

use serial_test::serial;
use std::{env, error::Error, path::PathBuf, result::Result};

mod common;

use dotenvy::*;

use crate::common::*;

type Loader = fn(&PathBuf) -> dotenvy::Result<Option<PathBuf>>;

fn check_missing_fails(loader: Loader) -> Result<(), Box<dyn Error>> {
    println!("check_missing_fails");
    let _ce = CleanEnv::new();
    let dir = tempdir_without_dotenv()?;

    let mut path = env::current_dir()?;
    path.push(".env");

    assert!(loader(&path).is_err());

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

fn check_missing_optional(loader: Loader) -> Result<(), Box<dyn Error>> {
    println!("check_missing_optional");
    let _ce = CleanEnv::new();
    let dir = tempdir_without_dotenv()?;

    let mut path = env::current_dir()?;
    path.push(".env");

    loader(&path)?;

    println!("TESTKEY: {:?}", env::var("TESTKEY"));

    assert!(env::var("TESTKEY").is_err());
    assert_eq!(env::var("EXISTING")?, "from_env");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

fn check_normal(loader: Loader) -> Result<(), Box<dyn Error>> {
    println!("check_normal");
    let _ce = CleanEnv::new();
    let dir = make_test_dotenv()?;

    let mut path = env::current_dir()?;
    path.push(".env");

    loader(&path)?;

    assert_eq!(env::var("TESTKEY")?, "test_val");
    assert_eq!(env::var("EXISTING")?, "from_env");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;

    Ok(())
}

fn check_override(loader: Loader) -> Result<(), Box<dyn Error>> {
    println!("check_override");
    let _ce = CleanEnv::new();
    let dir = make_test_dotenv()?;

    let mut path = env::current_dir()?;
    path.push(".env");

    loader(&path)?;

    assert_eq!(env::var("TESTKEY")?, "test_val_overridden");
    assert_eq!(env::var("EXISTING")?, "from_file");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;

    Ok(())
}

#[test]
#[serial]
fn test_builder_default() -> Result<(), Box<dyn Error>> {
    check_missing_fails(|_| build().load())?;
    check_missing_optional(|_| build().optional().load())?;
    check_normal(|_| build().load())?;
    check_override(|_| build().overryde().load())?;

    Ok(())
}

#[test]
#[serial]
fn test_builder_filename() -> Result<(), Box<dyn Error>> {
    check_missing_fails(|_| build().from_filename(".env").load())?;
    check_missing_optional(|_| build().from_filename(".env").optional().load())?;
    check_normal(|_| build().from_filename(".env").load())?;
    check_override(|_| build().from_filename(".env").overryde().load())?;

    Ok(())
}

#[test]
#[serial]
fn test_builder_path() -> Result<(), Box<dyn Error>> {
    check_missing_fails(|p| build().from_path(p).load())?;
    check_missing_optional(|p| build().from_path(p).optional().load())?;
    check_normal(|p| build().from_path(p).load())?;
    check_override(|p| build().from_path(p).overryde().load())?;

    Ok(())
}

// TODO: all iter cases, all reader cases, all path cases

use serial_test::serial;
use std::{env, error::Error, result::Result};

mod common;

use dotenvy::*;

use crate::common::*;

#[test]
#[serial]
fn test_builder_missing_fails() -> Result<(), Box<dyn Error>> {
    let _ce = CleanEnv::new();
    let dir = tempdir_without_dotenv()?;

    assert!(build().load().is_err());

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

#[test]
#[serial]
fn test_builder_missing_optional() -> Result<(), Box<dyn Error>> {
    let _ce = CleanEnv::new();
    let dir = tempdir_without_dotenv()?;

    build().optional().load()?;

    println!("TESTKEY: {:?}", env::var("TESTKEY"));

    assert!(env::var("TESTKEY").is_err());
    assert_eq!(env::var("EXISTING")?, "from_env");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

#[test]
#[serial]
fn test_builder_default() -> Result<(), Box<dyn Error>> {
    let _ce = CleanEnv::new();
    let dir = make_test_dotenv()?;

    build().load()?;

    assert_eq!(env::var("TESTKEY")?, "test_val");
    assert_eq!(env::var("EXISTING")?, "from_env");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;

    Ok(())
}

#[test]
#[serial]
fn test_builder_default_override() -> Result<(), Box<dyn Error>> {
    let _ce = CleanEnv::new();
    let dir = make_test_dotenv()?;

    build().overryde().load()?;

    assert_eq!(env::var("TESTKEY")?, "test_val_overridden");
    assert_eq!(env::var("EXISTING")?, "from_file");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

#[test]
#[serial]
fn test_builder_from_filename() -> Result<(), Box<dyn Error>> {
    let _ce = CleanEnv::new();
    let dir = make_test_dotenv()?;

    build().from_filename(".env").load()?;

    assert_eq!(env::var("TESTKEY")?, "test_val");
    assert_eq!(env::var("EXISTING")?, "from_env");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

#[test]
#[serial]
fn test_builder_from_filename_override() -> Result<(), Box<dyn Error>> {
    let _ce = CleanEnv::new();
    let dir = make_test_dotenv()?;

    build().from_filename(".env").overryde().load()?;

    assert_eq!(env::var("TESTKEY")?, "test_val_overridden");
    assert_eq!(env::var("EXISTING")?, "from_file");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

#[test]
#[serial]
fn test_builder_from_filename_missing_fails() -> Result<(), Box<dyn Error>> {
    let _ce = CleanEnv::new();
    let dir = tempdir_without_dotenv()?;

    assert!(build().from_filename(".env").load().is_err());

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

#[test]
#[serial]
fn test_builder_from_filename_missing_optional() -> Result<(), Box<dyn Error>> {
    let _ce = CleanEnv::new();
    let dir = tempdir_without_dotenv()?;

    build().from_filename(".env").optional().load()?;

    println!("TESTKEY: {:?}", env::var("TESTKEY"));

    assert!(env::var("TESTKEY").is_err());
    assert_eq!(env::var("EXISTING")?, "from_env");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

// TODO: all iter cases, all reader cases, all path cases

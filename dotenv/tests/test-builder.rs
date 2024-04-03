use serial_test::serial;
use std::{env, error::Error, fs::File, path::PathBuf, result::Result};

mod common;

use dotenvy::*;

use crate::common::*;

type InjectLoad = fn(&PathBuf) -> dotenvy::Result<Option<PathBuf>>;
type InjectIter = fn(&PathBuf) -> dotenvy::Result<Option<Iter<File>>>;

fn check_missing_fails_load(loader: InjectLoad) -> Result<(), Box<dyn Error>> {
    println!("check_missing_fails_load");
    let _ce = CleanEnv::new();
    let dir = tempdir_without_dotenv()?;

    let mut path = env::current_dir()?;
    path.push(".env");

    assert!(loader(&path).is_err());

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

fn check_missing_optional_load(loader: InjectLoad) -> Result<(), Box<dyn Error>> {
    println!("check_missing_optional_load");
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

fn check_normal_load(loader: InjectLoad) -> Result<(), Box<dyn Error>> {
    println!("check_normal_load");
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

fn check_override_load(loader: InjectLoad) -> Result<(), Box<dyn Error>> {
    println!("check_override_load");
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

fn check_missing_fails_iter(loader: InjectIter) -> Result<(), Box<dyn Error>> {
    println!("check_missing_fails_iter");
    let _ce = CleanEnv::new();
    let dir = tempdir_without_dotenv()?;

    let mut path = env::current_dir()?;
    path.push(".env");

    assert!(loader(&path).is_err());

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

fn check_missing_optional_iter(loader: InjectIter) -> Result<(), Box<dyn Error>> {
    println!("check_missing_optional_iter");
    let _ce = CleanEnv::new();
    let dir = tempdir_without_dotenv()?;

    let mut path = env::current_dir()?;
    path.push(".env");

    let iter = loader(&path)?;

    assert!(env::var("TESTKEY").is_err());

    if let Some(iter) = iter {
        iter.load()?;
    }
    assert!(env::var("TESTKEY").is_err());
    assert_eq!(env::var("EXISTING")?, "from_env");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;

    Ok(())
}

fn check_normal_iter(loader: InjectIter) -> Result<(), Box<dyn Error>> {
    println!("check_normal_iter");
    let _ce = CleanEnv::new();
    let dir = make_test_dotenv()?;

    let mut path = env::current_dir()?;
    path.push(".env");

    let iter = loader(&path)?;

    assert!(env::var("TESTKEY").is_err());

    if let Some(iter) = iter {
        iter.load()?;
    }
    assert_eq!(env::var("TESTKEY")?, "test_val");
    assert_eq!(env::var("EXISTING")?, "from_env");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;

    Ok(())
}

#[test]
#[serial]
fn test_load_builder_default() -> Result<(), Box<dyn Error>> {
    check_missing_fails_load(|_| build().load())?;
    check_missing_optional_load(|_| build().optional().load())?;
    check_normal_load(|_| build().load())?;
    check_override_load(|_| build().overryde().load())?;

    Ok(())
}

#[test]
#[serial]
fn test_load_builder_filename() -> Result<(), Box<dyn Error>> {
    check_missing_fails_load(|_| build().from_filename(".env").load())?;
    check_missing_optional_load(|_| build().from_filename(".env").optional().load())?;
    check_normal_load(|_| build().from_filename(".env").load())?;
    check_override_load(|_| build().from_filename(".env").overryde().load())?;

    Ok(())
}

#[test]
#[serial]
fn test_load_builder_path() -> Result<(), Box<dyn Error>> {
    check_missing_fails_load(|p| build().from_path(p).load())?;
    check_missing_optional_load(|p| build().from_path(p).optional().load())?;
    check_normal_load(|p| build().from_path(p).load())?;
    check_override_load(|p| build().from_path(p).overryde().load())?;

    Ok(())
}

#[test]
#[serial]
fn test_load_builder_read() -> Result<(), Box<dyn Error>> {
    check_normal_load(|p| {
        build()
            .from_read(&mut File::open(p).expect("Provided path is missing"))
            .load()
    })?;
    check_override_load(|p| {
        build()
            .from_read(&mut File::open(p).expect("Provided path is missing"))
            .overryde()
            .load()
    })?;

    Ok(())
}

// #[test]
// #[serial]
// fn test_iter_default() -> Result<(), Box<dyn Error>> {
//     check_missing_fails_iter(|_| build().iter())?;
//     check_missing_optional_iter(|_| build().optional().iter())?;
//     check_normal_iter(|_| build().iter())?;
//     // Note: There is no override test as this is a function of the loader.
//
//     Ok(())
// }

// TODO: remaining iter use cases

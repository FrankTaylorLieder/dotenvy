mod common;

use dotenvy::*;

use std::{env, error::Error, result::Result};

use crate::common::*;

#[test]
fn test_missing_fails() -> Result<(), Box<dyn Error>> {
    let dir = tempdir_without_dotenv()?;

    assert!(dotenv().is_err());

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

#[test]
fn test_missing_optional() -> Result<(), Box<dyn Error>> {
    let dir = tempdir_without_dotenv()?;

    dotenv_optional()?;

    assert!(env::var("TESTKEY").is_err());
    assert_eq!(env::var("EXISTING")?, "from_env");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}

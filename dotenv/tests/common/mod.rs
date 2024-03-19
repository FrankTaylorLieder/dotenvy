use rand::distributions::{Alphanumeric, DistString};
use std::fs::File;
use std::io::prelude::*;
use std::{env, io};
use tempfile::{tempdir, TempDir};

pub const EXISTING: &str = "EXISTING";
pub const TESTKEY: &str = "TESTKEY";
pub const CLEANENV: &str = "CLEANENV";

pub fn tempdir_with_dotenv(dotenv_text: &str) -> io::Result<TempDir> {
    env::set_var(EXISTING, "from_env");
    let dir = tempdir()?;
    env::set_current_dir(dir.path())?;
    let dotenv_path = dir.path().join(".env");
    let mut dotenv_file = File::create(dotenv_path)?;
    dotenv_file.write_all(dotenv_text.as_bytes())?;
    dotenv_file.sync_all()?;
    Ok(dir)
}

#[allow(dead_code)]
pub fn tempdir_without_dotenv() -> io::Result<TempDir> {
    env::set_var(EXISTING, "from_env");
    let dir = tempdir()?;
    env::set_current_dir(dir.path())?;
    Ok(dir)
}

#[allow(dead_code)]
pub fn make_test_dotenv() -> io::Result<TempDir> {
    tempdir_with_dotenv(&format!(
        "{TESTKEY}=test_val\n{TESTKEY}=test_val_overridden\n{EXISTING}=from_file"
    ))
}

pub struct CleanEnv {
    id: String,
}

impl CleanEnv {
    pub fn new() -> Self {
        let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        println!("Starting a clean env: {}", id);
        env::set_var(CLEANENV, id.clone());
        CleanEnv { id }
    }
}

impl Drop for CleanEnv {
    fn drop(&mut self) {
        env::remove_var(EXISTING);
        env::remove_var(TESTKEY);

        match env::var(CLEANENV) {
            Err(_) => panic!("CleanEnv missing id: {}", self.id),
            Ok(ce) => {
                assert_eq!(
                    ce, self.id,
                    "Incorrect CleanEnv being closed. Expected: {}, found: {}",
                    self.id, ce,
                );
            }
        }

        env::remove_var(CLEANENV);

        println!("Cleaned up the environment: {}", self.id);
    }
}

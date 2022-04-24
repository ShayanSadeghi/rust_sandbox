use assert_cmd::prelude::*; // add methods on commands
use predicates::prelude::*; // used for writing assertions
use std::process::Command; // run programs

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .assert()
        .success() // check program is running
        .stdout(predicate::str::contains("Meow!")); // check program contains a string
    Ok(())
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure();

    Ok(())
}

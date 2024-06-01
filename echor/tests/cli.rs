use assert_cmd::Command;
use predicates::prelude::*;
// use pretty_assertions::assert_eq;
use anyhow::Result;

use std::fs;

#[test]
fn test_ls() {
    use std::process::Command;
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    println!("{:?}", res);
    assert!(res.is_ok());
}

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn test_arg_ok() -> Result<()> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hi").assert().success();
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;

    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn hello1() -> Result<()> {
    run(
        &["Hello there"], 
        "tests/expected/hello1.txt",
    )
}

#[test]
fn hello2() -> Result<()> {
    run(
        &["Hello", "there"], 
        "tests/expected/hello2.txt"
    )
}

#[test]
fn hello1_no_newline() -> Result<()> {
    run(
        &["Hello there", "-n"], 
        "tests/expected/hello1.n.txt",
    )
}

#[test]
fn hello2_no_newline() -> Result<()> {
    run(
        &["-n", "Hello", "there"], 
        "tests/expected/hello2.n.txt"
    )
}

#[test]
fn bad_args() -> Result<()> {
    Command::cargo_bin("echor")?
        .arg("-x")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: unexpected argument '-x' found"
        ));
    Ok(())
}

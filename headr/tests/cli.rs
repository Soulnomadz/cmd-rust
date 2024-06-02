use assert_cmd::Command;
use anyhow::Result;
use predicates::prelude::*;
use rand::Rng;
use pretty_assertions::assert_eq;

// use headr::open;

const PRG: &str = "headr";
const EMPTY: &str = "./tests/inputs/empty.txt";
const ONE: &str = "./tests/inputs/one.txt";
const TWO: &str = "./tests/inputs/two.txt";
const THREE: &str = "./tests/inputs/three.txt";
const TWELVE: &str = "./tests/inputs/twelve.txt";

fn get_random_string() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

fn gen_bad_filename() -> String {
    loop {
        let filename = get_random_string();
        if std::fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

fn run(args: &[&str], expected: &str) -> Result<()> {
    let expected = std::fs::read_to_string(expected)?;

    let output = Command::cargo_bin(PRG)?
        .args(args)
        .output()?;

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout), 
        expected,
    );

    Ok(())
}

#[test]
fn test_bad_bytes() -> Result<()> {
    let bad = get_random_string();
    let expected = format!("error: invalid value '{}' for '--bytes <BYTES>'", bad);

    Command::cargo_bin("headr")?
        .args(["-c", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn test_bad_lines() -> Result<()> {
    let bad = get_random_string();
    let expected = format!("error: invalid value '{}' for '--lines <LINES>'", bad);

    Command::cargo_bin("headr")?
        .args(["-n", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn test_bytes_and_lines() -> Result<()> {
    let expected = "error: the argument '--lines <LINES>' \
    cannot be used with '--bytes <BYTES>'";

    Command::cargo_bin(PRG)?
        .args(["-n", "1", "-c", "1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));

    Ok(())
}

#[test]
fn test_bad_file() -> Result<()> {
    let bad_file = gen_bad_filename();
    let expected = format!("{bad_file}: .* [(]os error 2[)]");

    Command::cargo_bin(PRG)?
     .args([&bad_file])
     .assert()
     //Warn: this would be successful
     //.failure()  
     .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

#[test]
fn test_empty_file() -> Result<()>{
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn test_empty_n() -> Result<()> {
    run(&["-n2", EMPTY], "tests/expected/empty.txt.n2.out")
}
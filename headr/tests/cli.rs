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
const CN: &str = "./tests/inputs/cn.txt";

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
    // Warnings: read_to_string will fail when split utf8
    // let expected = std::fs::read_to_string(expected)?;
    let mut buf: Vec<u8> = Vec::new();
    headr::open(expected)?
        .read_to_end(&mut buf);
    let expected = String::from_utf8_lossy(&buf);

    let output = Command::cargo_bin(PRG)?
        .args(args)
        .output()?;

    // dbg!(&output);
    assert!(&output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout), 
        expected,
    );

    Ok(())
}

fn run_stdin(
    args: &[&str], 
    input: &str, 
    expected: &str,
) -> Result<()> {
    // let expected = std::fs::read_to_string(expected)?;
    let mut buf: Vec<u8> = Vec::new();
    headr::open(expected)?
        .read_to_end(&mut buf);
    let expected = String::from_utf8_lossy(&buf);

    let input = std::fs::read_to_string(input)?;

    let output = Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
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

#[test]
fn test_empty_c() -> Result<()> {
    run(&["-c2", EMPTY], "tests/expected/empty.txt.c2.out")
}


#[test]
fn one() -> Result<()> {
    run(&[ONE], "tests/expected/one.txt.out")
}

#[test]
fn one_n2() -> Result<()> {
    run(&[ONE, "-n", "2"], "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4() -> Result<()> {
    run(&[ONE, "-n", "4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c1() -> Result<()> {
    run(&[ONE, "-c", "1"], "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2() -> Result<()> {
    run(&[ONE, "-c", "2"], "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4() -> Result<()> {
    run(&[ONE, "-c", "4"], "tests/expected/one.txt.c4.out")
}

#[test]
fn one_stdin() -> Result<()> {
    run_stdin(&[], ONE, "tests/expected/one.txt.out")
}

#[test]
fn one_n2_stdin() -> Result<()> {
    run_stdin(&["-n", "2"], ONE, "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4_stdin() -> Result<()> {
    run_stdin(&["-n", "4"], ONE, "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c1_stdin() -> Result<()> {
    run_stdin(&["-c", "1"], ONE, "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2_stdin() -> Result<()> {
    run_stdin(&["-c", "2"], ONE, "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4_stdin() -> Result<()> {
    run_stdin(&["-c", "4"], ONE, "tests/expected/one.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn two() -> Result<()> {
    run(&[TWO], "tests/expected/two.txt.out")
}

#[test]
fn two_n2() -> Result<()> {
    run(&[TWO, "-n", "2"], "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4() -> Result<()> {
    run(&[TWO, "-n", "4"], "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2() -> Result<()> {
    run(&[TWO, "-c", "2"], "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4() -> Result<()> {
    run(&[TWO, "-c", "4"], "tests/expected/two.txt.c4.out")
}

#[test]
fn two_stdin() -> Result<()> {
    run_stdin(&[], TWO, "tests/expected/two.txt.out")
}

#[test]
fn two_n2_stdin() -> Result<()> {
    run_stdin(&["-n", "2"], TWO, "tests/expected/two.txt.n2.out")
}

#[test]
fn two_n4_stdin() -> Result<()> {
    run_stdin(&["-n", "4"], TWO, "tests/expected/two.txt.n4.out")
}

#[test]
fn two_c2_stdin() -> Result<()> {
    run_stdin(&["-c", "2"], TWO, "tests/expected/two.txt.c2.out")
}

#[test]
fn two_c4_stdin() -> Result<()> {
    run_stdin(&["-c", "4"], TWO, "tests/expected/two.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn three() -> Result<()> {
    run(&[THREE], "tests/expected/three.txt.out")
}

#[test]
fn three_n2() -> Result<()> {
    run(&[THREE, "-n", "2"], "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4() -> Result<()> {
    run(&[THREE, "-n", "4"], "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2() -> Result<()> {
    run(&[THREE, "-c", "2"], "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4() -> Result<()> {
    run(&[THREE, "-c", "4"], "tests/expected/three.txt.c4.out")
}

#[test]
fn three_stdin() -> Result<()> {
    run_stdin(&[], THREE, "tests/expected/three.txt.out")
}

#[test]
fn three_n2_stdin() -> Result<()> {
    run_stdin(&["-n", "2"], THREE, "tests/expected/three.txt.n2.out")
}

#[test]
fn three_n4_stdin() -> Result<()> {
    run_stdin(&["-n", "4"], THREE, "tests/expected/three.txt.n4.out")
}

#[test]
fn three_c2_stdin() -> Result<()> {
    run_stdin(&["-c", "2"], THREE, "tests/expected/three.txt.c2.out")
}

#[test]
fn three_c4_stdin() -> Result<()> {
    run_stdin(&["-c", "4"], THREE, "tests/expected/three.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn twelve() -> Result<()> {
    run(&[TWELVE], "tests/expected/twelve.txt.out")
}

#[test]
fn twelve_n2() -> Result<()> {
    run(&[TWELVE, "-n", "2"], "tests/expected/twelve.txt.n2.out")
}

#[test]
fn twelve_n4() -> Result<()> {
    run(&[TWELVE, "-n", "4"], "tests/expected/twelve.txt.n4.out")
}

#[test]
fn twelve_c2() -> Result<()> {
    run(&[TWELVE, "-c", "2"], "tests/expected/twelve.txt.c2.out")
}

#[test]
fn twelve_c4() -> Result<()> {
    run(&[TWELVE, "-c", "4"], "tests/expected/twelve.txt.c4.out")
}

#[test]
fn twelve_stdin() -> Result<()> {
    run_stdin(&[], TWELVE, "tests/expected/twelve.txt.out")
}

#[test]
fn twelve_n2_stdin() -> Result<()> {
    run_stdin(&["-n", "2"], TWELVE, "tests/expected/twelve.txt.n2.out")
}

#[test]
fn twelve_n4_stdin() -> Result<()> {
    run_stdin(&["-n", "4"], TWELVE, "tests/expected/twelve.txt.n4.out")
}

#[test]
fn twelve_c2_stdin() -> Result<()> {
    run_stdin(&["-c", "2"], TWELVE, "tests/expected/twelve.txt.c2.out")
}

#[test]
fn twelve_c4_stdin() -> Result<()> {
    run_stdin(&["-c", "4"], TWELVE, "tests/expected/twelve.txt.c4.out")
}

// --------------------------------------------------
#[test]
fn multiple_files() -> Result<()> {
    run(&[EMPTY, ONE, TWO, THREE, TWELVE], "tests/expected/all.out")
}

#[test]
fn multiple_files_n2() -> Result<()> {
    run(
        &[EMPTY, ONE, TWO, THREE, TWELVE, "-n", "2"],
        "tests/expected/all.n2.out",
    )
}

#[test]
fn multiple_files_n4() -> Result<()> {
    run(
        &["-n", "4", EMPTY, ONE, TWO, THREE, TWELVE],
        "tests/expected/all.n4.out",
    )
}

#[test]
fn multiple_files_c1() -> Result<()> {
    run(
        &[EMPTY, ONE, TWO, THREE, TWELVE, "-c", "1"],
        "tests/expected/all.c1.out",
    )
}

#[test]
fn multiple_files_c2() -> Result<()> {
    run(
        &[EMPTY, ONE, TWO, THREE, TWELVE, "-c", "2"],
        "tests/expected/all.c2.out",
    )
}

#[test]
fn multiple_files_c4() -> Result<()> {
    run(
        &["-c", "4", EMPTY, ONE, TWO, THREE, TWELVE],
        "tests/expected/all.c4.out",
    )
}

#[test]
fn cn_c1() -> Result<()> {
    run(
        &[CN, "-c", "1"],
        "tests/expected/cn.txt.c1.out",
    )
}

#[test]
fn cn_c2() -> Result<()> {
    run(
        &[CN, "-c", "2"],
        "tests/expected/cn.txt.c2.out",
    )
}

#[test]
fn cn_c4() -> Result<()> {
    run(
        &[CN, "-c", "4"],
        "tests/expected/cn.txt.c4.out",
    )
}

#[test]
fn cn_n2() -> Result<()> {
    run(
        &[CN, "-n", "2"],
        "tests/expected/cn.txt.n2.out",
    )
}

#[test]
fn cn_n4() -> Result<()> {
    run(
        &[CN, "-n", "4"],
        "tests/expected/cn.txt.n4.out",
    )
}

#[test]
fn cn() -> Result<()> {
    run(
        &[CN],
        "tests/expected/cn.txt.out",
    )
}

use assert_cmd::Command;
use predicates::prelude::*;
use anyhow::Result;

#[test]
fn file_not_exists() -> Result<()>{
    Command::cargo_bin("grrs")?
        .args(["aaa", "bbbb"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot read bbbb"));

    Ok(())
}
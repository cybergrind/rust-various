use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_basic_file_count() {
    let mut cmd = Command::cargo_bin("wordcount").unwrap();

    cmd.arg("tests/fixtures/simple.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("3"))
        .stdout(predicate::str::contains("11"))
        .stdout(predicate::str::contains("simple.txt"));
}

#[test]
fn test_multiple_files() {
    let mut cmd = Command::cargo_bin("wordcount").unwrap();

    cmd.args(&["tests/fixtures/simple.txt", "tests/fixtures/empty.txt"])
        .assert()
        .success()
        .stdout(predicate::str::contains("total"));
}

#[test]
fn test_nonexistent_file() {
    let mut cmd = Command::cargo_bin("wordcount").unwrap();

    cmd.arg("nonexistent.txt")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Some files could not be processed.\n",
        ));
}

#[test]
fn test_lines_only_flag() {
    let mut cmd = Command::cargo_bin("wordcount").unwrap();

    cmd.args(&["--lines", "tests/fixtures/simple.txt"])
        .assert()
        .success()
        .stdout(predicate::str::contains("3"));
}

#[test]
fn test_empty_file() {
    let mut cmd = Command::cargo_bin("wordcount").unwrap();

    cmd.arg("tests/fixtures/empty.txt")
        .assert()
        .success()
        .stdout(predicate::str::contains("0"));
}

#[test]
fn test_no_files_provided() {
    let mut cmd = Command::cargo_bin("wordcount").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No files provided"));
}

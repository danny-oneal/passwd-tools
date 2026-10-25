use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::cargo_bin("passwd-tools").unwrap()
}

#[test]
fn filters_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("words.txt");
    std::fs::write(&path, "a\nabc\nabcd\n").unwrap();
    cmd()
        .args(["min-len", "3"])
        .arg(&path)
        .assert()
        .success()
        .stdout("abc\nabcd\n");
}

#[test]
fn reads_stdin_by_default_and_with_dash() {
    for args in [&["min-len", "2"][..], &["min-len", "2", "-"][..]] {
        cmd()
            .args(args)
            .write_stdin("a\nab\n")
            .assert()
            .success()
            .stdout("ab\n");
    }
}

#[test]
fn missing_file_reports_path_and_fails() {
    cmd()
        .args(["min-len", "1", "/nonexistent/file"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot open /nonexistent/file"));
}

#[test]
fn rejects_non_numeric_length() {
    cmd().args(["min-len", "x"]).assert().code(2);
}

use assert_cmd::Command;


#[test]
fn case_sensitive_run_no_color() {
    let mut cmd = Command::cargo_bin("minigrep").unwrap();
    cmd.args(&[
            "are",
            "tests/input/case_sensitive_run.txt",
            "--do-print-color",
            "false",
    ])
        .assert()
        .success()
        .stdout(predicates::str::contains(r"Waffles are good"));
}
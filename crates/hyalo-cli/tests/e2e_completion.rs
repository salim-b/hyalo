mod common;

use common::hyalo_no_hints;

#[test]
fn completion_bash_produces_output() {
    let output = hyalo_no_hints()
        .args(["completion", "bash"])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("hyalo"),
        "bash completions should reference the binary name"
    );
}

#[test]
fn completion_zsh_produces_output() {
    let output = hyalo_no_hints()
        .args(["completion", "zsh"])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("hyalo"),
        "zsh completions should reference the binary name"
    );
}

#[test]
fn completion_fish_produces_output() {
    let output = hyalo_no_hints()
        .args(["completion", "fish"])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("hyalo"),
        "fish completions should reference the binary name"
    );
}

#[test]
fn completion_elvish_produces_output() {
    let output = hyalo_no_hints()
        .args(["completion", "elvish"])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("hyalo"),
        "elvish completions should reference the binary name"
    );
}

#[test]
fn completion_powershell_produces_output() {
    let output = hyalo_no_hints()
        .args(["completion", "powershell"])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "stderr: {stderr}");

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("hyalo"),
        "powershell completions should reference the binary name"
    );
}

#[test]
fn completion_missing_shell_fails() {
    let output = hyalo_no_hints().args(["completion"]).output().unwrap();

    assert!(
        !output.status.success(),
        "completion without a shell argument should fail"
    );
}

#[test]
fn completion_invalid_shell_fails() {
    let output = hyalo_no_hints()
        .args(["completion", "invalid-shell"])
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "completion with an invalid shell should fail"
    );
}

#[test]
fn completion_listed_in_help() {
    let output = hyalo_no_hints().arg("-h").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(
        stdout.contains("completion"),
        "completion command should appear in help output"
    );
}

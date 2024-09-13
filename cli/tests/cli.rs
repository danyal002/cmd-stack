use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_add_and_list() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;

    let mut add_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    add_cmd.env("CMD_STACK_DIRECTORY", dir.path()).arg("add").arg("ls").arg("--alias=\"test\"");
    add_cmd.assert().success().stdout(predicate::str::contains("successfully"));

    Ok(())
}

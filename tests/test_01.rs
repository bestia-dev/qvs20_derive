use std::fs;
use std::process::Command;

/// run cargo expand, compare with saved file
#[test]
fn test_01_expand_compare() {
    // cargo expand --example example_01
    let args = vec!["expand", "--example", "example_01"];
    let output = Command::new("cargo").args(args).output().unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    let expand_01 = fs::read_to_string("sample_data/expand_01.txt").unwrap();
    assert_eq!(output, expand_01);
}

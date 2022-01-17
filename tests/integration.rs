fn run_command(file_path: &str) -> Vec<String> {
    let mut cmd = assert_cmd::Command::cargo_bin("transaction-engine").unwrap();
    let assert = cmd.arg(file_path).assert().success();
    let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();
    let mut output = stdout
        .trim()
        .lines()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    output.sort_unstable();
    output
}

#[test]
fn test_sample_data() {
    let mut sorted_expected = vec![
        "client,available,held,total,locked",
        "2,0.0,2.0,2.0,false",
        "3,3.0,0,3.0,false",
        "1,0.5,0.0,0.5,true",
    ];
    sorted_expected.sort_unstable();

    assert_eq!(sorted_expected, run_command("./tests/test_txns.csv"));
}

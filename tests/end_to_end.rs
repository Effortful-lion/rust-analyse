use std::process::Command;

#[test]
fn cli_without_arguments_prints_usage() {
    let binary = env!("CARGO_BIN_EXE_grammar-analyse");
    let output = Command::new(binary).output().expect("run binary");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stderr.contains("用法"));
    assert!(stderr.contains("grammar-analyse <输入文件>"));
}

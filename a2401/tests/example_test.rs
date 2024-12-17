use assert_cmd::Command;
use std::fs::read_to_string;
use std::path::PathBuf;

#[test]
fn run_both() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //println!("{test_file_path:?}");
    let mut inp = root.clone();
    inp.push("inputs\\test.txt");
    let mut p1o = root.clone();
    let mut p2o = root.clone();
    p1o.push("inputs\\p1.txt");
    p2o.push("inputs\\p2.txt");
    let p1out = read_to_string(p1o)
        .expect("Failed to read the file")
        .replace("\r\n", "\n");
    let p2out = read_to_string(p2o)
        .expect("Failed to read the file")
        .replace("\r\n", "\n");

    Command::cargo_bin("a2401")
        .unwrap()
        .arg(&inp)
        .assert()
        .success()
        .stdout(p1out.clone() + &p2out);
    Command::cargo_bin("a2401")
        .unwrap()
        .arg(&inp)
        .arg("-p")
        .assert()
        .success()
        .stdout(p1out);
    Command::cargo_bin("a2401")
        .unwrap()
        .arg(&inp)
        .arg("-pp")
        .assert()
        .success()
        .stdout(p2out);
}

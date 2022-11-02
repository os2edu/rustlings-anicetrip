use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn check() {
    Command::cargo_bin("rustlings")
        .unwrap()
        .args(&["--nocapture","myverify"])
        // .current_dir("exercises/")
        .assert()
        .success()
        // .stdout("rust test 完成") // predicates::str::contains("总的题目数")
        ;
}



    let mut x = 100;
    let y = &mut x;
    *y += 100;
    *y += 1000;
    assert_eq!(x, 1200);
}
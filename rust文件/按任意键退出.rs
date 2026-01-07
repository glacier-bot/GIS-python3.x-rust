use std::process::Command;
let _ = Command::new("cmd")
       .args(&["/C", "pause"])
       .status();
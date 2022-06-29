use std::fs::File;
use std::process::{Command, Stdio};
use std::time::Duration;

fn main() {
    let child = Command::new("godot").args(&[""]).stdout(Stdio::piped()).spawn();

    if let Ok(child_process) = child {
        let mut stdout_file = File::create("stdout.txt").unwrap();
        std::io::copy(&mut child_process.stdout.unwrap(), &mut stdout_file).unwrap();
    }
    else {
        println!("Cannot open godot!");
        println!("{:#?}", child);
        std::thread::sleep(Duration::from_secs(10));
    }
}

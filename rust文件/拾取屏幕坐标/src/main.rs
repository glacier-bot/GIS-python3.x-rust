use core::time;

use uiautomation::inputs::Mouse;

fn main() {
    println!("开始拾取屏幕坐标...");
    println!("每3秒显示一次屏幕坐标，共显示60次");
    display_mouse_position(time::Duration::from_secs(3));
    pause();
}

fn display_mouse_position (d: time::Duration) {
    use std::thread::sleep;
    let mut n = 0;
    while n < 60 {
        let position = Mouse::get_cursor_pos().unwrap();
        println!("{}", format!("屏幕坐标{}/60--> x坐标: {}, y坐标: {}", n+1, position.get_x(), position.get_y()));
        sleep(d);
        n+=1;
    }
}

fn pause() {
    use std::process::Command;
    let _ = Command::new("cmd")
       .args(&["/C", "pause"])
       .status();
}
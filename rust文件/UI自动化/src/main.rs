use core::time;
use std::thread::sleep;
use time::Duration;
// use std::convert::identity;
use uiautomation::{inputs::{Keyboard, Mouse}, types::Point};

/// 根据xlsx文件传入的参数操作桌面
fn main () {
    let template_path = "读取屏幕坐标.xlsx";
    let workbook = umya_spreadsheet::reader::xlsx::read(template_path).unwrap();
    let sheet = workbook.get_sheet_by_name("Sheet1").unwrap();
    let sheet_max_row = sheet.get_highest_row();
    
    for i in 2..sheet_max_row+1 {
        let op_type = sheet.get_value(format!("B{}",i).as_str());
        let op_type_str = op_type.as_str();

        let text_content = sheet.get_value(format!("E{}",i).as_str());
        
        let _sleep_interval = sheet.get_value(format!("F{}",i).as_str()).parse::<u64>().expect(format!("第{}行输入了错误的时间间隔！", i-1).as_str());

        match op_type_str {
            "鼠标左键单击" | "鼠标右键单击" | "鼠标左键双击" => {
                let _coord_x = sheet.get_value(format!("C{}",i).as_str()).parse::<i32>().expect(format!("第{}行输入了错误的x坐标！", i-1).as_str());
                let _coord_y = sheet.get_value(format!("D{}",i).as_str()).parse::<i32>().expect(format!("第{}行输入了错误的y坐标！", i-1).as_str());
            },
            "键盘输入文本" => if text_content.len() == 0 {panic!("{}", format!("第{}行的文本不能为空！", i-1))}
            _ => panic!("第{}行输入了不正确的操作名称!", i-1)
        } 
    }

    for i in 2..sheet_max_row+1 {
        let op_type = sheet.get_value(format!("B{}",i).as_str());
        let op_type_str = op_type.as_str();
        let coord_x = sheet.get_value(format!("C{}",i).as_str()).parse::<i32>().unwrap_or_default();
        let coord_y = sheet.get_value(format!("D{}",i).as_str()).parse::<i32>().unwrap_or_default();
        let text_content = sheet.get_value(format!("E{}",i).as_str());
        let sleep_interval = sheet.get_value(format!("F{}",i).as_str()).parse::<u64>().unwrap_or_default();
        let duration = Duration::from_secs(sleep_interval);

        println!("开始执行操作{}：{}",i-1,op_type);
        work_once(op_type_str, coord_x, coord_y, text_content);
        sleep(duration);
        // println!("{}",format!("{op_type},{coord_x},{coord_y},{text_content},{sleep_interval}"))
    }
    // let duration = Duration::from_secs(3);
    // _display_mouse_position(duration);
    
    println!("done.");
    pause();
    
}

/// 按任意键继续
fn pause() {
    use std::process::Command;
    let _ = Command::new("cmd")
       .args(&["/C", "pause"])
       .status();
}

/// 完成一次操作
fn work_once (op_type_str: &str, coord_x: i32, coord_y: i32, text_content: String) {
    let mouse = Mouse::new().move_time(800);
    let keyboard = Keyboard::new().interval(100);
    match op_type_str {
        "鼠标左键单击" => mouse.click(&Point::new(coord_x, coord_y)).unwrap(),
        "鼠标右键单击" => mouse.right_click(&Point::new(coord_x, coord_y)).unwrap(),
        "鼠标左键双击" => mouse.double_click(&Point::new(coord_x, coord_y)).unwrap(),
        "键盘输入文本" => keyboard.send_text(text_content.as_str()).unwrap(),
        _ => {}
    }
}

/// 测试rust的鼠标自动化
fn _old_proj_main () {
    let mouse = Mouse::new().move_time(800);
    let duration = time::Duration::from_secs(5);
    _display_mouse_position(duration);
    mouse.move_to(&Point::new(1024, 768)).unwrap();
    mouse.right_click(&Point::new(200, 55)).unwrap();
}

/// 显示鼠标在屏幕上的位置
fn _display_mouse_position (d: time::Duration) {
    let mut n = 0;
    while n < 3 {
        let position = Mouse::get_cursor_pos().unwrap();
        println!("{}", format!("position:{position}"));
        sleep(d);
        n+=1;
    }
}
/// require: umya-spreadsheet = "2.3.3"
pub fn test_Excel() {
    let sheet_path = "/path/to/xx.xlsx"
    let mut book = umya_spreadsheet::reader::xlsx::read(sheet_path).unwrap();
    let sheet = book.get_sheet_by_name_mut("Sheet3").unwrap();

    // 如果area_sq_meter为0，则记录该行，稍后统一删除
    for i in 2..166 {
        if sheet.get_value(format!("D{}",i)).parse::<f64>().unwrap_or(0.0) == 0f64 {
            rows_to_remove.push(i as u32);
            // println!("Row {} marked for removal due to zero area.", i);
            continue;
        }
    }

    // 从后向前删除记录的行，避免删除时索引变化影响未处理的行
    if !rows_to_remove.is_empty() {
        rows_to_remove.sort_unstable();
        for row in rows_to_remove.iter().rev() {
            // 调用 remove_row 使用 u32 引用和删除数量为 1 
            let _ = sheet.remove_row(row, &1u32);

            // println!("Removed row {}.", row);
        }
    }

    // 删除辅助列
    sheet.remove_column("H", &4u32);

    // 保存修改后的文件
    let _ = umya_spreadsheet::writer::xlsx::write(&book, output);
}
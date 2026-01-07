/// require: polars = { version = "0.50.0", features = ["lazy", "temporal"] }
pub fn test_DataFrame() {
    use polars::prelude::*;
    let csv_path = "/path/to/xx.csv"
    let df = CsvReadOptions::default().with_has_header(true).try_into_reader_with_file_path(Some(input.into()))?.finish()?.lazy();
    // 按“地类名称”列分组，并计算面积总和
    let group_area = df.clone().group_by(["地类名称"]).agg([col("^.*(SHAPE_Area|Shape_Area).*$").sum()]);
    // 查找“地类名称”列中，包含“草地”的行
    let mask = col("地类名称").eq(lit("草地"));
    let filtered_df = df.clone().select([col("地类名称"),col("^.*(SHAPE_Area|Shape_Area).*$")]).filter(mask).collect()?;
    // 按“地类名称”列分组，并计算面积总和
    let group_area = df.clone().group_by(["地类名称"]).agg([col("^.*(SHAPE_Area|Shape_Area).*$").sum()]);
}
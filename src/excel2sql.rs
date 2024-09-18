pub mod excel2sql {
    use std::error::Error;
    use std::ffi::c_int;
    use std::fs;
    use std::path::Path;
    use chrono::{DateTime, Local};
    use xl::Workbook;
    use crate::auto_code::gen_code;

    pub fn convert(file_path: &str) -> Result<String, Box<dyn Error>> {
        fs::exists(file_path)?;
        let mut wb = Workbook::open(file_path)?;
        let sheets = wb.sheets();
        let sheet = sheets.get("Sheet1").unwrap();
        let mut rows = sheet.rows(&mut wb);
        let mut i = 0;
        let mut lines = Vec::new();
        for x in rows {
            i = i + 1;
            if i < 3 {
                continue;
            }
            let id = generate_unique_id(i); // 根据日期生成唯一id
            let string = format!("INSERT INTO `test`(`id`, `no`, `name`) VALUES ({}, '{}', '{}');",
                                 id,
                                 x[1].value.to_string().replace("\"", ""),
                                 x[2].value.to_string().replace("\"", "")
            );
            lines.push(string);
        }
        let output_dir = (file_path.replace("xlsx", "").replace("xlx", "") + "sql");
        let path = Path::new(&output_dir);
        fs::write(path, gen_code::vac2str(lines)).unwrap();

        Ok("success".to_owned())
    }

    /// 生成基于年月日和四位序列号的唯一 ID
    fn generate_unique_id(num: c_int) -> String {
        let date = format_timestamp();
        let sequence = format!("{:04}", num); // 生成四位数的序列号
        format!("{}{}", date, sequence)
    }

    /// 将时间戳转换为年月日的字符串形式
    fn format_timestamp() -> String {
        let local: DateTime<Local> = Local::now();
        let formatted_datetime = local.format("%Y%m%d%H%M%S").to_string();
        formatted_datetime
    }
}
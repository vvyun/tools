mod request;
mod auto_code;
mod dbutils;
mod config;
mod excel2sql;

use reqwest::Error;
use std::io::stdin;


fn main() -> Result<(), Error> {
    println!("请输入excel文件名开始执行");
    let mut str_buf = String::new();
    stdin().read_line(&mut str_buf).expect("error: unable to read user input");
    match excel2sql::excel2sql::convert(str_buf.replace("\r\n", "").as_str()) {
        Ok(res) => {
            println!("sql转化完成!");
        }
        Err(e) => {
            println!("error: {}", e);
        }
    }
    println!("输入enter退出系统!");
    let mut str_buf = String::new();
    stdin().read_line(&mut str_buf).expect("error: unable to read user input");
    Ok(())
}

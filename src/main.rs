mod request;
mod gencode;
mod dbutils;

use reqwest::Error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    // request::my_request::get_token().await?;

    gencode::gen_code::gen_all().expect("error");

    // 测试dbutils
    // dbutils::dbutils::list_tables().unwrap();

    Ok(())
}

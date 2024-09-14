mod request;
mod gencode;
mod dbutils;

use reqwest::Error;
extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;  // Optional. Only if the derive macro is used.

#[tokio::main]
async fn main() -> Result<(), Error> {
    // gui test
    gencode::gen_code::gen_all().unwrap();
    Ok(())
}

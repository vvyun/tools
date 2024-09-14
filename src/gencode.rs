/// 自动生成项目目录结构和pojo等代码
pub mod gen_code {
    use crate::dbutils::dbutils::DbOptWrapper;
    use std::fs;

    /// 生成code
    pub fn gen_all() -> std::io::Result<()> {
        gen_package()?;
        gen_code()?;
        Ok(())
    }

    /// 生成目录
    pub fn gen_package() -> std::io::Result<()> {
        let model_name = "myetc";
        let dir = "D:/tools/src/main/java/cn/net/aaa/";
        let dir = dir.to_owned() + model_name;
        let package_array = [
            "application/repository",
            "domain/repository",
            "domain/service",
            "infrastructure/bo",
            "infrastructure/repository/apprepo",
            "infrastructure/repository/domainrepo",
            "infrastructure/repository/persistent/mappers/mapper",
            "infrastructure/repository/persistent/po",
            "interfaces"
        ];
        for it_dir in package_array {
            let tep = dir.to_owned() + "/" + it_dir;
            mkdir(&tep)?;
        }
        Ok(())
    }

    pub fn gen_code() -> std::io::Result<()> {
        let wrapper = DbOptWrapper::new("mysql://root:123456@localhost:3306/test");
        let table_info = wrapper.unwrap().list_tables().unwrap();
        for items_tb in table_info {
            println!("{}, {}", items_tb.table_name, items_tb.table_comment)
            // TODO 获取表结构，根据表结构生成pojo等代码

        }
        Ok(())
    }

    fn mkdir(dir: &String) -> std::io::Result<()> {
        let x = fs::exists(dir.as_str())?;
        if !x {
            fs::create_dir_all(dir.as_str())?;
        }
        Ok(())
    }
}
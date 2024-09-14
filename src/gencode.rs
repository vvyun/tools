/// 自动生成项目目录结构和pojo等代码
pub mod gen_code {
    use std::fs;
    use crate::dbutils::dbutils::list_tables;

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
        let result = list_tables();
        let vec = result.unwrap();
        for x in vec {
            println!("{}, {}", x.table_name, x.table_comment)
        }
        // TODO 获取表结构，根据表结构生成pojo等代码
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
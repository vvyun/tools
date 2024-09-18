/// 自动生成项目目录结构和pojo等代码
pub mod gen_code {
    use crate::dbutils::dbutils::{DBColumn, DbOptWrapper, TableInfo};
    use std::collections::{HashMap, HashSet};
    use std::fs;
    use std::path::Path;
    /// 数据库url
    const URL: &'static str = "mysql://root:123456@127.0.0.1:3306/test";
    /// 数据库
    const DATABASE: &'static str = "test";
    /// 需要生成的表前缀
    const TB_PRE_FEX: &'static str = "some_";
    /// 包名
    const PACKAGE: &'static str = "cn.net.xxx";
    /// persistent包路径
    const BASE_DIR: &'static str = "xxx/xxx/xxx";
    /// MapperService 模板
    const TMP_MAPPER_SVC: &'static str = "package BASE_PACKAGE_NAME.infrastructure.repository.persistent.mappers;

import BASE_PACKAGE_NAME.infrastructure.repository.persistent.mappers.mapper.MAPPER_NAME;
import BASE_PACKAGE_NAME.infrastructure.repository.persistent.po.PO_NAME;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import org.springframework.stereotype.Service;

@Service(value = \"MAPPER_SERVICE_NAME\")
public class MAPPER_SERVICE_NAME extends ServiceImpl<MAPPER_NAME, PO_NAME> {

}
";
    /// Mapper 模板
    const TMP_MAPPER: &'static str = "package BASE_PACKAGE_NAME.infrastructure.repository.persistent.mappers.mapper;

import BASE_PACKAGE_NAME.infrastructure.repository.persistent.po.PO_NAME;
import com.baomidou.mybatisplus.core.mapper.BaseMapper;

public interface MAPPER_NAME extends BaseMapper<PO_NAME> {

}
";

    /// 生成code
    pub fn gen_all() -> std::io::Result<()> {
        gen_package()?;
        gen_code()?;
        Ok(())
    }

    /// 按项目规范生成目录
    pub fn gen_package() -> std::io::Result<()> {
        let dir = BASE_DIR.to_string();
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

    /// 根据数据库表生成po/mapper/mapperService
    pub fn gen_code() -> std::io::Result<()> {
        let wrapper = DbOptWrapper::new(URL, DATABASE)
            .expect("init db pool error");
        let table_info = wrapper.list_tables().unwrap();
        for items_tb in table_info {
            if items_tb.table_name.starts_with(TB_PRE_FEX) {
                let column = wrapper.get_table_info(items_tb.table_name.as_str()).unwrap();
                column_export_process(&items_tb, PACKAGE, column);
            }
        }
        Ok(())
    }

    fn column_export_process(table: &TableInfo, base_package: &str, db_columns: Vec<DBColumn>) {
        let mut special_types = HashSet::new();
        let table_name = table.table_name.as_str();
        let po_name = to_camel_case_upper(&table_name).to_owned() + "PO";
        let table_comment = table.table_comment.as_str();
        let mut lines = Vec::new();
        lines.push(format!("package {}.infrastructure.repository.persistent.po;", base_package));
        lines.push("
import xxx.BaseEntity;
import com.baomidou.mybatisplus.annotation.TableName;
import lombok.Data;
import lombok.EqualsAndHashCode;

import javax.persistence.Column;
import java.io.Serializable;
".to_string());

        lines.push(format!("/** {} */
@Data
@EqualsAndHashCode(callSuper = true)
@TableName(\"{}\")
public class {} extends BaseEntity<Long, {}> implements Serializable {{",
                           table_comment, table_name, po_name.as_str(), po_name.as_str()));

        for db_column in db_columns {
            let column_name = db_column.column_name.as_str();
            if ["key_id", "del_status", "tenant_id", "add_request_id", "modify_request_id", "add_user_id", "modify_user_id", "add_time", "modify_time"]
                .contains(&column_name) {
                continue;
            }
            let sf_type = db_column.data_type.as_str();
            special_types.insert(type_convert(sf_type));
            let comment = db_column.column_comment.as_str();
            lines.push(String::from("    /**"));
            lines.push(format!("    * {}", comment));
            lines.push(String::from("    */"));
            lines.push(format!("    @Column(name = \"{}\", columnDefinition = \"{}\")", column_name, comment));
            lines.push(format!("    private {} {};", type_convert(sf_type), to_camel_case(&column_name, '_')));
        }
        lines.push(String::from("}"));
        lines.push(String::from(""));

        if special_types.contains("BigDecimal") {
            lines.insert(1, String::from("import java.math.BigDecimal;"));
        }
        if special_types.contains("Date") {
            lines.insert(1, String::from("import java.util.Date;"));
        }
        let base_dir = BASE_DIR.to_string();
        // PO 文件
        let dir1 = String::from(base_dir.as_str().to_owned() + "/po/" + &po_name + ".java");
        let po_file_path = Path::new(dir1.as_str());
        fs::write(po_file_path, vac2str(lines)).unwrap();

        let po_name_no_po = &po_name[..po_name.len() - 2];
        let mapper_name = format!("I{}Mapper", po_name_no_po);
        let mapper_svc_name = format!("{}MapperService", po_name_no_po);

        // IMapper MapperService 文件
        let dir1 = String::from(base_dir.as_str().to_owned() + "/mappers/" + &mapper_svc_name + ".java");
        let i_demo_mapper_service_file_path = Path::new(dir1.as_str());
        let mapper_svc_content = temp_convert(TMP_MAPPER_SVC, &mapper_name, &mapper_svc_name, &po_name, base_package);
        fs::write(i_demo_mapper_service_file_path, vac2str(mapper_svc_content)).unwrap();

        let dir1 = String::from(base_dir.as_str().to_owned() + "/mappers/mapper/" + &mapper_name + ".java");
        let i_demo_mapper_file_path = Path::new(dir1.as_str());
        let mapper_content = temp_convert(TMP_MAPPER, &mapper_name, &mapper_svc_name, &po_name, base_package);
        fs::write(i_demo_mapper_file_path, vac2str(mapper_content)).unwrap();
    }

    fn to_camel_case(input: &str, separator: char) -> String {
        input.split(separator)
            .enumerate()
            .map(|(i, s)|
                if i == 0 { s.to_lowercase() } else {
                    s.chars().enumerate().map(|(j, c)| {
                        if j == 0 {
                            c.to_uppercase().collect::<String>()
                        } else {
                            c.to_string()
                        }
                    }).collect::<String>()
                })
            .collect::<Vec<_>>()
            .join("")
    }

    /// 将 snake_case 转换为 CamelCase
    fn to_camel_case_upper(s: &str) -> String {
        s.split('_')
            .map(|part| {
                part.chars().enumerate().map(|(j, c)| {
                    if j == 0 {
                        c.to_uppercase().collect::<String>()
                    } else {
                        c.to_string()
                    }
                }).collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// 转换数据库类型为代码类型
    fn type_convert(db_type: &str) -> String {
        let mut mappings = HashMap::new();
        mappings.insert("date", "Date");
        mappings.insert("datetime", "Date");
        mappings.insert("longtext", "String");
        mappings.insert("text", "String");
        mappings.insert("smallint", "Integer");
        mappings.insert("varchar", "String");
        mappings.insert("char", "String");
        mappings.insert("tinyint", "Integer");
        mappings.insert("decimal", "BigDecimal");
        mappings.insert("bigint", "Long");
        mappings.insert("int", "Integer");
        mappings.insert("timestamp", "Date");

        mappings.get(db_type.clone()).cloned().expect("unknown type").to_string()
    }

    /// 将模板字符串中的占位符替换为实际值
    fn temp_convert(tmp_str: &str, mapper_name: &str, mapper_svc_name: &str, po_name: &str, base_package: &str) -> Vec<String> {
        let mut temp = tmp_str.to_string();
        temp = temp.replace("MAPPER_NAME", mapper_name);
        temp = temp.replace("MAPPER_SERVICE_NAME", mapper_svc_name);
        temp = temp.replace("PO_NAME", po_name);
        temp = temp.replace("BASE_PACKAGE_NAME", base_package);
        temp.split("\n").map(|s| s.to_owned()).collect::<Vec<_>>()
    }

    pub fn vac2str(list: Vec<String>) -> String {
        let mut a = String::from("");
        let mut first = true;
        for x in list {
            if !first {
                a = a.as_str().to_owned() + "\r\n" + x.as_str();
            } else {
                first = false;
                a = x;
            }
        }
        a.to_string()
    }

    fn mkdir(dir: &String) -> std::io::Result<()> {
        let x = fs::exists(dir.as_str())?;
        if !x {
            fs::create_dir_all(dir.as_str())?;
        }
        Ok(())
    }
}
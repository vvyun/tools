/// db tools
pub mod dbutils {
    use mysql::prelude::*;
    use mysql::*;


    // 表信息
    #[derive(Debug, PartialEq, Eq)]
    pub struct TableInfo {
        pub(crate) table_name: String,
        pub(crate) table_comment: String,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct DBColumn {
        /**
         * 字段名
         */
        pub(crate) column_name: String,
        /**
         * 字段类型
         */
        pub(crate) column_type: String,
        /**
         * 字段类型
         */
        pub(crate) data_type: String,
        /**
         * 是否为空
         */
        pub(crate) is_nullable: String,
        /**
         * 注释
         */
        pub(crate) column_comment: String,
    }

    pub struct DbOptWrapper {
        url: String,
        db_name: String,
        pool: Pool,
    }

    impl DbOptWrapper {
        pub fn new(url: &str, db_name: &str) -> Result<DbOptWrapper> {
            Ok(DbOptWrapper {
                url: url.to_string(),
                db_name: db_name.to_string(),
                pool: Pool::new(url)?
            })
        }

        pub fn get_url(&self) -> &str {
            self.url.as_str()
        }

        pub fn get_db(&self) -> &str {
            self.db_name.as_str()
        }

        pub fn list_tables(&self) -> std::result::Result<Vec<TableInfo>, Box<dyn std::error::Error>> {
            let mut conn = self.pool.get_conn()?;
            let sql = "select table_name,table_comment from INFORMATION_SCHEMA.TABLES where TABLE_SCHEMA = '".to_owned() + self.get_db() + "'";
            // 输出到Vec
            let res_list = conn
                .query_map(
                    sql,
                    |(table_name, table_comment)| {
                        TableInfo { table_name, table_comment }
                    },
                )?;

            Ok(res_list)
        }

        pub fn get_table_info(&self, table_name: &str) -> std::result::Result<Vec<DBColumn>, Box<dyn std::error::Error>> {
            let mut conn = self.pool.get_conn()?;
            let sql = "select column_name,column_type,data_type,is_nullable,column_comment from information_schema.COLUMNS where TABLE_SCHEMA = '".to_owned() + self.get_db() + "' and TABLE_NAME = '" + table_name + "'";
            // 输出到Vec
            let res_list = conn
                .query_map(
                    sql,
                    |(column_name,
                         column_type,
                         data_type,
                         is_nullable,
                         column_comment)|
                        {
                        DBColumn {
                            column_name,
                            column_type,
                            data_type,
                            is_nullable,
                            column_comment,
                        }
                    },
                )?;

            Ok(res_list)
        }
    }
}
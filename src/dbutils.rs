/// db tools
pub mod dbutils {
    use mysql::prelude::*;
    use mysql::*;

    #[derive(Debug, PartialEq, Eq)]
    pub struct TableInfo {
        pub(crate) table_name: String,
        pub(crate) table_comment: String,
    }

    pub fn list_tables() -> std::result::Result<Vec<TableInfo>, Box<dyn std::error::Error>> {
        let url = "mysql://root:123456@localhost:3306/test";
        let pool = Pool::new(url)?;
        let mut conn = pool.get_conn()?;

        let sql = "select table_name,table_comment from INFORMATION_SCHEMA.TABLES where TABLE_SCHEMA = 'assets'";
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

}
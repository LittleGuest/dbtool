#![feature(lazy_cell)]

use std::{collections::HashMap, sync::Mutex};

use serde::{Deserialize, Serialize};
use sqlx::{
    any::AnyPoolOptions, Any, AnyPool, MySql, MySqlPool, PgPool, Pool, Postgres, Row, Sqlite,
    SqlitePool,
};

mod mysql;

lazy_static::lazy_static! {
    static ref DBConnMap: Mutex<HashMap<u64, DbConn>> = Mutex::new(HashMap::new());
}

pub type DResult<T, E = &'static str> = std::result::Result<T, E>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Driver {
    Mysql,
    Postgres,
    Sqlite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DbConn {
    #[serde(default)]
    id: u64,
    driver: Driver,
    name: String,
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

impl DbConn {
    fn url(&self) -> String {
        match self.driver {
            Driver::Mysql => format!(
                "mysql://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database
            ),
            Driver::Postgres => format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database
            ),
            Driver::Sqlite => format!("sqlite://{}", self.database),
        }
    }
}

pub async fn conn_any(url: &str) -> DResult<Pool<Any>> {
    sqlx::any::install_default_drivers();
    AnyPool::connect(url)
        .await
        .map_err(|_| "SQL connect failed")
}

pub async fn conn_mysql(url: &str) -> DResult<Pool<MySql>> {
    sqlx::any::install_default_drivers();
    MySqlPool::connect(url)
        .await
        .map_err(|_| "MySQL connect failed")
}

pub async fn conn_pg(url: &str) -> DResult<Pool<Postgres>> {
    sqlx::any::install_default_drivers();
    PgPool::connect(url)
        .await
        .map_err(|_| "MySQL connect failed")
}

pub async fn conn_sqlite(url: &str) -> DResult<Pool<Sqlite>> {
    sqlx::any::install_default_drivers();
    SqlitePool::connect(url)
        .await
        .map_err(|_| "MySQL connect failed")
}

#[derive(Debug, Serialize, Deserialize)]
struct TableColumn {
    table_schema: String,
    table_name: String,
    column_name: Option<String>,
    ordinal_position: u32,
    column_default: Option<String>,
    /// YES/NO
    is_nullable: String,
    data_type: Option<String>,
    /// 对于字符串列，最大长度（以字符为单位）。
    character_maximum_length: Option<i64>,
    /// 对于字符串列，最大长度（字节）。
    character_octet_length: Option<i64>,
    /// 对于数值列，数值精度。
    numeric_precision: Option<u64>,
    /// 对于数字列，数字刻度。
    numeric_scale: Option<u64>,
    /// 对于时间列，精确度为零点几秒。
    datetime_precision: Option<u32>,
    /// 对于字符串列，字符集名称。
    character_set_name: Option<String>,
    column_type: String,
    column_key: String,
    privileges: Option<String>,
    column_comment: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TableIndex {
    /// 包含索引的表所属模式（数据库）的名称。
    table_schema: String,
    /// 包含索引的表的名称。
    table_name: String,
    /// 如果索引不能包含重复内容，则为 0；如果可以，则为 1。
    non_unique: bool,
    /// 索引的名称。如果索引是主键，则名称总是 PRIMARY。
    index_name: String,
    /// 索引中的列序列号，从 1 开始。
    seq_in_index: u32,
    /// 列名。
    column_name: Option<String>,
    /// 索引列中未说明的有关索引的信息
    comment: String,
    /// 创建索引时使用 COMMENT 属性为索引提供的任何注释。
    index_comment: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Table {
    /// 表格所属模式（数据库）的名称。
    table_schema: String,
    /// 表格的名称。
    table_name: String,
    /// 表的 BASE TABLE、视图的 VIEW 或 INFORMATION_SCHEMA 表的 SYSTEM VIEW。
    /// TABLES 表不列出临时表。
    ///
    /// enum('BASE TABLE','VIEW','SYSTEM VIEW')
    table_type: String,
    /// 表的存储引擎。
    /// 对于分区表，ENGINE 显示所有分区使用的存储引擎名称。
    engine: Option<String>,
    /// 创建表格时使用的注释（或 MySQL 无法访问表格信息的原因）。
    table_comment: Option<String>,
    fileds: HashMap<String, TableColumn>,
    indexs: HashMap<String, TableIndex>,
}

/// 获取数据库连接信息
#[tauri::command]
fn dbconn_list<'a>() -> Vec<DbConn> {
    DBConnMap
        .lock()
        .unwrap()
        .values()
        .cloned()
        .collect::<Vec<_>>()
}

/// 编辑数据库连接
#[tauri::command]
fn edit_dbconn(mut conn: DbConn) -> DResult<DbConn> {
    let mut lock = DBConnMap.lock().unwrap();
    if lock.get(&conn.id).is_none() {
        let id_max = lock.keys().max();
        let id = match id_max {
            Some(id) => id + 1,
            None => 1,
        };
        conn.id = id;
    }
    lock.insert(conn.id, conn.clone());
    Ok(conn)
}

/// 删除数据库连接
#[tauri::command]
fn del_dbconn(id: u64) {
    DBConnMap.lock().unwrap().remove(&id);
}

///
/// 生成数据库差异报告
/// 下载数据库差异报告,png,pdf,word
///
/// 生成结构差异SQL
/// 生成数据差异SQL
///
/// 数据库规范检查
/// 数据库规范检查结果下载
/// 设置忽略检查拼写的单词
/// 设置忽略检查复数的单词
/// 自定义检查规范
///
/// 逆向生成代码配置
/// 逆向生成代码

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // tauri::async_runtime::block_on(conn());
    // let url = "mysql://root:root@127.0.0:3306/employees";
    // let res = tauri::async_runtime::block_on(mysql::tables(url));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            dbconn_list,
            edit_dbconn,
            del_dbconn
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

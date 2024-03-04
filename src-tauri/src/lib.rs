#![feature(lazy_cell)]

use std::{
    collections::HashMap,
    error::Error,
    sync::{LazyLock, Mutex, RwLock},
};

use serde::{Deserialize, Serialize};

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

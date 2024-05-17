use serde::{Deserialize, Serialize};
use sqlx::{any::AnyRow, mysql::MySqlRow, FromRow, Pool, Row};

use crate::DResult;

const MYSQL_SHOW_TABLE_CREATE: &str = "show create table ";

const DDL_BLANKET: &str = " ";
const DDL_ALTER_TABLE: &str = "alter table ";
const DDL_ADD: &str = " add ";
const DDL_MODIFY: &str = " modify ";
const DDL_AFTER: &str = " after ";
const DDL_FIRST: &str = " first";
const DDL_NOT: &str = " not ";
const DDL_NULL: &str = " null ";
const DDL_COMMENT: &str = " comment ";
const DDL_AUTO_INCR: &str = " auto_increment ";
const DDL_AUTO_INCR2: &str = "auto_increment";
const DDL_UNSIGNED: &str = " unsigned ";
const DDL_DEFAULT: &str = " default ";

/// +-----------------+--------------------------------------------------------------------+------+-----+---------+-------+
/// | Field           | Type                                                               | Null | Key | Default | Extra |
/// +-----------------+--------------------------------------------------------------------+------+-----+---------+-------+
/// | TABLE_CATALOG   | varchar(64)                                                        | NO   |     | NULL    |       |
/// | TABLE_SCHEMA    | varchar(64)                                                        | NO   |     | NULL    |       |
/// | TABLE_NAME      | varchar(64)                                                        | NO   |     | NULL    |       |
/// | TABLE_TYPE      | enum('BASE TABLE','VIEW','SYSTEM VIEW')                            | NO   |     | NULL    |       |
/// | ENGINE          | varchar(64)                                                        | YES  |     | NULL    |       |
/// | VERSION         | int                                                                | YES  |     | NULL    |       |
/// | ROW_FORMAT      | enum('Fixed','Dynamic','Compressed','Redundant','Compact','Paged') | YES  |     | NULL    |       |
/// | TABLE_ROWS      | bigint unsigned                                                    | YES  |     | NULL    |       |
/// | AVG_ROW_LENGTH  | bigint unsigned                                                    | YES  |     | NULL    |       |
/// | DATA_LENGTH     | bigint unsigned                                                    | YES  |     | NULL    |       |
/// | MAX_DATA_LENGTH | bigint unsigned                                                    | YES  |     | NULL    |       |
/// | INDEX_LENGTH    | bigint unsigned                                                    | YES  |     | NULL    |       |
/// | DATA_FREE       | bigint unsigned                                                    | YES  |     | NULL    |       |
/// | AUTO_INCREMENT  | bigint unsigned                                                    | YES  |     | NULL    |       |
/// | CREATE_TIME     | timestamp                                                          | NO   |     | NULL    |       |
/// | UPDATE_TIME     | datetime                                                           | YES  |     | NULL    |       |
/// | CHECK_TIME      | datetime                                                           | YES  |     | NULL    |       |
/// | TABLE_COLLATION | varchar(64)                                                        | YES  |     | NULL    |       |
/// | CHECKSUM        | bigint                                                             | YES  |     | NULL    |       |
/// | CREATE_OPTIONS  | varchar(256)                                                       | YES  |     | NULL    |       |
/// | TABLE_COMMENT   | text                                                               | YES  |     | NULL    |       |
/// +-----------------+--------------------------------------------------------------------+------+-----+---------+-------+
#[derive(Default, Debug, Serialize, Deserialize, FromRow)]
struct Table {
    // /// 表所属目录的名称。该值始终为 def。
    // table_catalog: String,
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
    // /// 此列未使用。在 MySQL 8.0 中删除 .frm 文件后，此列现在报告的硬编码值为 10，
    // /// 这是 MySQL 5.7 中最后使用的 .frm 文件版本。
    // version: Option<i32>,
    // /// 行存储格式（Fixed, Dynamic, Compressed, Redundant, Compact）。
    // /// 对于 MyISAM 表，动态格式与 myisamchk -dvv 报告的打包格式相对应。
    // ///
    // /// enum('Fixed','Dynamic','Compressed','Redundant','Compact','Paged')
    // row_format: Option<String>,
    // /// 行数。
    // /// 某些存储引擎（如 MyISAM）会存储精确的计数。对于 InnoDB 等其他存储引擎，
    // /// 该值只是一个近似值，可能与实际值相差 40% 到 50%。在这种情况下，
    // /// 请使用 SELECT COUNT(*) 获得准确的计数。
    // /// 对于 INFORMATION_SCHEMA 表，TABLE_ROWS 为空。
    // /// 对于 InnoDB 表，行计数只是用于 SQL 优化的粗略估计值。(如果 InnoDB 表是分区表，情况也是如此）。
    // table_rows: Option<u64>,
    // /// 平均行长。
    // avg_row_length: Option<u64>,
    // /// 对于 MyISAM，DATA_LENGTH 是数据文件的长度，单位为字节。
    // /// 对于 InnoDB，DATA_LENGTH 是为聚簇索引分配的大致空间大小，单位为字节。
    // /// 具体来说，它是以页面为单位的聚类索引大小乘以 InnoDB 页面大小。
    // data_length: Option<u64>,
    // /// 对于 MyISAM，MAX_DATA_LENGTH 是数据文件的最大长度。
    // /// 这是在使用数据指针大小的情况下，表中可存储数据的总字节数。
    // /// 未用于 InnoDB。
    // max_data_length: Option<u64>,
    // /// 对于 MyISAM，INDEX_LENGTH 是索引文件的长度，单位为字节。
    // /// 对于 InnoDB，INDEX_LENGTH 是为非聚类索引分配的大致空间大小，单位为字节。
    // /// 具体来说，它是以页面为单位的非聚类索引大小之和乘以 InnoDB 页面大小。
    // index_length: Option<u64>,
    // /// 已分配但未使用的字节数。
    // /// InnoDB 表会报告表所属表空间的可用空间。对于共享表空间中的表，这是共享表空间的可用空间。如果使用的是多个表空间，且表有自己的表空间，则可用空间仅指该表的可用空间。可用空间是指完全空闲的扩展区中的字节数减去安全系数。即使可用空间显示为 0，只要不需要分配新的扩展，就可以插入行。
    // /// 对于 NDB 群集，DATA_FREE 显示磁盘上为磁盘数据表或磁盘片段分配但未使用的空间。(内存数据资源使用情况由 DATA_LENGTH 列报告）。
    // /// 对于分区表，该值只是一个估计值，不一定绝对正确。
    // /// 在这种情况下，获取该信息的更准确方法是查询 INFORMATION_SCHEMA PARTITIONS 表
    // data_free: Option<u64>,
    // /// 下一个 AUTO_INCREMENT 值。
    // auto_increment: Option<u64>,
    // /// 创建表格的时间。
    // create_time: String,
    // /// 表最后一次更新的时间。对于某些存储引擎，该值为空。即使在每个 InnoDB 表都位于单独 .ibd文件中的按表生成文件模式下，更改缓冲也会延迟写入数据文件，因此文件修改时间与上次插入、更新或删除的时间不同。对于 MyISAM，使用的是数据文件时间戳；但在 Windows 上，时间戳不会因更新而更新，因此该值并不准确。
    // /// UPDATE_TIME 显示在未分区的 InnoDB 表上执行的最后一次 UPDATE、INSERT 或 DELETE 的时间戳值。对于 MVCC，时间戳值反映的是 COMMIT 时间，即最后一次更新时间。服务器重启或表从 InnoDB 数据字典缓存中删除时，时间戳不会被持久化。
    // update_time: Option<String>,
    // /// 最后一次检查表的时间。
    // /// 并非所有存储引擎都会更新这个时间，在这种情况下，该值始终为空。
    // /// 对于 InnoDB 分区表，CHECK_TIME 始终为空。
    // check_time: Option<String>,
    // /// 表格默认校对方式。输出结果不会明确列出表格默认字符集，但校对名称以字符集名称开头。
    // table_collation: Option<String>,
    // /// 实时校验和值（如果有）。
    // checksum: Option<i64>,
    // /// 与 CREATE TABLE 一起使用的额外选项。
    // /// 对于分区表，CREATE_OPTIONS 显示 partitioned。
    // /// 在 MySQL 8.0.16 之前，CREATE_OPTIONS 会显示在按文件表空间创建的表中指定的 ENCRYPTION 子句。
    // /// 从 MySQL 8.0.16 起，如果表已加密或指定的加密与模式加密不同，它将显示按文件表表空间的加密子句。在一般表空间中创建的表不会显示加密子句。要识别加密的按表文件表空间和一般表空间，请查询 INNODB_TABLESPACES ENCRYPTION 列。
    // /// 在禁用严格模式的情况下创建表格时，如果不支持指定的行格式，则使用存储引擎的默认行格式。表的实际行格式在 ROW_FORMAT 列中报告。CREATE_OPTIONS 显示在 CREATE TABLE 语句中指定的行格式。
    // /// 在更改表的存储引擎时，不适用于新存储引擎的表选项会保留在表定义中，以便在必要时使用以前定义的选项将表还原为原始存储引擎。CREATE_OPTIONS 列可能会显示保留的选项。
    // create_options: Option<String>,
    /// 创建表格时使用的注释（或 MySQL 无法访问表格信息的原因）。
    table_comment: Option<String>,
}

/// +--------------------------+----------------------------+------+-----+---------+-------+
/// | Field                    | Type                       | Null | Key | Default | Extra |
/// +--------------------------+----------------------------+------+-----+---------+-------+
/// | TABLE_CATALOG            | varchar(64)                | NO   |     | NULL    |       |
/// | TABLE_SCHEMA             | varchar(64)                | NO   |     | NULL    |       |
/// | TABLE_NAME               | varchar(64)                | NO   |     | NULL    |       |
/// | COLUMN_NAME              | varchar(64)                | YES  |     | NULL    |       |
/// | ORDINAL_POSITION         | int unsigned               | NO   |     | NULL    |       |
/// | COLUMN_DEFAULT           | text                       | YES  |     | NULL    |       |
/// | IS_NULLABLE              | varchar(3)                 | NO   |     |         |       |
/// | DATA_TYPE                | longtext                   | YES  |     | NULL    |       |
/// | CHARACTER_MAXIMUM_LENGTH | bigint                     | YES  |     | NULL    |       |
/// | CHARACTER_OCTET_LENGTH   | bigint                     | YES  |     | NULL    |       |
/// | NUMERIC_PRECISION        | bigint unsigned            | YES  |     | NULL    |       |
/// | NUMERIC_SCALE            | bigint unsigned            | YES  |     | NULL    |       |
/// | DATETIME_PRECISION       | int unsigned               | YES  |     | NULL    |       |
/// | CHARACTER_SET_NAME       | varchar(64)                | YES  |     | NULL    |       |
/// | COLLATION_NAME           | varchar(64)                | YES  |     | NULL    |       |
/// | COLUMN_TYPE              | mediumtext                 | NO   |     | NULL    |       |
/// | COLUMN_KEY               | enum('','PRI','UNI','MUL') | NO   |     | NULL    |       |
/// | EXTRA                    | varchar(256)               | YES  |     | NULL    |       |
/// | PRIVILEGES               | varchar(154)               | YES  |     | NULL    |       |
/// | COLUMN_COMMENT           | text                       | NO   |     | NULL    |       |
/// | GENERATION_EXPRESSION    | longtext                   | NO   |     | NULL    |       |
/// | SRS_ID                   | int unsigned               | YES  |     | NULL    |       |
/// +--------------------------+----------------------------+------+-----+---------+-------+
#[derive(Default, Debug, Serialize, Deserialize, FromRow)]
struct TableColumn {
    // /// 包含列的表所属目录的名称。该值始终为 def
    // table_catalog: String,
    /// 包含列的表所属模式（数据库）的名称
    table_schema: String,
    /// 包含列的表的名称
    table_name: String,
    /// 列的名称
    column_name: Option<String>,
    /// 列在表中的位置。
    /// 之所以需要 ORDINAL_POSITION，是因为可能需要使用 ORDER BY ORDINAL_POSITION。
    /// 与 SHOW COLUMNS 不同，从 COLUMNS 表中 SELECT 不会自动排序
    ordinal_position: u32,
    /// 列的默认值。
    /// 如果列的显式默认值为 NULL，或者列定义中没有 DEFAULT 子句，则默认值为 NULL
    column_default: Option<String>,
    /// YES/NO
    /// 列的无效性。
    /// 如果列中可以存储 NULL 值，则值为 "YES"；
    /// 如果不能存储 NULL 值，则值为 "NO"。
    is_nullable: String,
    /// 列数据类型。
    /// DATA_TYPE 值只有类型名称，没有其他信息。
    /// COLUMN_TYPE 值包含类型名称和其他信息，如精度或长度。
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
    // /// 对于字符串列，校对名称。
    // collation_name: Option<String>,
    /// 列数据类型。
    /// DATA_TYPE 值只有类型名称，没有其他信息。
    /// COLUMN_TYPE 值包含类型名称和其他信息，如精度或长度。
    column_type: String,
    /// enum('','PRI','UNI','MUL')
    /// 列是否有索引：
    /// 如果 COLUMN_KEY 为空，则表示该列未被索引，或仅作为多列非唯一索引中的辅助列被索引。
    /// 如果 COLUMN_KEY 为 PRI，则该列是 PRIMARY KEY，或者是多列 PRIMARY KEY 中的一列。
    /// 如果 COLUMN_KEY 为 UNI，则该列为 UNIQUE 索引的第一列。(UNIQUE 索引允许多个 NULL 值，但可以通过检查 Null 列来确定列是否允许 NULL）。
    /// 如果 COLUMN_KEY 为 MUL，则该列为非唯一索引的第一列，该索引允许在列中多次出现给定值。
    ///
    /// 如果有多个 COLUMN_KEY 值适用于表中的某一列，则 COLUMN_KEY 将按照 PRI、UNI、MUL 的顺序显示优先级最高的值。
    /// 如果 UNIQUE 索引不能包含 NULL 值，并且表中没有 PRIMARY KEY，则 UNIQUE 索引可能显示为 PRI。如果多个列组成一个复合 UNIQUE 索引，则 UNIQUE 索引可能显示为 MUL；虽然列的组合是唯一的，但每个列仍可包含给定值的多次出现。
    column_key: String,
    // /// 关于给定列的任何附加信息。
    // /// 在这些情况下，该值为非空值：
    // /// auto_increment 用于具有 AUTO_INCREMENT 属性的列。
    // /// ON UPDATE CURRENT_TIMESTAMP 属性的 TIMESTAMP 或 DATETIME 列的 ON UPDATE CURRENT_TIMESTAMP。
    // /// STORED GENERATED 或 VIRTUAL GENERATED 用于生成列。
    // /// DEFAULT_GENERATED 用于具有表达式默认值的列。
    // extra: Option<String>,
    /// 拥有的栏目权限。
    privileges: Option<String>,
    /// 列定义中包含的任何注释。
    column_comment: String,
    // /// 对于生成的列，显示用于计算列值的表达式。非生成列为空。
    // generation_expression: Option<String>,
    // /// 该值适用于空间列。
    // /// 它包含列 SRID 值，表示存储在列中的值的空间参照系统。
    // /// 对于非空间列和没有 SRID 属性的空间列，该值为 NULL。
    // srs_id: Option<u32>,
}

/// / +---------------+---------------+------+-----+---------+-------+
/// | Field         | Type          | Null | Key | Default | Extra |
/// +---------------+---------------+------+-----+---------+-------+
/// | TABLE_CATALOG | varchar(64)   | NO   |     | NULL    |       |
/// | TABLE_SCHEMA  | varchar(64)   | NO   |     | NULL    |       |
/// | TABLE_NAME    | varchar(64)   | NO   |     | NULL    |       |
/// | NON_UNIQUE    | int           | NO   |     | 0       |       |
/// | INDEX_SCHEMA  | varchar(64)   | NO   |     | NULL    |       |
/// | INDEX_NAME    | varchar(64)   | YES  |     | NULL    |       |
/// | SEQ_IN_INDEX  | int unsigned  | NO   |     | NULL    |       |
/// | COLUMN_NAME   | varchar(64)   | YES  |     | NULL    |       |
/// | COLLATION     | varchar(1)    | YES  |     | NULL    |       |
/// | CARDINALITY   | bigint        | YES  |     | NULL    |       |
/// | SUB_PART      | bigint        | YES  |     | NULL    |       |
/// | PACKED        | binary(0)     | YES  |     | NULL    |       |
/// | NULLABLE      | varchar(3)    | NO   |     |         |       |
/// | INDEX_TYPE    | varchar(11)   | NO   |     |         |       |
/// | COMMENT       | varchar(8)    | NO   |     |         |       |
/// | INDEX_COMMENT | varchar(2048) | NO   |     | NULL    |       |
/// | IS_VISIBLE    | varchar(3)    | NO   |     |         |       |
/// | EXPRESSION    | longtext      | YES  |     | NULL    |       |
/// +---------------+---------------+------+-----+---------+-------+
#[derive(Default, Debug, Serialize, Deserialize, FromRow)]
struct TableIndex {
    // /// 包含索引的表所属目录的名称。该值始终为 def。
    // table_catalog: String,
    /// 包含索引的表所属模式（数据库）的名称。
    table_schema: String,
    /// 包含索引的表的名称。
    table_name: String,
    /// 如果索引不能包含重复内容，则为 0；如果可以，则为 1。
    non_unique: i32,
    // /// 索引所属模式（数据库）的名称。
    // index_schema: String,
    /// 索引的名称。如果索引是主键，则名称总是 PRIMARY。
    index_name: Option<String>,
    /// 索引中的列序列号，从 1 开始。
    seq_in_index: u32,
    /// 列名。
    column_name: Option<String>,
    /// 列在索引中的排序方式。其值可以是 A（升序）、D（降序）或 NULL（未排序）。
    collation: Option<String>,
    // /// 索引中唯一值的估计数量。
    // /// 要更新这个数字，请运行 ANALYZE TABLE 或（对于 MyISAM 表）myisamchk -a。
    // /// CARDINALITY 是根据以整数形式存储的统计数据计算的，因此即使对于小表，数值也不一定精确。卡片数越高，MySQL 在进行连接时使用索引的机会就越大。
    // cardinality: Option<i64>,
    // /// 索引前缀。
    // /// 也就是说，如果列只有部分索引，则为索引字符数；如果整个列都有索引，则为 NULL。
    // sub_part: Option<i64>,
    // /// 表示键的打包方式。否则为空。
    // packed: Option<Vec<u8>>,
    /// 如果列可能包含 NULL 值，则包含 "YES"；如果不包含 NULL 值，则包含"'"。
    nullable: String,
    /// 使用的索引方法（BTREE、FULLTEXT、HASH、RTREE）。
    index_type: String,
    /// 索引列中未说明的有关索引的信息
    comment: String,
    /// 创建索引时使用 COMMENT 属性为索引提供的任何注释。
    index_comment: String,
    // /// 优化器是否可见索引。
    // is_visible: String,
    // /// MySQL 8.0.13 及更高版本支持功能键部分（请参阅功能键部分），
    // /// 这对 COLUMN_NAME 和 EXPRESSION 列都有影响：
    // /// 对于非功能键部分，COLUMN_NAME 表示由键部分索引的列，EXPRESSION 为 NULL。
    // /// 对于功能键部分，COLUMN_NAME 列为空，EXPRESSION 表示键部分的表达式。
    // expression: Option<String>,
}

/// Rust type             MySQL type(s)
/// bool                    TINYINT(1), BOOLEAN
/// i8                      TINYINT
/// i16                     SMALLINT
/// i32                     INT
/// i64                     BIGINT
/// u8                      TINYINT UNSIGNED
/// u16                     SMALLINT UNSIGNED
/// u32                     INT UNSIGNED
/// u64                     BIGINT UNSIGNED
/// f32                     FLOAT
/// f64                     DOUBLE
/// &str, String            VARCHAR, CHAR, TEXT
/// &[u8], Vec<u8>          VARBINARY, BINARY, BLOB
///
/// time::PrimitiveDateTime DATETIME
/// time::OffsetDateTime    TIMESTAMP
/// time::Date              DATE
/// time::Time              TIME
///
/// bigdecimal::BigDecimal  DECIMAL
///
/// uuid::Uuid              BYTE(16), VARCHAR, CHAR, TEXT
/// uuid::fmt::Hyphenated   CHAR(36)
/// uuid::fmt::Simple       CHAR(32)
///
/// serde_json::JsonValue  JSON
///
/// Mysql 类型转换为Rust对应类型
fn t2t(ty: &str) -> &str {
    match ty.to_uppercase().as_str() {
        "TINYINT(1)" | "BOOLEAN" => "bool",
        "TINYINT" => "i8",
        "TINYINT UNSIGNED" | "BIT" => "u8",
        "SMALLINT" => "i16",
        "SMALLINT UNSIGNED" => "u16",
        "INT" | "MEDIUMINT" => "i32",
        "INT UNSIGNED" | "MEDIUMINT UNSIGNED" => "u32",
        "BIGINT" => "i64",
        "BIGINT UNSIGNED" => "u64",
        "FLOAT" => "f32",
        "DOUBLE" | "NUMERIC" => "f64",
        "VARBINARY" | "BINARY" | "BLOB" => "Vec<u8>",
        "YEAR" => "time::Date",
        "DATE" => "time::Date",
        "TIME" => "time::Time",
        "DATETIME" => "time::PrimitiveDateTime",
        "TIMESTAMP" => "time::offsetDateTime",
        "DECIMAL" => "bigdecimal::BigDecimal",
        "JSON" => "serde_json:JsonValue",
        _ => "String",
    }
}

async fn tables(url: &str) -> DResult<Vec<Table>> {
    let sql = "SELECT table_schema,table_name,table_type,engine,table_comment
         FROM information_schema.`TABLES` WHERE TABLE_SCHEMA = ( SELECT DATABASE ())";

    let pool = crate::conn_mysql(url).await?;

    let res = sqlx::query(&sql)
        .map(|row: MySqlRow| {
            let table_schema: String = row.try_get(0).unwrap();
            let table_name: String = row.try_get(1).unwrap();
            let table_type: String = row.try_get(2).unwrap();
            let engine: Option<String> = row.try_get(3).unwrap();
            let table_comment: Option<String> = row.try_get(4).unwrap();

            Table {
                table_schema,
                table_name,
                table_type,
                engine,
                table_comment,
            }
        })
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            dbg!(&e);
            "SQL ERROR"
        })?;
    Ok(res)
}

async fn columns(url: &str) -> DResult<Vec<TableColumn>> {
    let sql = r#"SELECT table_schema,table_name,column_name,ordinal_position,column_default,is_nullable,data_type,
            character_maximum_length,character_octet_length,numeric_precision,numeric_scale,datetime_precision,character_set_name,
            column_type,column_key,privileges,column_comment 
            FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = ( SELECT DATABASE ())"#
        .to_string();
    let pool = crate::conn_mysql(url).await?;
    let res = sqlx::query(&sql)
        .map(|row: MySqlRow| {
            let table_schema: String = row.try_get(0).unwrap();
            let table_name: String = row.try_get(1).unwrap();
            let column_name: Option<String> = row.try_get(2).unwrap();
            let ordinal_position: u32 = row.try_get(3).unwrap();
            let column_default: Option<String> = row.try_get(4).unwrap();
            let is_nullable: String = row.try_get(5).unwrap();
            let data_type: Option<String> = row.try_get(6).unwrap();
            let character_maximum_length: Option<i64> = row.try_get(7).unwrap();
            let character_octet_length: Option<i64> = row.try_get(8).unwrap();
            let numeric_precision: Option<u64> = row.try_get(9).unwrap();
            let numeric_scale: Option<u64> = row.try_get(10).unwrap();
            let datetime_precision: Option<u32> = row.try_get(11).unwrap();
            let character_set_name: Option<String> = row.try_get(12).unwrap();
            let column_type: String = row.try_get(13).unwrap();
            let column_key: String = row.try_get(14).unwrap();
            let privileges: Option<String> = row.try_get(15).unwrap();
            let column_comment: String = row.try_get(16).unwrap();

            TableColumn {
                table_schema,
                table_name,
                column_name,
                ordinal_position,
                column_default,
                is_nullable,
                data_type,
                character_maximum_length,
                character_octet_length,
                numeric_precision,
                numeric_scale,
                datetime_precision,
                character_set_name,
                column_type,
                column_key,
                privileges,
                column_comment,
            }
        })
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            dbg!(&e);
            "SQL ERROR"
        })?;
    Ok(res)
}

async fn indexs(url: &str) -> DResult<Vec<TableIndex>> {
    let sql = "SELECT table_schema,table_name,non_unique,index_name,seq_in_index,
         column_name,collation,nullable,index_type,comment,index_comment
         FROM information_schema.STATISTICS WHERE TABLE_SCHEMA = ( SELECT DATABASE ())"
        .to_string();

    let pool = crate::conn_mysql(url).await?;

    let res = sqlx::query(&sql)
        .map(|row: MySqlRow| {
            let table_schema: String = row.try_get(0).unwrap();
            let table_name: String = row.try_get(1).unwrap();
            let non_unique: i32 = row.try_get(2).unwrap();
            let index_name: Option<String> = row.try_get(3).unwrap();
            let seq_in_index: u32 = row.try_get(4).unwrap();
            let column_name: Option<String> = row.try_get(5).unwrap();
            let collation: Option<String> = row.try_get(6).unwrap();
            let nullable: String = row.try_get(7).unwrap();
            let index_type: String = row.try_get(8).unwrap();
            let comment: String = row.try_get(9).unwrap();
            let index_comment: String = row.try_get(10).unwrap();

            TableIndex {
                table_schema,
                table_name,
                non_unique,
                index_name,
                seq_in_index,
                column_name,
                collation,
                nullable,
                index_type,
                comment,
                index_comment,
            }
        })
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            dbg!(&e);
            "SQL ERROR"
        })?;
    Ok(res)
}

pub async fn table(url: &str) -> DResult<Vec<super::Table>> {
    let tables = tables(url).await?;
    if tables.is_empty() {
        return Ok(Vec::with_capacity(0));
    }
    let columns = columns(url).await?;
    let indexs = indexs(url).await?;

    unimplemented!()
}

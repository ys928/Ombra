use std::sync::Mutex;

use rusqlite::LoadExtensionGuard;

use crate::{tools, FileInfo};

static DB_CONNECT: Mutex<Option<rusqlite::Connection>> = Mutex::new(None);

pub fn init(reset: bool) {
    let path = tools::get_data_dir(None);
    let fc = std::path::Path::new(&path)
        .join("file_catch.db")
        .to_string_lossy()
        .to_string();
    let mut db = DB_CONNECT.lock().unwrap();
    if let None = *db {
        *db = Some(rusqlite::Connection::open(fc).unwrap());
        unsafe {
            let _guard = LoadExtensionGuard::new(db.as_ref().unwrap());
            let _ = db.as_ref().unwrap().load_extension("regex0.dll", None);
        };
    }
    if reset {
        db.as_ref()
            .unwrap()
            .execute("DROP TABLE IF EXISTS files;", [])
            .unwrap();
    }
    db.as_ref()
        .unwrap()
        .execute(
            "create table if not exists files(
    name text,
    path text,
    time INTEGER,
    type INTEGER);",
            [],
        )
        .unwrap();
}

//大量插入操作
pub fn insert_files(files: Vec<FileInfo>) {
    let db = DB_CONNECT.lock().unwrap();
    let mut stat = db
        .as_ref()
        .unwrap()
        .prepare("insert into files (name,path,time,type) values (?1,?2,?3,?4);")
        .unwrap();
    let _ = db.as_ref().unwrap().execute("BEGIN TRANSACTION", []);
    for i in files {
        let _ = stat.execute([i.name, i.path, i.time.to_string(), i.ftype.to_string()]);
    }
    let _ = db.as_ref().unwrap().execute("COMMIT", []);
    let _ = db
        .as_ref()
        .unwrap()
        .execute("CREATE INDEX index_name ON files(name);", []); //建立索引，提高匹配效率
    let _ = db.as_ref().unwrap().execute("VACUUM;", []); //缩减占用空间
}

pub fn search_file(name: &str, limit: i32, offset: i32) -> Vec<FileInfo> {
    init(false);
    let db = DB_CONNECT.lock().unwrap();
    let qstr = format!(
        "SELECT name,path,time,type FROM files WHERE name LIKE '%{}%' limit {} OFFSET {};",
        name, limit, offset
    );

    let stat = db.as_ref().unwrap().prepare(&qstr);
    if stat.is_err(){
        return Vec::new();
    }
    let mut stat=stat.unwrap();
    let rows = stat
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, u64>(2)?,
                row.get::<_, i32>(3)?,
            ))
        })
        .unwrap();
    let mut files = Vec::new();
    for row in rows {
        let (name, path, time, ftype) = row.unwrap();
        files.push(FileInfo {
            name: name,
            path: path,
            time: time,
            ftype: ftype,
        });
    }
    return files;
}

pub fn search_file_as_regex(re: &str,limit: i32, offset: i32) -> Vec<FileInfo> {
    init(false);
    let db = DB_CONNECT.lock().unwrap();
    let qstr = format!(
        "SELECT name,path,time,type FROM files WHERE name REGEXP {} limit {} OFFSET {};",
        re, limit,offset
    );
    let stat = db.as_ref().unwrap().prepare(&qstr);
    if stat.is_err() {
        return Vec::new();
    }
    let mut stat = stat.unwrap();
    let rows = stat
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, u64>(2)?,
                row.get::<_, i32>(3)?,
            ))
        })
        .unwrap();
    let mut files = Vec::new();
    for row in rows {
        let (name, path, time, ftype) = row.unwrap();
        files.push(FileInfo {
            name: name,
            path: path,
            time: time,
            ftype: ftype,
        });
    }
    return files;
}

pub fn search_file_as_whole_word(name: &str,limit: i32, offset: i32) -> Vec<FileInfo> {
    init(false);
    let name = name
        .replace(r"\", r"\\")
        .replace(".", "\\.")
        .replace("^", r"\^")
        .replace("$", r"\$")
        .replace("+", r"\+")
        .replace("*", r"\*")
        .replace("?", r"\?")
        .replace("+", r"\+")
        .replace("+", r"\+");
    let db = DB_CONNECT.lock().unwrap();
    let qstr = format!(
        r"SELECT name,path,time,type FROM files WHERE name REGEXP '.*\b{}\b.*' limit {} OFFSET {};",
        name, limit,offset
    );
    let stat = db.as_ref().unwrap().prepare(&qstr);
    if stat.is_err() {
        return Vec::new();
    }
    let mut stat = stat.unwrap();
    let rows = stat
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, u64>(2)?,
                row.get::<_, i32>(3)?,
            ))
        })
        .unwrap();
    let mut files = Vec::new();
    for row in rows {
        let (name, path, time, ftype) = row.unwrap();
        files.push(FileInfo {
            name: name,
            path: path,
            time: time,
            ftype: ftype,
        });
    }
    return files;
}

pub fn get_file_num() -> i32 {
    init(false);
    let db = DB_CONNECT.lock().unwrap();
    let num: Result<i32, rusqlite::Error> =
        db.as_ref()
            .unwrap()
            .query_row("select count(*) from files;", [], |row| row.get(0));
    return num.unwrap();
}

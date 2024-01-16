use std::{collections::LinkedList, io::ErrorKind, path::PathBuf, sync::Mutex};

use crate::{tools, FileInfo};

static DB_CONNECT: Mutex<Option<rusqlite::Connection>> = Mutex::new(None);

use log::{debug, info};

pub fn init() {
    let fc = get_catch_file_path();
    let mut db = DB_CONNECT.lock().unwrap();
    let _ = std::fs::remove_file(&fc);
    *db = Some(rusqlite::Connection::open(fc).unwrap());
    db.as_ref()
        .unwrap()
        .execute(
            "create table files(
    name text,
    path text,
    ext text,
    isdir INTEGER,
    time INTEGER,
    UNIQUE (path, name,ext,isdir));",
            [],
        )
        .unwrap();
}

fn get_catch_file_path() -> PathBuf {
    let path = tools::get_data_dir(None);
    let fc = std::path::Path::new(&path).join("file_catch.db");
    return fc;
}

#[tauri::command]
pub fn get_file_catch_info() -> i32 {
    let fc = get_catch_file_path();
    if fc.exists() {
        return get_file_num();
    } else {
        return 0;
    }
}

//大量插入操作
pub fn insert_files(files: LinkedList<FileInfo>) {
    let db = DB_CONNECT.lock().unwrap();
    let mut stat = db
        .as_ref()
        .unwrap()
        .prepare("insert into files (name,ext,path,time,isdir) values (?1,?2,?3,?4,?5);")
        .unwrap();
    let _ = db.as_ref().unwrap().execute("BEGIN TRANSACTION", []);
    for i in files {
        let mut t = 0;
        if i.isdir {
            t = 1;
        }
        let _ = stat.execute([i.name, i.ext, i.path, i.time.to_string(), t.to_string()]);
    }
    let _ = db.as_ref().unwrap().execute("COMMIT", []);
    let _ = db
        .as_ref()
        .unwrap()
        .execute("CREATE INDEX index_name ON files(name);", []); //建立索引，提高匹配效率
    let _ = db.as_ref().unwrap().execute("VACUUM;", []); //缩减占用空间
}
//搜索文件
pub fn search_file(name: &str, ext: &str, limit: i32, offset: i32) -> Vec<FileInfo> {
    debug!("enter search_file");
    let db = DB_CONNECT.try_lock();
    if db.is_err() {
        info!("lock db failed");
        return Vec::new();
    }
    let db = db.unwrap();
    if db.is_none() {
        return Vec::new();
    }
    let qstr = format!(
            "SELECT name,ext,path,time,isdir FROM files WHERE name LIKE '%{}%' and ext LIKE '%{}%' limit {} OFFSET {};",
            name,ext, limit, offset
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
                row.get::<_, String>(2)?,
                row.get::<_, u64>(3)?,
                row.get::<_, i32>(4)?,
            ))
        })
        .unwrap();
    let mut files = Vec::new();
    for row in rows {
        let (name, ext, path, time, t) = row.unwrap();
        let mut isdir = false;
        if t == 1 {
            isdir = true;
        }
        files.push(FileInfo {
            name: name,
            path: path,
            ext: ext,
            time: time,
            isdir: isdir,
        });
    }
    return files;
}

pub fn search_file_as_exact(name: &str, ext: &str, limit: i32, offset: i32) -> Vec<FileInfo> {
    let db = DB_CONNECT.lock().unwrap();
    let qstr = format!(
        r"SELECT name,ext,path,time,isdir FROM files WHERE name='{}' and ext='{}' limit {} OFFSET {};",
        name, ext, limit, offset
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
                row.get::<_, String>(2)?,
                row.get::<_, u64>(3)?,
                row.get::<_, i32>(4)?,
            ))
        })
        .unwrap();
    let mut files = Vec::new();
    for row in rows {
        let (name, ext, path, time, t) = row.unwrap();
        let mut isdir = false;
        if t == 1 {
            isdir = true;
        }
        files.push(FileInfo {
            name: name,
            ext: ext,
            path: path,
            time: time,
            isdir: isdir,
        });
    }
    return files;
}

//更新文件
pub fn update_file(path: &Vec<PathBuf>) {
    let db = DB_CONNECT.lock();
    if db.is_err() {
        return;
    }
    let db = db.unwrap();
    let mut update: rusqlite::Statement<'_> = db
        .as_ref()
        .unwrap()
        .prepare("insert or replace into files (name,ext,path,time,isdir) values (?1,?2,?3,?4,?5);")
        .unwrap();
    let mut remove = db
        .as_ref()
        .unwrap()
        .prepare("delete from files where (path=? and name=?) or (path=?);")
        .unwrap();
    let _ = db.as_ref().unwrap().execute("BEGIN TRANSACTION", []);
    for file_full_path in path.iter() {
        let meta = file_full_path.metadata();

        let name = file_full_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let parent_path = file_full_path
            .parent()
            .unwrap_or(&PathBuf::new())
            .to_string_lossy()
            .to_string();

        if let Err(e) = meta {
            //删除文件事件处理
            if e.kind() == ErrorKind::NotFound {
                remove
                    .execute([
                        parent_path,
                        name,
                        file_full_path.to_string_lossy().to_string(),
                    ])
                    .unwrap();
            }
            continue;
        }
        let meta = meta.unwrap();

        let time = meta.modified().unwrap();
        let time = tools::sys_time_to_seconds(time);

        let ext;
        let mut t = 0;
        if meta.is_dir() {
            t = 1;
            ext = String::new();
        } else {
            ext = file_full_path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
        }
        update
            .insert([name, ext, parent_path, time.to_string(), t.to_string()])
            .unwrap();
    }
    let _ = db.as_ref().unwrap().execute("COMMIT", []);
}

pub fn get_file_num() -> i32 {
    let db = DB_CONNECT.try_lock();
    if db.is_err() {
        info!("lock db failed");
        return 0;
    }
    let db = db.unwrap();
    if db.is_none() {
        info!("db is none failed");
        return 0;
    }
    let num: Result<i32, rusqlite::Error> =
        db.as_ref()
            .unwrap()
            .query_row("select count(*) from files;", [], |row| row.get(0));
    return num.unwrap();
}

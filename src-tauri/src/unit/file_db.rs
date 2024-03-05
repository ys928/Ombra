use std::{collections::LinkedList, path::PathBuf, sync::Mutex, time::Duration};

use crate::FileInfo;

static DB_CONNECT: Mutex<Option<rusqlite::Connection>> = Mutex::new(None);

use log::{debug, info};

pub fn init() {
    let fc = get_db_path();
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

pub fn get_db_path() -> PathBuf {
    let path = crate::unit::fs::data_dir(None);
    let fc = std::path::Path::new(&path).join("file_catch.db");
    return fc;
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
        info!("search_file lock db failed");
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
    let mut db;
    loop {
        db = DB_CONNECT.try_lock();
        if db.is_err() {
            std::thread::sleep(Duration::from_millis(200));
            continue;
        }
        break;
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
        .prepare("delete from files where (path=? and name=? and ext=?) or (path=?);")
        .unwrap();
    let _ = db.as_ref().unwrap().execute("BEGIN TRANSACTION", []);
    for file_full_path in path.iter() {
        //删除文件事件处理
        if !file_full_path.exists() {
            let name = file_full_path
                .file_stem()
                .unwrap_or_else(|| file_full_path.file_name().unwrap_or_default())
                .to_string_lossy()
                .to_string();
            let ext = file_full_path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let parent_path = file_full_path
                .parent()
                .unwrap()
                .to_string_lossy()
                .to_string();
            remove
                .execute([
                    parent_path,
                    name,
                    ext,
                    file_full_path.to_string_lossy().to_string(),
                ])
                .unwrap();
        }
        let fi = crate::unit::fs::get_file_info(&file_full_path);
        if fi.is_err() {
            continue;
        }
        let fi = fi.unwrap();
        let t;
        if fi.isdir {
            t = 1;
        } else {
            t = 0;
        }
        update
            .insert([fi.name, fi.ext, fi.path, fi.time.to_string(), t.to_string()])
            .unwrap();
    }
    let _ = db.as_ref().unwrap().execute("COMMIT", []);
}

pub fn get_file_num() -> i32 {
    let db = DB_CONNECT.try_lock();
    if db.is_err() {
        info!("get_file_num lock db failed");
        return 0;
    }
    let db = db.unwrap();
    if db.is_none() {
        info!("get_file_num db is none");
        return 0;
    }
    let num: Result<i32, rusqlite::Error> =
        db.as_ref()
            .unwrap()
            .query_row("select count(*) from files;", [], |row| row.get(0));
    return num.unwrap();
}

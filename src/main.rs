

use rusqlite::{params, Connection, Result};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Utc, Local, DateTime, Date};
use chrono::{NaiveDateTime, Duration};

/*
fn exec_sql(file_sqlite3: String) {
    let conn = Connection::open(file_sqlite3).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        params![me.name, me.data],
    ).unwrap();
}
*/

//
pub fn get_content(filename: &str ) -> String{
//    println!("In file {}", filename);
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

//    println!("With text:\n{}", contents);
    return contents;
}
#[derive(Debug)]
pub struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}
//
fn exec_sql_array(file_sqlite3: String, items: Vec<TaskItem>) {
    let conn = Connection::open(file_sqlite3).unwrap();

    for row in &items {
//        println!("exe.name={}", &row.name );
        let s_title = &row.title;
        let s_content = &row.content;
        let me = TaskItem {
            id: 0,
            title: s_title.to_string(),
            content: s_content.to_string(),
        };    
        conn.execute(
            "INSERT INTO tasks (title, content) VALUES (?1, ?2)",
            params![ me.title, me.content ],
        ).unwrap();
    }
//    Ok(())
}

//
#[derive(Serialize, Deserialize , Debug)]
struct TestItem {
    age: i64,
    name: String,
}
//
#[derive(Serialize, Deserialize , Debug)]
struct TaskItem {
    id: i64,
    title: String,
    content: String,
}

//
fn main(){
    println!("#-start-main");
    let text = Utc::now().format("%Y-%m-%d %H:%M:%S.%f").to_string();
    println!("text_1={}", text);
    let dt1: NaiveDateTime = NaiveDateTime::parse_from_str( &text, "%Y-%m-%d %H:%M:%S.%f").unwrap();

    let json = get_content("./tasks.json");
//    println!("main={}", json);
    let deserialized: Vec<TaskItem> = serde_json::from_str(&json).unwrap();
/*
    for row in &deserialized {
        println!("name={}", &row.title );
    }
*/
    exec_sql_array("test.db".to_string(), deserialized);

    let text_2 = Utc::now().format("%Y-%m-%d %H:%M:%S.%f").to_string();
    let dt2: NaiveDateTime = NaiveDateTime::parse_from_str( &text_2, "%Y-%m-%d %H:%M:%S.%f" ).unwrap();
    let duration: Duration = dt2 - dt1;
    println!("text_2={}", text_2 );
    println!("secs: {}", duration.num_seconds());
    println!("milliseconds: {}", duration.num_milliseconds());    
}

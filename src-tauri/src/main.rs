// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #[macro_use]
// extern crate prettytable;

use rusqlite::{Connection, Error, Result};
use serde::{Deserialize, Serialize};
// use prettytable::Table;
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TodoInput {
    title: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
}

fn init_db() -> Result<Connection, Error> {
    let conn = Connection::open("todos.db")?;

    conn.execute(
        "create table if not exists todos (
             id text primary key,
             title text not null,
             completed boolean not null
        )",
        (),
    )?;

    Ok(conn)
}

// fn show_one(conn: &Connection, id: String) -> Result<Vec<Todo>, Error> {
//     let mut stmt = conn.prepare("SELECT * from todos WHERE (id) = :id;")?;
//     let todos = stmt.query_map(&[(":id", &id)], |row| {
//         Ok(Todo {
//             id: row.get(0)?,
//             title: row.get(1)?,
//             completed: row.get(2)?,
//         })
//     })?;
//     let mut todos_array = Vec::new();
//     for todo in todos {
//         match todo {
//             Ok(todo) => todos_array.push(todo),
//             Err(err) => println!("Error occurred: {:?}", err),
//         }
//     }
//     Ok(todos_array)
// }

#[tauri::command]
async fn add_todo(input: Value) -> Value {
    // println!("{:#?}", input);
    let result = init_db();

    let conn: Connection;

    match result {
        Ok(connection) => {
            conn = connection;

            let value: TodoInput = serde_json::from_value(input).unwrap();
            let TodoInput {title} = &value;
            // println!("title: {}", title, );
            
            let todo = Todo {
                id: Uuid::new_v4().to_string(),
                title: title.trim().to_owned(),
                completed: false,
            };
            println!("Todo: {:?}", todo);

            let result = conn.execute(
                "INSERT INTO todos (id, title, completed) values (?1, ?2, ?3)",
                [&todo.id, &todo.title, &(todo.completed as i32).to_string()],
            ).unwrap();
            println!("result: {}", result);
            // // Create table
            // let mut todos_table = Table::new();

            // // Add a row per time
            // todos_table.add_row(row![b->"ID", b->"Title", b->"Completed"]);
            // todos_table.add_row(row![todo.id, todo.title, todo.completed]);

            // // Print the table to stdout
            // todos_table.printstd();
            
            let json_value = serde_json::to_value(&todo).unwrap();
            return json_value;
        }
        Err(err) => panic!("Error initialising database connection: {}", err)
    }
}

#[tauri::command]
async fn complete_todo(id: String) -> usize {
    let conn = init_db().unwrap();
    let result = conn.execute("UPDATE todos set completed = 1 WHERE (id) = (?1);", [id]).unwrap();
    return result;
}

#[tauri::command]
async fn delete_todo(id: String) -> usize {
    let conn = init_db().unwrap();
    let result = conn.execute("DELETE FROM todos WHERE (id) = (?1);", [id]).unwrap();
    return result;
}

#[tauri::command]
async fn delete_all() -> String {
    let conn = init_db().unwrap();
    let result = conn.execute("DELETE FROM todos", []).unwrap();
    println!("{}", result);
    return result.to_string();
}

#[tauri::command]
async fn show_all() -> Value {
    let result = init_db();

    let conn: Connection;
    match result {
        Ok(connection) => {
            conn = connection;
            let mut stmt = conn.prepare("SELECT * from todos;").unwrap();
            let todos = stmt.query_map((), |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    completed: row.get(2)?,
                })
            }).unwrap();
            let mut todos_array = Vec::new();
            for todo in todos {
                match todo {
                    Ok(todo) => todos_array.push(todo),
                    Err(err) => println!("Error occurred: {:?}", err),
                }
            }
            println!("{:#?}", todos_array);

            // // Create the table
            // let mut todos_table = Table::new();

            // // Add a row per time
            // todos_table.add_row(row![b->"ID", b->"Title", b->"Completed"]);

            // for Todo {
            //     id,
            //     title,
            //     completed,
            // } in v.iter()
            // {
            //     todos_table.add_row(row![id, title, completed]);
            // }

            // // Print the table to stdout
            // todos_table.printstd();
            
            let json_values = serde_json::to_value(todos_array).unwrap();
            return json_values;
        }
        Err(err) => panic!("Error initialising database connection: {}", err)
    }

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_todo, complete_todo, delete_todo, delete_all, show_all])
        .run(tauri::generate_context!())
        .expect("Error occurred while initialising the application!");
}

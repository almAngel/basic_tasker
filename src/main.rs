mod task_holder;
mod tests;

use uuid::Uuid;
use std::io::{BufReader, Read, stdin, Write};
use regex::Regex;
use std::fs::File;
use std::rc::Rc;
use std::str::FromStr;
use chrono::{DateTime, TimeZone, Utc};
use crate::task_holder::task_holder::{TaskHolder, Task, Priority};


fn create_task(th: &mut TaskHolder) {

    println!("Enter task title:");
    let mut input = String::from("");
    stdin().read_line(&mut input).expect("Couldn't get user's input.");
    let title = input.trim().to_string();

    println!("Enter task description:");
    input = String::from("");
    stdin().read_line(&mut input).expect("Couldn't get user's input.");
    let description = input.trim().to_string();

    let priority_match: Priority;
    loop {
        println!("Enter task priority (Critical, Important, CanWait):");
        input = String::from("");
        stdin().read_line(&mut input).expect("Couldn't get user's input.");
        let priority = input.trim().to_lowercase();

        priority_match = match priority.as_str() {
            "critical" => Priority::Critical,
            "important" => Priority::Important,
            "canwait" => Priority::CanWait,
            _ => continue
        };
        break;
    }

    let mut due_date: Option<DateTime<Utc>> = None;
    match priority_match {
        Priority::CanWait => {  },
        _ => loop {
            println!("Enter task's due date (yyyy-mm-dd)/(none):");
            input = String::from("");
            stdin().read_line(&mut input).expect("Couldn't get user's input.");

            let pre_date = match input.trim().to_lowercase().as_str() {
                "none" => Some("none".to_string()),
                _ => Some(input.trim().to_lowercase())
            };

            let date_opt = match Regex::new(
                r"^[0-9]{4}-[0-9]{2}-[0-9]{2}").
                unwrap().
                is_match(&pre_date.clone().unwrap().as_str()) {
                true => {
                    let s = &pre_date.unwrap();

                    let params: Vec<u16> = s.split("-").
                        map(|x| x.to_string().parse::<u16>().unwrap()).
                        collect();

                    Some(Utc.with_ymd_and_hms(params[0] as i32,
                                              params[1] as u32,
                                              params[2] as u32,
                                              0, 0, 0).unwrap()
                    )
                },
                false => break
            };

            due_date = date_opt;
            break;
        }
    }

    let _ = &th.add_task(
        Task {
            id: Some(Uuid::new_v4()),
            title,
            description,
            due_date,
            priority: priority_match,
        }
    );

}

fn edit_task(th: &mut TaskHolder) {

    println!("Enter task id:");
    let mut input = String::from("");
    stdin().read_line(&mut input).expect("Couldn't get user's input.");
    let id = Uuid::from_str(input.trim()).unwrap();

    let task = th.get_task_by_id(Some(&id)).unwrap();

    println!("Enter task title:");
    let mut input = String::from("");
    stdin().read_line(&mut input).expect("Couldn't get user's input.");
    let title = match input.trim() {
        "" => &task.title,
        _ => input.trim()
    }.to_string();

    println!("Enter task description:");
    input = String::from("");
    stdin().read_line(&mut input).expect("Couldn't get user's input.");
    let description =  match input.trim() {
        "" => &task.description,
        _ => input.trim()
    }.to_string();

    let priority: Priority;
    loop {
        println!("Enter task priority (Critical, Important, CanWait):");
        input = String::from("");
        stdin().read_line(&mut input).expect("Couldn't get user's input.");
        priority =  match input.trim() {
            "" => task.priority.clone(),
            _ => match input.trim() {
                "critical" => Priority::Critical,
                "important" => Priority::Important,
                "canwait" => Priority::CanWait,
                _ => continue
            }
        };

        break;
    }

    let mut due_date: Option<DateTime<Utc>> = None;
    match priority {
        Priority::CanWait => {  },
        _ => loop {
            println!("Enter task's due date (yyyy-mm-dd)/(none):");
            input = String::from("");
            stdin().read_line(&mut input).expect("Couldn't get user's input.");

            let pre_date = match input.trim().to_lowercase().as_str() {
                "none" => Some("none".to_string()),
                _ => Some(input.trim().to_lowercase())
            };

            let date_opt = match Regex::new(
                r"^[0-9]{4}-[0-9]{2}-[0-9]{2}").
                unwrap().
                is_match(&pre_date.clone().unwrap().as_str()) {
                true => {
                    let s = &pre_date.unwrap();

                    let params: Vec<u16> = s.split("-").
                        map(|x| x.to_string().parse::<u16>().unwrap()).
                        collect();

                    Some(Utc.with_ymd_and_hms(params[0] as i32,
                                              params[1] as u32,
                                              params[2] as u32,
                                              0, 0, 0).unwrap()
                    )
                },
                false => break
            };

            due_date = date_opt;
            break;
        }
    }

    let _ = &th.edit_task(
        Some(&id),
        Task {
            id: None,
            title,
            description,
            due_date,
            priority,
        }
    );

}

fn delete_task(th: &mut TaskHolder) {

    println!("Enter task id:");
    let mut input = String::from("");
    stdin().read_line(&mut input).expect("Couldn't get user's input.");
    let id = Rc::new(Uuid::from_str(input.trim()).unwrap());

    let task_exists= th.get_task_by_id(Some(&id)).is_some();

    if task_exists {
        let _ = &th.delete_task(Some(&id));
    }

}

fn list_tasks(th: &mut TaskHolder) {

    let tasks = th.get_tasks();

    println!("Printing tasks... \n");
    for task in tasks.iter() {
        println!("{:?}\n", task);
    }

}

fn load_tasks(th: &mut TaskHolder) -> std::io::Result<()> {

    println!("Enter file name:");
    let mut input = String::from("");
    stdin().read_line(&mut input).expect("Couldn't get user's input.");
    let mut filename = input.trim().to_string();

    let file = File::open(&mut filename)?;
    let mut buf_reader = BufReader::new(file);

    let mut data = Rc::new(String::new());
    let _ = buf_reader.read_to_string(Rc::get_mut(&mut data).unwrap());

    let from_json: Vec<Task> = serde_json::from_str(Rc::get_mut(&mut data).unwrap().as_str())?;

    let _ = &th.add_multiple_tasks(from_json);

    Ok(())

}

fn save_tasks(th: &mut TaskHolder) -> std::io::Result<()> {
    let tasks = th.get_tasks();

    let tasks_json: String = serde_json::to_string(&tasks).unwrap();

    println!("Enter file name:");
    let mut input = String::from("");
    stdin().read_line(&mut input).expect("Couldn't get user's input.");
    let filename = input.trim().to_string();

    let mut file = File::create(filename)?;
    file.write_all(&tasks_json.into_bytes())?;
    Ok(())

}

fn main() {

    println!("> WELCOME TO ALM TASK MANAGER\n");

    let mut th: TaskHolder = TaskHolder {
        tasks: &mut vec!()
    };

    loop {
        println!("\
        ---OPTION SELECTOR--- \n\
        > 0. EXIT\n\
        > 1. LIST TASKS\n\
        > 2. ADD A TASK TO THE STACK\n\
        > 3. EDIT TASK BY ID\n\
        > 4. DELETE TASK BY ID\n\
        > 5. LOAD TASKS FROM FILE\n\
        > 6. SAVE TASKS TO FILE
        ");

        let mut input = String::from("");
        stdin().read_line(&mut input).expect("Couldn't get user's input.");

        match input.trim().parse::<u8>() {
            Ok(n) => match n {
                0 => break,
                1 => list_tasks(&mut th),
                2 => create_task(&mut th),
                3 => edit_task(&mut th),
                4 => delete_task(&mut th),
                5 => load_tasks(&mut th).unwrap(),
                6 => save_tasks(&mut th).unwrap(),

                _ => continue,
            },
            Err(_) => continue
        };


    }

}
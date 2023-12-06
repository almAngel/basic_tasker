pub mod task_holder {
    use chrono::{DateTime, Utc};

    use uuid::Uuid;
    use chrono::serde::ts_seconds_option;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Priority {
        Critical,
        Important,
        CanWait
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Task {
        pub(crate) id: Option<Uuid>,
        pub(crate) title: String,
        pub(crate) description: String,
        #[serde(with = "ts_seconds_option")]
        pub(crate) due_date: Option<DateTime<Utc>>,
        pub(crate) priority: Priority,
    }

    #[derive(Debug)]
    pub struct TaskHolder<'a> {
        pub(crate) tasks: &'a mut Vec<Task>,
    }

    impl <'a> TaskHolder<'a> {

        pub fn add_task(&mut self, task: Task) -> (Uuid, bool) {
            let id = task.id.clone().unwrap();
            let _ = &self.tasks.push(task);
            return (id, true);

        }

        pub fn add_multiple_tasks(&mut self, tasks: Vec<Task>) -> bool {

            for task in tasks {
                let _ = &self.tasks.push(task);
            }
            true

        }

        pub fn get_tasks(&mut self) -> &Vec<Task> {
            &self.tasks
        }

        pub fn get_task_by_id(&mut self, id: Option<&Uuid>) -> Option<&Task> {
            for task in self.tasks.iter() {
                if task.id.as_ref() == id {
                    return Some(task);
                }
            }
            return None;

        }

        pub fn edit_task<'b>(&'b mut self, id: Option<&'b Uuid>, new_task: Task) -> (&Uuid, bool) {

            let xec = |task: &mut Task| {
                task.title = new_task.title;
                task.description = new_task.description;
                task.due_date = new_task.due_date;
                task.priority = new_task.priority;
            };

            for (i, task) in &mut self.tasks.clone().iter().enumerate() {
                if task.id.as_ref() == id {
                    xec(&mut self.tasks[i]);
                    return (id.unwrap(), true);
                }
            }
            return (id.unwrap(), false)
        }

        pub fn delete_task(&mut self, id: Option<&Uuid>) -> bool {
            let task_chain = &mut self.tasks;

            for (i, task) in task_chain.iter().enumerate() {
                if task.id.as_ref() == id {
                    let _ = task_chain.remove(i);
                    return true;
                }
            }

            return false;
        }

    }
}
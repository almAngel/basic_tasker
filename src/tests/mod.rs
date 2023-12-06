#[cfg(test)]
pub mod tests {
    use chrono::{TimeZone, Utc};
    use uuid::Uuid;
    use crate::task_holder::task_holder::{TaskHolder, Task, Priority};

    #[test]
    fn test_add_task_ok() {
        let task: fn() -> Task = || Task {
            id: Some(Uuid::new_v4()),
            title: "First task".to_string(),
            description: "My first task".to_string(),
            due_date: Some(Utc.with_ymd_and_hms(2023,
                                                12,
                                                05,
                                                0, 0, 0).unwrap()
            ),
            priority: Priority::Important,
        };

        let mut th: TaskHolder = TaskHolder { tasks: &mut vec!() };

        assert!(th.add_task(task()).1);
    }

    #[test]
    fn test_get_tasks_ok() -> () {

        let task: fn() -> Task = || Task {
            id: Some(Uuid::new_v4()),
            title: "First task".to_string(),
            description: "My first task".to_string(),
            due_date: Some(Utc.with_ymd_and_hms(2023,
                                                12,
                                                05,
                                                0, 0, 0).unwrap()
            ),
            priority: Priority::Important,
        };

        let mut th: TaskHolder = TaskHolder { tasks: &mut vec!() };

        let _ = &th.add_task(task());

        let res = th.get_tasks();

        assert!(res.last().is_some());
    }

    #[test]
    fn test_get_tasks_ko() -> () {

        let mut th: TaskHolder = TaskHolder { tasks: &mut vec!() };

        let res = th.get_tasks();

        assert!(res.last().is_none());
    }

    #[test]
    fn test_edit_task_ok() {
        let task: fn() -> Task = || Task {
            id: Some(Uuid::new_v4()),
            title: "First task".to_string(),
            description: "My first task".to_string(),
            due_date: Some(Utc.with_ymd_and_hms(2023,
                                                12,
                                                05,
                                                0, 0, 0).unwrap()
            ),
            priority: Priority::Important,
        };

        let mut th: TaskHolder = TaskHolder { tasks: &mut vec!() };

        let res = th.add_task(task());

        assert!(th.edit_task(Some(&res.0), task()).1);
    }

    #[test]
    fn test_delete_task_ok() {
        let task: fn() -> Task = || Task {
            id: Some(Uuid::new_v4()),
            title: "First task".to_string(),
            description: "My first task".to_string(),
            due_date: Some(Utc.with_ymd_and_hms(2023,
                                                12,
                                                05,
                                                0, 0, 0).unwrap()
            ),
            priority: Priority::Important,
        };

        let mut th: TaskHolder = TaskHolder { tasks: &mut vec!() };

        let res = th.add_task(task());

        assert!(th.delete_task(Some(&res.0)));
    }
}

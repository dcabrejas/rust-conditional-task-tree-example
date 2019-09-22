use std::io;

pub enum TaskOk {
    Continue,
    SkipSubTasks
}

struct Task {
    action: Box<dyn Fn() -> Result<TaskOk, String>>,
    sub_tasks: Vec<Task>
}

fn create_msg_action(msg: String) -> Box<dyn Fn() -> Result<TaskOk, String>> {
    Box::new(move || {
        println!("{}", msg);
        Result::Ok(TaskOk::Continue)
    })
}

fn create_bool_question_action(question: String) -> Box<dyn Fn() -> Result<TaskOk, String>> {

    Box::new(move || {
        loop {
            println!("{} Y/n", question);
            let mut input = String::new();
            io::stdin().read_line(&mut input).ok().expect("Couldn't read line");

            match input.trim() {
                "y" | "Y" => {
                    return Ok(TaskOk::Continue)
                },
                "n" | "N" => {
                    return Ok(TaskOk::SkipSubTasks)
                },
                _ => {
                    println!("Sorry, we didn't recognise that answer, try again");
                    continue;
                }
            };
        }
    })
}

fn execute_tasks(tasks: &Vec<Task>) -> Result<(), String> {
    for task in tasks.iter() {
        match (task.action)() {
            Ok(proceed) => {
                match proceed {
                    TaskOk::SkipSubTasks => continue,
                    TaskOk::Continue => {
                        execute_tasks(&task.sub_tasks)?
                    }
                };
            },
            Err(err) => { return Err(err)},
        };
    };

    Ok(())
}


fn main() {

    let task_tree = vec![
        Task {
            action: create_msg_action(String::from("Task #1")),
            sub_tasks: vec![
                Task {
                    action: create_msg_action(String::from("  Task #1.1")),
                    sub_tasks: vec![]
                },
                Task {
                    action: create_bool_question_action(String::from("Would you like to execute tasks 2.x?")),
                    sub_tasks: vec![
                        Task {
                            action: create_msg_action(String::from("  Task #2.1")),
                            sub_tasks: vec![]
                        },
                        Task {
                            action: create_msg_action(String::from("  Task #2.1")),
                            sub_tasks: vec![]
                        },
                    ]
                }
            ]
        }
    ];

    match execute_tasks(&task_tree) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Err {}", e),
    }
}

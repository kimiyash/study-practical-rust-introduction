type UserName = String;

#[derive(Debug)]
enum Task {
    Open,
    AssignedTo(UserName),
    Working {
        assignee: UserName,
        remaining_hours: u16,
    },
    Done,
}

use crate::Task::*;

enum Status {
    Active(u32),
    Inactive,
    Pending,
}

mod shape {
    #[derive(Default)]
    pub struct Polygon {
        pub vertexes: Vec<(i32, i32)>,
        pub stroke_width: u8,
        pub fill: (u8, u8, u8),
        internal_id: String,
    }
}

use shape::Polygon;

struct  StrRefs<'a> {
    s1: &'a str,
    s2: &'a str,
}

fn main() {
    let tasks = [
        AssignedTo(String::from("junko")),
        Working {
            assignee: String::from("hiro"),
            remaining_hours: 18,
        },
        Working {
            assignee: String::from("kazuo"),
            remaining_hours: 10,
        },
        Done,
        Open,
    ];

    for (i, task) in tasks.iter().enumerate() {
        match task {
            AssignedTo(assignee) => {
                println!("タスク{}は{}さんにアサインされてます", i, assignee)
            }
            Working {
                assignee,
                remaining_hours,
            } => {
                println!(
                    "タスク{}は{}さんが作業中です。残り{}時間の見込み",
                    i, assignee, remaining_hours
                )
            }
            _ => println!("タスク{}はその他のステータス（{:?}）です", i, task),
        }
    }

    let statuses = [
        Status::Active(10),
        Status::Active(3),
        Status::Inactive,
        Status::Pending,
    ];

    for status in statuses {
        match status {
            Status::Active(value) if value > 5 => {
                println!("Active with value greater than 5: {}", value);
            }
            Status::Active(value) => {
                println!("Active with value: {}", value);
            }
            Status::Inactive => {
                println!("Inactive");
            }
            Status::Pending => {
                println!("Pending");
            }
        }
    }
}

use crate::model::status::Status;

#[derive(Debug)]
pub struct TodoItem {
    pub todo: String,
    pub info: String,
    pub status: Status,
}

impl TodoItem {
    pub fn new(status: Status, todo: &str, info: &str) -> Self {
        Self {
            status,
            todo: todo.to_string(),
            info: info.to_string(),
        }
    }
}


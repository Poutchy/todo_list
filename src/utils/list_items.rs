use ratatui::widgets::ListState;


use crate::utils::item::TodoItem;
use crate::utils::status::Status;

pub struct TodoList {
    pub items: Vec<TodoItem>,
    pub state: ListState,
}

impl TodoList {
    pub fn push(&mut self, item: TodoItem) {
        self.items.push(item);
    }
}

impl FromIterator<(Status, &'static str, &'static str)> for TodoList {
    fn from_iter<I: IntoIterator<Item = (Status, &'static str, &'static str)>>(iter: I) -> Self {
        let items = iter
            .into_iter()
            .map(|(status, todo, info)| TodoItem::new(status, todo, info))
            .collect();
        let state = ListState::default();
        Self { items, state }
    }
}


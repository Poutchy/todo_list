/// App struct file
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use std::char;

use crate::utils::{
    app_state::State, edit_state::CurrentlyEditing, item::TodoItem, list_items::TodoList, status::Status
}

pub struct App {
    pub application_state: State,
    pub currently_editing: Option<CurrentlyEditing>,
    pub description_input: String,
    pub name_input: String,
    pub should_exit: bool,
    pub todo_list: TodoList,
}

impl Default for App {
    fn default() -> Self {
        Self {
            application_state: State::Look,
            currently_editing: None,
            description_input: String::new(),
            name_input: String::new(),
            should_exit: false,
            todo_list: TodoList::from_iter([
                (Status::Todo, "Rewrite everything with Rust!", "I can't hold my inner voice. He tells me to rewrite the complete universe with Rust"),
                (Status::Completed, "Rewrite all of your tui apps with Ratatui", "Yes, you heard that right. Go and replace your tui with Ratatui."),
                (Status::Todo, "Pet your cat", "Minnak loves to be pet by you! Don't forget to pet and give some treats!"),
                (Status::Todo, "Walk with your dog", "Max is bored, go walk with him!"),
                (Status::Completed, "Pay the bills", "Pay the train subscription!!!"),
                (Status::Completed, "Refactor list example", "If you see this info that means I completed this task!"),
            ]),
        }
    }
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }
        }
        Ok(())
    }
    
    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match self.application_state {
            State::Look => {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.should_exit = true,
                    KeyCode::Char('h') | KeyCode::Left => self.select_none(),
                    KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
                    KeyCode::Char('g') | KeyCode::Home => self.select_first(),
                    KeyCode::Char('G') | KeyCode::End => self.select_last(),
                    KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                        self.toggle_status();
                    }
                    KeyCode::Char('e') => self.start_editing(), 
                    _ => {}
                }
            },
            State::Edit => {
                match key.code {
                    KeyCode::Char(value) => self.edit(value),
                    KeyCode::Esc => self.quit_edit(),
                    KeyCode::Tab => self.toggle_edit(),
                    KeyCode::Backspace => self.remove(),
                    KeyCode::Enter => self.validate_edit(),
                    _ => {},
                }
            },
            State::Valid => {
                match key.code {
                    KeyCode::Enter | KeyCode::Char('y') => self.save_edit(),
                    KeyCode::Esc | KeyCode::Char('n') => self.cancel_edit(),
                    KeyCode::Char('e') => self.quit_validation(),
                    _ => {},
                }
            },
        }
    }

    fn cancel_edit(&mut self) {}

    fn edit(&mut self, value: char) {}

    fn quit_edit(&mut self) {}
    
    fn quit_validation(&mut self) {}
    
    fn save_edit(&mut self) {
        let name = self.name_input.clone();
        let description = self.description_input.clone();

        let new_item = TodoItem::new(Status::Todo, &name, &description);
        self.todo_list.push(new_item);
    }
    
    fn select_none(&mut self) {
        self.todo_list.state.select(None);
    }
    
    fn select_next(&mut self) {
        self.todo_list.state.select_next();
    }
    
    fn select_previous(&mut self) {
        self.todo_list.state.select_previous();
    }
    
    fn select_first(&mut self) {
        self.todo_list.state.select_first();
    }
    
    fn select_last(&mut self) {
        self.todo_list.state.select_last();
    }

    // Start the edition of a new task
    fn start_editing(&mut self) {
        self.toggle_status();
        self.toggle_edit();
    }

    // Toggle the input you are currently editing
    fn toggle_edit(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Name => self.currently_editing = Some(CurrentlyEditing::Description),
                CurrentlyEditing::Description => self.currently_editing = Some(CurrentlyEditing::Name),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Name);
        }
    }
    
    /// Changes the status of the selected list item
    fn toggle_status(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            self.todo_list.items[i].status = match self.todo_list.items[i].status {
                Status::Completed => Status::Todo,
                Status::Todo => Status::Completed,
            }
        }
    }

    // Remove a caracter from the edited input
    fn remove(&mut self) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                CurrentlyEditing::Name => {
                    self.name_input.pop();
                }
                CurrentlyEditing::Description => {
                    self.description_input.pop();
                }
            }
        }
    }

    fn validate_edit(&mut self) {

    }
}


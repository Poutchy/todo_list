use std::rc::Rc;

use ratatui::buffer::Buffer;
use ratatui::layout::{
    Constraint,
    Direction,
    Layout,
    Rect
};
use ratatui::style::{
    Color,
    Stylize,
    Modifier,
    Style,
    palette::tailwind::{
        BLUE,
        GREEN,
        SLATE
    },
};
use ratatui::symbols;
use ratatui::text::Line;
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, Padding, Paragraph,
    StatefulWidget, Widget, Wrap,
};

use crate::app::App;
use crate::model::{
    item::TodoItem,
    status::Status,
    app_state::State,
    edit_state::CurrentlyEditing,
};
use crate::utils::ui_help::centered_rect;

pub const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
pub const NORMAL_ROW_BG: Color = SLATE.c950;
pub const ALT_ROW_BG_COLOR: Color = SLATE.c900;
pub const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
pub const TEXT_FG_COLOR: Color = SLATE.c200;
pub const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;


impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);
        let [list_area, item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);
        let popup_area = centered_rect(60, 25, area);
        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        match self.application_state {
            State::Edit => {
                self.render_popup(popup_area, buf);
            },
            State::Look => {
                self.render_list(list_area, buf);
                self.render_selected_item(item_area, buf);
            },
            State::Valid => {},
        }
    }
}

/// Rendering logic for the app
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Ratatui Todo List Example")
            .bold()
            .centered()
            .render(area, buf);
    }
    
    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }
    
    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("TODO List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);
        
        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .todo_list
            .items
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();
    
        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);
        
        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.todo_list.state);
    }

    fn render_popup(&mut self, area: Rect, buf: &mut Buffer) {
        Block::default()
            .title("Enter a new name-description pair")
            .borders(Borders::NONE)
            .style(TODO_HEADER_STYLE)
            .render(area, buf);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        self.render_popup_text(popup_chunks, buf);
    }

    fn render_popup_text(&mut self, area: Rc<[Rect]>, buf: &mut Buffer) {
        if let Some(editing) = &self.currently_editing {
            let mut name_block = Block::default()
                .title("Name")
                .borders(Borders::ALL);

            let mut description_block = Block::default()
                .title("Description")
                .borders(Borders::ALL);
    
            let active_style = Style::default().bg(ALT_ROW_BG_COLOR).fg(TEXT_FG_COLOR);

            match editing {
                CurrentlyEditing::Name => name_block = name_block.style(active_style),
                CurrentlyEditing::Description => description_block = description_block.style(active_style),
            };

            Paragraph::new(self.name_input.clone())
                .block(name_block)
                .render(area[0], buf);

            Paragraph::new(self.description_input.clone())
                .block(description_block)
                .render(area[1], buf);
        }
    }

    
    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.todo_list.state.selected() {
            match self.todo_list.items[i].status {
                Status::Completed => format!("✓ DONE: {}", self.todo_list.items[i].info),
                Status::Todo => format!("☐ TODO: {}", self.todo_list.items[i].info),
            }
        } else {
            "Nothing selected...".to_string()
        };
    
        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("TODO Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));
        
        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl From<&TodoItem> for ListItem<'_> {
    fn from(value: &TodoItem) -> Self {
        let line = match value.status {
            Status::Todo => Line::styled(format!(" ☐ {}", value.todo), TEXT_FG_COLOR),
            Status::Completed => {
                Line::styled(format!(" ✓ {}", value.todo), COMPLETED_TEXT_FG_COLOR)
            }
        };
        
        ListItem::new(line)
    }
}


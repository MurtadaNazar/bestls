// src/table.rs
use crate::fsops::FileEntry;
use tabled::settings::object::{Columns, Rows};
use tabled::settings::{Color, Style};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct DisplayEntry {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    e_type: String,
    #[tabled(rename = "Size")]
    human_size: String,
    #[tabled(rename = "Modified")]
    modified: String,
    #[tabled(rename = "Permissions")]
    permissions: String,
    #[tabled(rename = "Owner")]
    owner: String,
    #[tabled(rename = "Group")]
    group: String,
}

pub fn print_table(entries: Vec<FileEntry>) {
    let data: Vec<DisplayEntry> = entries
        .iter()
        .map(|e: &FileEntry| DisplayEntry {
            name: e.name.clone(),
            e_type: e.e_type.to_string(),
            human_size: e.human_size.clone(),
            modified: e.modified.clone(),
            permissions: e.permissions.clone(),
            owner: e.owner.clone(),
            group: e.group.clone(),
        })
        .collect();

    let mut table: Table = Table::new(data);
    table.with(Style::rounded());
    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
    table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
    println!("{table}");
}

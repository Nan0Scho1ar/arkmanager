use crate::ark::{ArkServerMod};
use crate::db::{read_db};
use tui::{
    layout::{Alignment, Constraint},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table,
    },
};

pub fn home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Welcome")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("to")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::styled(
            "ark_server-CLI",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    home
}

pub fn view_ark_server<'a>(ark_server_list_state: &ListState) -> Table<'a> {
    let ark_server_list = read_db().expect("can fetch ark_server list");
    let selected_ark_server = ark_server_list
        .get(
            ark_server_list_state
                .selected()
                .expect("there is always a selected ark_server"),
        )
        .expect("exists")
        .clone();

    let mods_str = selected_ark_server.mods.into_iter().map(|i| i.name + &", ".to_string()).collect::<String>();

    let ark_server_detail = Table::new(vec![
        Row::new(vec![
            Cell::from(Span::raw("ID:".to_string())),
            Cell::from(Span::raw(selected_ark_server.id.to_string())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Name:".to_string())),
            Cell::from(Span::raw(selected_ark_server.name)),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Category:".to_string())),
            Cell::from(Span::raw(selected_ark_server.category)),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Age:".to_string())),
            Cell::from(Span::raw(selected_ark_server.age.to_string())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Created At:".to_string())),
            Cell::from(Span::raw(selected_ark_server.created_at.to_string())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Mods:".to_string())),
            Cell::from(Span::raw(mods_str)),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Server Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ]);

    ark_server_detail
}

pub fn edit_ark_server_mod<'a>(ark_server_list_state: &ListState, ark_server_mod_list_state: &ListState) -> Table<'a> {
    let ark_server_list = read_db().expect("can fetch ark_server list");
    let selected_ark_server = ark_server_list
        .get(
            ark_server_list_state
                .selected()
                .expect("there is always a selected ark_server"),
        )
        .expect("exists")
        .clone();

    let selected_ark_server_mod = selected_ark_server.mods
        .get(
            ark_server_mod_list_state
                .selected()
                .expect("there is always a selected ark_server"),
        )
        .expect("exists")
        .clone();

    let ark_server_mod_detail = Table::new(vec![
        Row::new(vec![
            Cell::from(Span::raw("ID:".to_string())),
            Cell::from(Span::raw(selected_ark_server_mod.id.to_string())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Name:".to_string())),
            Cell::from(Span::raw(selected_ark_server_mod.name)),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Category:".to_string())),
            Cell::from(Span::raw(selected_ark_server_mod.category)),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Age:".to_string())),
            Cell::from(Span::raw(selected_ark_server_mod.age.to_string())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Created At:".to_string())),
            Cell::from(Span::raw(selected_ark_server_mod.created_at.to_string())),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Mod Detail")
            .border_type(BorderType::Plain),
    )
    .highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
    ]);

    ark_server_mod_detail
}

pub fn view_ark_server_mod<'a>(ark_server_list_state: &ListState, ark_server_mod_list_state: &ListState) -> Table<'a> {
    let ark_server_list = read_db().expect("can fetch ark_server list");
    let selected_ark_server = ark_server_list
        .get(
            ark_server_list_state
                .selected()
                .expect("there is always a selected ark_server"),
        )
        .expect("exists")
        .clone();

    let selected_ark_server_mod = selected_ark_server.mods
        .get(
            ark_server_mod_list_state
                .selected()
                .expect("there is always a selected ark_server"),
        )
        .expect("exists")
        .clone();

    let ark_server_mod_detail = Table::new(vec![
        Row::new(vec![
            Cell::from(Span::raw("ID:".to_string())),
            Cell::from(Span::raw(selected_ark_server_mod.id.to_string())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Name:".to_string())),
            Cell::from(Span::raw(selected_ark_server_mod.name)),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Category:".to_string())),
            Cell::from(Span::raw(selected_ark_server_mod.category)),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Age:".to_string())),
            Cell::from(Span::raw(selected_ark_server_mod.age.to_string())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Created At:".to_string())),
            Cell::from(Span::raw(selected_ark_server_mod.created_at.to_string())),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Mod Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
    ]);

    ark_server_mod_detail
}

pub fn ark_server_mods<'a>(ark_server_list_state: &ListState, ark_server_mod_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let ark_server_mods = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Server Mods")
        .border_type(BorderType::Plain);

    let ark_server_list = read_db().expect("can fetch ark_server list");

    let selected_ark_server = ark_server_list
        .get(
            ark_server_list_state
                .selected()
                .expect("there is always a selected ark_server"),
        )
        .expect("exists")
        .clone();

    let items: Vec<_> = selected_ark_server.mods
        .iter()
        .map(|ark_server| {
            ListItem::new(Spans::from(vec![Span::styled(
                ark_server.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let list = List::new(items).block(ark_server_mods).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let selected_ark_server_mod = if selected_ark_server.mods.len() > 0 {
        selected_ark_server.mods
            .get(
                ark_server_mod_list_state
                    .selected()
                    .expect("there is always a selected ark_server"),
            )
            .expect("exists")
            .clone()
    } else { ArkServerMod::new() };

    let ark_server_mod_detail = if selected_ark_server_mod.id != 0 {
        Table::new(vec![Row::new(vec![
            Cell::from(Span::raw(selected_ark_server_mod.id.to_string())),
            Cell::from(Span::raw(selected_ark_server_mod.name)),
            Cell::from(Span::raw(selected_ark_server_mod.category)),
            Cell::from(Span::raw(selected_ark_server_mod.age.to_string())),
            Cell::from(Span::raw(selected_ark_server_mod.created_at.to_string())),
        ])])
    } else {
        Table::new(vec![Row::new(vec![
            Cell::from(Span::raw("".to_string())),
            Cell::from(Span::raw("".to_string())),
            Cell::from(Span::raw("".to_string())),
            Cell::from(Span::raw("".to_string())),
            Cell::from(Span::raw("".to_string())),
        ])])
    }
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Category",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Created At",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Mod Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
    ]);
    (list, ark_server_mod_detail)

}

pub fn ark_servers<'a>(ark_server_list_state: &ListState) -> (List<'a>, Table<'a>) {
    let ark_servers = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Servers")
        .border_type(BorderType::Plain);

    let ark_server_list = read_db().expect("can fetch ark_server list");
    let items: Vec<_> = ark_server_list
        .iter()
        .map(|ark_server| {
            ListItem::new(Spans::from(vec![Span::styled(
                ark_server.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_ark_server = ark_server_list
        .get(
            ark_server_list_state
                .selected()
                .expect("there is always a selected ark_server"),
        )
        .expect("exists")
        .clone();

    let list = List::new(items).block(ark_servers).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );
    let mods_str = selected_ark_server.mods.len().to_string();

    let ark_server_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_ark_server.id.to_string())),
        Cell::from(Span::raw(selected_ark_server.name)),
        Cell::from(Span::raw(selected_ark_server.category)),
        Cell::from(Span::raw(selected_ark_server.age.to_string())),
        Cell::from(Span::raw(selected_ark_server.created_at.to_string())),
        Cell::from(Span::raw(mods_str)),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "ID",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Category",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Age",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Created At",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Mods",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Server Detail")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(5),
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(5),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ]);

    (list, ark_server_detail)
}

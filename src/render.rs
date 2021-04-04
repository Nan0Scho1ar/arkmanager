use crate::ark::{RenderState, ArkServerMod, Error, MenuItem};
use crate::db::{read_db};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};
use std::io;

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

pub fn render(terminal: &mut tui::Terminal<CrosstermBackend<io::Stdout>>, state: &mut RenderState) -> Result<(), Error> {
    terminal.draw(|rect| {
        let size = rect.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(size);

        let copyright = Paragraph::new("ark_server_manager-CLI 2021 - all rights reserved")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Copyright")
                    .border_type(BorderType::Plain),
            );

        let menu = state.menu_titles
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Spans::from(vec![
                    Span::styled(
                        first,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::styled(rest, Style::default().fg(Color::White)),
                ])
            })
            .collect();

        let tabs = Tabs::new(menu)
            .select(state.active_menu_highlight.into())
            .block(Block::default().title("Menu").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(Span::raw("|"));

        rect.render_widget(tabs, chunks[0]);
        match state.active_menu_item {
            MenuItem::Home => {
                state.menu_titles = vec!["Home", "Servers", "Quit"];
                state.active_menu_highlight = MenuItem::Home;
                rect.render_widget(home(), chunks[1]);
            }
            MenuItem::Servers => {
                state.menu_titles = vec!["Home", "Servers", "Add", "Delete", "Quit"];
                state.active_menu_highlight = MenuItem::Servers;
                let ark_servers_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                    )
                    .split(chunks[1]);
                let (left, right) = ark_servers(&state.ark_server_list_state);
                rect.render_stateful_widget(left, ark_servers_chunks[0], &mut state.ark_server_list_state);
                rect.render_widget(right, ark_servers_chunks[1]);
            }
            MenuItem::ViewServer => {
                state.menu_titles = vec!["Home", "Servers", "Mods", "Back", "Quit"];
                state.active_menu_highlight = MenuItem::Servers;
                rect.render_widget(view_ark_server(&state.ark_server_list_state), chunks[1]);
            }
            MenuItem::ServerMods => {
                state.menu_titles = vec!["Home", "Servers", "Mods", "Add", "Delete", "Back", "Quit"];
                state.active_menu_highlight = MenuItem::ServerMods;
                let ark_servers_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                    )
                    .split(chunks[1]);
                let (left, right) = ark_server_mods(&state.ark_server_list_state, &state.ark_server_mod_list_state);
                rect.render_stateful_widget(left, ark_servers_chunks[0], &mut state.ark_server_mod_list_state);
                rect.render_widget(right, ark_servers_chunks[1]);
            }
            MenuItem::ViewMod => {
                state.menu_titles = vec!["Home", "Servers", "Mods", "Toggle", "Edit", "Back", "Quit"];
                state.active_menu_highlight = MenuItem::ServerMods;
                rect.render_widget(view_ark_server_mod(&state.ark_server_list_state, &state.ark_server_mod_list_state), chunks[1]);
            }
            MenuItem::EditMod => {
                state.menu_titles = vec!["Home", "Servers", "Mods", "Back", "Quit"];
                state.active_menu_highlight = MenuItem::ServerMods;
                let left = edit_ark_server_mod(&state.ark_server_list_state, &state.ark_server_mod_list_state);
                rect.render_stateful_widget(left, chunks[1], &mut state.ark_server_mod_list_edit_state);
            }
        }
        rect.render_widget(copyright, chunks[2]);
    })?;
    Ok(())
}

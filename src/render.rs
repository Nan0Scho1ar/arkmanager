use crate::ark::{ProgState, ArkServerMod, Error, MenuItem};
use crate::db::{get_servers, get_server, get_server_mod, get_server_mod_property, get_server_mod_properties, get_server_mods_str, get_server_properties};
use crate::service::{status_ark_server};
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

pub fn render(terminal: &mut tui::Terminal<CrosstermBackend<io::Stdout>>, state: &mut ProgState) -> Result<(), Error> {
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
                state.menu_titles = vec!["Home", "List Servers", "Quit"];
                state.active_menu_highlight = MenuItem::Home;
                rect.render_widget(home(state), chunks[1]);
            }
            MenuItem::Servers => {
                state.menu_titles = vec!["Home", "List Servers", "Add", "Delete", "Quit"];
                state.active_menu_highlight = MenuItem::Servers;
                let ark_servers_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                    )
                    .split(chunks[1]);
                let (left, right) = ark_servers(&state);
                rect.render_stateful_widget(left, ark_servers_chunks[0], &mut state.ark_server_list_state);
                rect.render_widget(right, ark_servers_chunks[1]);
            }
            MenuItem::ViewServer => {
                state.menu_titles = vec!["Home", "List Servers", "Mods", "Edit", "Back", "Quit"];
                state.active_menu_highlight = MenuItem::Servers;
                rect.render_widget(view_ark_server(&state), chunks[1]);
            }
            MenuItem::ServerMods => {
                state.menu_titles = vec!["Home", "List Servers", "Mods", "Add", "Delete", "Back", "Quit"];
                state.active_menu_highlight = MenuItem::ServerMods;
                let ark_servers_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(
                        [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                    )
                    .split(chunks[1]);
                let (left, right) = ark_server_mods(&state);
                rect.render_stateful_widget(left, ark_servers_chunks[0], &mut state.ark_server_mod_list_state);
                rect.render_widget(right, ark_servers_chunks[1]);
            }
            MenuItem::ViewMod => {
                state.menu_titles = vec!["Home", "List Servers", "Mods", "Toggle", "Edit", "Back", "Quit"];
                state.active_menu_highlight = MenuItem::ServerMods;
                rect.render_widget(view_ark_server_mod(&state.ark_server_list_state, &state.ark_server_mod_list_state), chunks[1]);
            }
            MenuItem::EditMod => {
                state.menu_titles = vec!["Home", "List Servers", "Mods", "Back", "Quit"];
                state.active_menu_highlight = MenuItem::ServerMods;
                let left = edit_ark_server_mod(&state);
                rect.render_stateful_widget(left, chunks[1], &mut state.ark_server_mod_list_edit_state);
            }
            MenuItem::EditServer => {
                state.menu_titles = vec!["Home", "List Servers", "Mods", "Back", "Quit"];
                state.active_menu_highlight = MenuItem::Servers;
                let left = edit_ark_server(&state);
                rect.render_stateful_widget(left, chunks[1], &mut state.ark_server_list_edit_state);
            }
        }
        rect.render_widget(copyright, chunks[2]);
    })?;
    Ok(())
}

pub fn home<'a>(state: &mut ProgState) -> Paragraph<'a> {
    let servers = get_servers().expect("servers exist");
    if servers.len() < 1 {
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
    else {
        let mut server_status = Vec::<Spans>::new();
        let mut i = 0;
        for server in servers {
            state.ark_server_list_state.select(Some(i));
            let status = status_ark_server(state).unwrap();
            let line = server.name + ":     " + &status.to_string();
            server_status.push(Spans::from(vec![Span::raw(line)]));
            i = i+1;
        }
        let home = Paragraph::new(server_status)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Home")
                .border_type(BorderType::Plain),
        );
        home
    }
}

pub fn view_ark_server<'a>(state: &ProgState) -> Table<'a> {
    let selected_ark_server = get_server(state).expect("Server exists");
    let mods_str = get_server_mods_str(state).expect("Mod str exists");
    let server_status = status_ark_server(state).unwrap();

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
            Cell::from(Span::raw("Service Name:".to_string())),
            Cell::from(Span::raw(selected_ark_server.service_name.clone())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Created At:".to_string())),
            Cell::from(Span::raw(selected_ark_server.created_at.to_string())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Mods:".to_string())),
            Cell::from(Span::raw(mods_str)),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Status:".to_string())),
            Cell::from(Span::raw(server_status)),
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
        Constraint::Percentage(20),
        Constraint::Percentage(80),
    ]);

    ark_server_detail
}

pub fn edit_ark_server<'a>(state: &ProgState) -> Table<'a> {
    let sel = state.get_server_edit_index();

    let mut vals = get_server_properties(state).expect("server mod has properties");
        if sel < vals.len() && state.editing_server {
        vals[sel] = state.tmp_server_field.to_string();
    }

    let ark_server_detail = Table::new(vec![
        Row::new(vec![
            Cell::from(Span::raw("ID:".to_string())),
            Cell::from(Span::raw(vals[0].clone())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Name:".to_string())),
            Cell::from(Span::raw(vals[1].clone())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Category:".to_string())),
            Cell::from(Span::raw(vals[2].clone())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Age:".to_string())),
            Cell::from(Span::raw(vals[3].clone())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Service Name:".to_string())),
            Cell::from(Span::raw(vals[4].clone())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Created At:".to_string())),
            Cell::from(Span::raw(vals[5].clone())),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Server Detail")
            .border_type(BorderType::Plain),
    )
    .highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    )
    .widths(&[
        Constraint::Percentage(20),
        Constraint::Percentage(90),
    ]);

    ark_server_detail
}

pub fn edit_ark_server_mod<'a>(state: &ProgState) -> Table<'a> {
    let sel = state.get_mod_edit_index();

    let mut vals = get_server_mod_properties(state).expect("server mod has properties");
        if sel < vals.len() && state.editing_mod {
        vals[sel] = state.tmp_mod_field.to_string();
    }



    let ark_server_mod_detail = Table::new(vec![
        Row::new(vec![
            Cell::from(Span::raw("ID:".to_string())),
            Cell::from(Span::raw(vals[0].clone())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Name:".to_string())),
            Cell::from(Span::raw(vals[1].clone())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Category:".to_string())),
            Cell::from(Span::raw(vals[2].clone())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Age:".to_string())),
            Cell::from(Span::raw(vals[3].clone())),
        ]),
        Row::new(vec![
            Cell::from(Span::raw("Created At:".to_string())),
            Cell::from(Span::raw(vals[4].clone())),
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
        Constraint::Percentage(20),
        Constraint::Percentage(80),
    ]);

    ark_server_mod_detail
}

pub fn view_ark_server_mod<'a>(ark_server_list_state: &ListState, ark_server_mod_list_state: &ListState) -> Table<'a> {
    let ark_server_list = get_servers().expect("can fetch ark_server list");
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
        Constraint::Percentage(20),
        Constraint::Percentage(80),
    ]);

    ark_server_mod_detail
}

pub fn ark_server_mods<'a>(state: &ProgState) -> (List<'a>, Table<'a>) {
    let ark_server_mods = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Server Mods")
        .border_type(BorderType::Plain);


    let selected_ark_server = get_server(state).expect("server exists");

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

    let selected_ark_server_mod = get_server_mod(state).expect("server mod exists");

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

pub fn ark_servers<'a>(state: &ProgState) -> (List<'a>, Table<'a>) {
    let ark_servers = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Servers")
        .border_type(BorderType::Plain);

    let ark_server_list = get_servers().expect("Servers exist");

    let items: Vec<_> = ark_server_list
        .iter()
        .map(|ark_server| {
            ListItem::new(Spans::from(vec![Span::styled(
                ark_server.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_ark_server = get_server(state).expect("Server exists");
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

//    let headings = vec![
//        "ID",
//        "Name",
//        "Category",
//        "Age",
//        "Created At",
//        "Mods",
//    ];
//    .add_table_header(headings)
//trait AddTableHeader {
//    fn add_table_header(&self, headings:Vec<&str>);
//}
//
//impl AddTableHeader for Table<'_> {
//    fn add_table_header(&self, headings: Vec<&str>) {
//        let mut row = Vec::<Cell>::new();
//        for heading in headings {
//            row.push(
//                Cell::from(Span::styled(
//                    heading,
//                    Style::default().add_modifier(Modifier::BOLD),
//                )),
//            );
//        }
//        self.header(Row::new(row));
//    }
//}

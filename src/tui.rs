use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap,
    },
    Frame, Terminal,
};
use std::io;
use anyhow::Result;
use crate::models::*;

pub struct App {
    pub game_data: GameData,
    pub current_view: View,
    pub route_list_state: ListState,
    pub step_list_state: ListState,
    pub should_quit: bool,
    pub show_help: bool,
    pub step_indices: Vec<(usize, usize)>,
    pub display_to_step_mapping: Vec<Option<usize>>,
}

#[derive(Debug, Clone)]
pub enum View {
    RouteSelection,
    RouteDetails { route_idx: usize },
    StepDetails { route_idx: usize, chapter_idx: usize, step_idx: usize },
}

impl App {
    pub fn new(game_data: GameData) -> Self {
        let mut route_list_state = ListState::default();
        route_list_state.select(Some(0));

        Self {
            game_data,
            current_view: View::RouteSelection,
            route_list_state,
            step_list_state: ListState::default(),
            should_quit: false,
            show_help: false,
            step_indices: Vec::new(),
            display_to_step_mapping: Vec::new(),
        }
    }

    pub fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| self.draw(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    self.handle_key_event(key.code);
                }
            }

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyCode) {
        if self.show_help {
            if matches!(key, KeyCode::Char('h') | KeyCode::Esc) {
                self.show_help = false;
            }
            return;
        }

        match key {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('h') => self.show_help = true,
            KeyCode::Esc => self.go_back(),
            KeyCode::Enter => self.select_current(),
            KeyCode::Up => self.move_up(),
            KeyCode::Down => self.move_down(),
            KeyCode::Char(' ') => self.toggle_step_completion(),
            _ => {}
        }
    }

    fn go_back(&mut self) {
        match &self.current_view {
            View::RouteSelection => {}
            View::RouteDetails { .. } => {
                self.current_view = View::RouteSelection;
                self.step_indices.clear();
                self.display_to_step_mapping.clear();
            }
            View::StepDetails { route_idx, .. } => {
                self.current_view = View::RouteDetails { route_idx: *route_idx };
                self.step_list_state = ListState::default();
            }
        }
    }

    fn select_current(&mut self) {
        match &self.current_view {
            View::RouteSelection => {
                if let Some(selected) = self.route_list_state.selected() {
                    if selected < self.game_data.routes.len() {
                        self.current_view = View::RouteDetails { route_idx: selected };
                        self.game_data.current_route = Some(selected);
                        self.build_step_display_mapping(selected);
                        self.step_list_state = ListState::default();
                        if let Some(first_selectable) = self.display_to_step_mapping.iter().position(|x| x.is_some()) {
                            self.step_list_state.select(Some(first_selectable));
                        }
                    }
                }
            }
            View::RouteDetails { route_idx } => {
                if let Some(selected_display_idx) = self.step_list_state.selected() {
                    if let Some(Some(step_idx)) = self.display_to_step_mapping.get(selected_display_idx) {
                        if let Some((chapter_idx, step_idx_in_chapter)) = self.step_indices.get(*step_idx) {
                            self.current_view = View::StepDetails {
                                route_idx: *route_idx,
                                chapter_idx: *chapter_idx,
                                step_idx: *step_idx_in_chapter,
                            };
                        }
                    }
                }
            }
            View::StepDetails { .. } => {}
        }
    }

    fn move_up(&mut self) {
        match &self.current_view {
            View::RouteSelection => {
                let selected = self.route_list_state.selected().unwrap_or(0);
                if selected > 0 {
                    self.route_list_state.select(Some(selected - 1));
                }
            }
            View::RouteDetails { route_idx: _ } => {
                let selected = self.step_list_state.selected().unwrap_or(0);
                if selected > 0 {
                    for i in (0..selected).rev() {
                        if self.display_to_step_mapping.get(i).and_then(|x| x.as_ref()).is_some() {
                            self.step_list_state.select(Some(i));
                            return;
                        }
                    }
                } else {
                    for i in (0..self.display_to_step_mapping.len()).rev() {
                        if self.display_to_step_mapping.get(i).and_then(|x| x.as_ref()).is_some() {
                            self.step_list_state.select(Some(i));
                            return;
                        }
                    }
                }
            }
            View::StepDetails { .. } => {}
        }
    }

    fn move_down(&mut self) {
        match &self.current_view {
            View::RouteSelection => {
                let selected = self.route_list_state.selected().unwrap_or(0);
                if selected < self.game_data.routes.len().saturating_sub(1) {
                    self.route_list_state.select(Some(selected + 1));
                }
            }
            View::RouteDetails { route_idx: _ } => {
                let selected = self.step_list_state.selected().unwrap_or(0);
                let total_items = self.display_to_step_mapping.len();
                
                for i in (selected + 1)..total_items {
                    if self.display_to_step_mapping.get(i).and_then(|x| x.as_ref()).is_some() {
                        self.step_list_state.select(Some(i));
                        return;
                    }
                }
                
                for i in 0..=selected {
                    if self.display_to_step_mapping.get(i).and_then(|x| x.as_ref()).is_some() {
                        self.step_list_state.select(Some(i));
                        return;
                    }
                }
            }
            View::StepDetails { .. } => {}
        }
    }

    fn toggle_step_completion(&mut self) {
        if let View::RouteDetails { route_idx } = &self.current_view {
            if let Some(selected_display_idx) = self.step_list_state.selected() {
                if let Some(Some(step_idx)) = self.display_to_step_mapping.get(selected_display_idx) {
                    if let Some((chapter_idx, step_idx_in_chapter)) = self.step_indices.get(*step_idx) {
                        let route_idx = *route_idx;
                        let chapter_idx = *chapter_idx;
                        let step_idx_in_chapter = *step_idx_in_chapter;
                        
                        let (step_id, new_completion) = if let Some(route) = self.game_data.routes.get(route_idx) {
                            if let Some(chapter) = route.chapters.get(chapter_idx) {
                                if let Some(step) = chapter.steps.get(step_idx_in_chapter) {
                                    (step.id.clone(), !step.completed)
                                } else {
                                    return;
                                }
                            } else {
                                return;
                            }
                        } else {
                            return;
                        };
                        
                        if let Some(route) = self.game_data.routes.get_mut(route_idx) {
                            if let Some(chapter) = route.chapters.get_mut(chapter_idx) {
                                if let Some(step) = chapter.steps.get_mut(step_idx_in_chapter) {
                                    step.completed = new_completion;
                                }
                            }
                        }
                        
                        self.game_data.mark_step_completed(&step_id, new_completion);
                    }
                }
            }
        }
    }

    fn build_step_display_mapping(&mut self, route_idx: usize) {
        self.step_indices.clear();
        self.display_to_step_mapping.clear();
        
        if let Some(route) = self.game_data.routes.get(route_idx) {
            for (chapter_idx, chapter) in route.chapters.iter().enumerate() {
                self.display_to_step_mapping.push(None);
                
                for (step_idx, _) in chapter.steps.iter().enumerate() {
                    let step_indices_idx = self.step_indices.len();
                    self.step_indices.push((chapter_idx, step_idx));
                    self.display_to_step_mapping.push(Some(step_indices_idx));
                }
            }
        }
    }

    fn draw(&mut self, f: &mut Frame) {
        let size = f.area();

        match &self.current_view {
            View::RouteSelection => self.draw_route_selection(f, size),
            View::RouteDetails { route_idx } => self.draw_route_details(f, size, *route_idx),
            View::StepDetails { route_idx, chapter_idx, step_idx } => {
                self.draw_step_details(f, size, *route_idx, *chapter_idx, *step_idx)
            }
        }

        if self.show_help {
            self.draw_help_popup(f, size);
        }
    }

    fn draw_route_selection(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(2),
            ])
            .split(area);

        let title = Paragraph::new("Chaos;Head NoAH - Route Guide")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        let routes: Vec<ListItem> = self.game_data.routes
            .iter()
            .enumerate()
            .map(|(i, route)| {
                let completion = self.game_data.get_completion_percentage(i);
                let available = self.is_route_available(i);
                
                let style = if !available {
                    Style::default().fg(Color::DarkGray)
                } else if completion >= 100.0 {
                    Style::default().fg(Color::Green)
                } else if completion > 0.0 {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::White)
                };

                let status_symbol = if !available {
                    "🔒"
                } else if completion >= 100.0 {
                    "✓"
                } else if completion > 0.0 {
                    "◐"
                } else {
                    "○"
                };

                ListItem::new(Line::from(vec![
                    Span::raw(format!("{} ", status_symbol)),
                    Span::styled(route.name.clone(), style),
                    Span::raw(format!(" ({:.0}%)", completion)),
                ]))
            })
            .collect();

        let routes_list = List::new(routes)
            .block(Block::default().title("Routes").borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::DarkGray))
            .highlight_symbol("► ");

        f.render_stateful_widget(routes_list, chunks[1], &mut self.route_list_state);

        let help_text = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Controls: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("↑/↓: Navigate | Enter: Select | h: Help | q: Quit"),
            ]),
        ])
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(help_text, chunks[2]);
    }

    fn draw_route_details(&mut self, f: &mut Frame, area: Rect, route_idx: usize) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Min(0),
                Constraint::Length(4),
            ])
            .split(area);

        if let Some(route) = self.game_data.routes.get(route_idx) {
            let completion = self.game_data.get_completion_percentage(route_idx);
            
            let header = Paragraph::new(vec![
                Line::from(vec![
                    Span::styled(&route.name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::raw(&route.description),
                ]),
                Line::from(vec![
                    Span::raw(format!("Progress: {:.0}%", completion)),
                ]),
            ])
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(header, chunks[0]);

            let mut display_items: Vec<ListItem> = Vec::new();
            let mut last_seen_chapter: Option<usize> = None;
            
            for (display_idx, step_mapping) in self.display_to_step_mapping.iter().enumerate() {
                match step_mapping {
                    None => {
                        let mut chapter_for_this_none = None;
                        for future_idx in (display_idx + 1)..self.display_to_step_mapping.len() {
                            if let Some(Some(next_step_idx)) = self.display_to_step_mapping.get(future_idx) {
                                if let Some((chapter_idx, _)) = self.step_indices.get(*next_step_idx) {
                                    chapter_for_this_none = Some(*chapter_idx);
                                    break;
                                }
                            }
                        }
                        
                        if let Some(chapter_idx) = chapter_for_this_none {
                            if last_seen_chapter != Some(chapter_idx) {
                                last_seen_chapter = Some(chapter_idx);
                                if let Some(chapter) = route.chapters.get(chapter_idx) {
                                    display_items.push(ListItem::new(Line::from(vec![
                                        Span::styled(
                                            format!("═══ {} ═══", chapter.name),
                                            Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)
                                        ),
                                    ])));
                                }
                            } else {
                                display_items.push(ListItem::new(Line::from("")));
                            }
                        } else {
                            display_items.push(ListItem::new(Line::from("")));
                        }
                    }
                    Some(step_idx) => {
                        if let Some((chapter_idx, step_idx_in_chapter)) = self.step_indices.get(*step_idx) {
                            if last_seen_chapter != Some(*chapter_idx) {
                                last_seen_chapter = Some(*chapter_idx);
                            }
                            
                            if let Some(chapter) = route.chapters.get(*chapter_idx) {
                                if let Some(step) = chapter.steps.get(*step_idx_in_chapter) {
                                    let status_symbol = if step.completed { "✓" } else { "○" };
                                    let style = if step.completed {
                                        Style::default().fg(Color::Green)
                                    } else {
                                        Style::default().fg(Color::White)
                                    };

                                    display_items.push(ListItem::new(Line::from(vec![
                                        Span::raw(format!("  {} ", status_symbol)),
                                        Span::styled(&step.description, style),
                                    ])));
                                }
                            }
                        }
                    }
                }
            }

            let steps_list = List::new(display_items)
                .block(Block::default().title("Steps").borders(Borders::ALL))
                .highlight_style(Style::default().bg(Color::DarkGray))
                .highlight_symbol("► ");

            f.render_stateful_widget(steps_list, chunks[1], &mut self.step_list_state);
        }

        let keybind_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(chunks[2]);

        let nav_help = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Navigation: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from("↑/↓: Move | Enter: Details | Esc: Back"),
        ])
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(nav_help, keybind_chunks[0]);

        let action_help = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Actions: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from("Space: Toggle | h: Help | q: Quit"),
        ])
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(action_help, keybind_chunks[1]);
    }

    fn draw_step_details(&mut self, f: &mut Frame, area: Rect, route_idx: usize, chapter_idx: usize, step_idx: usize) {
        if let Some(route) = self.game_data.routes.get(route_idx) {
            if let Some(chapter) = route.chapters.get(chapter_idx) {
                if let Some(step) = chapter.steps.get(step_idx) {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(3),
                            Constraint::Min(0),
                            Constraint::Length(2),
                        ])
                        .split(area);

                    let status = if step.completed { "✓ COMPLETED" } else { "○ PENDING" };
                    let status_style = if step.completed {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::Yellow)
                    };

                    let title = Paragraph::new(vec![
                        Line::from(vec![
                            Span::styled(&step.description, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                            Span::raw("  "),
                            Span::styled(status, status_style),
                        ]),
                    ])
                    .block(Block::default().borders(Borders::ALL));
                    f.render_widget(title, chunks[0]);

                    let details = self.format_step_details(step);
                    let details_paragraph = Paragraph::new(details)
                        .block(Block::default().title("Details").borders(Borders::ALL))
                        .wrap(Wrap { trim: true });
                    f.render_widget(details_paragraph, chunks[1]);

                    let help_text = Paragraph::new(vec![
                        Line::from(vec![
                            Span::styled("Controls: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                            Span::raw("Esc: Back | h: Help | q: Quit"),
                        ]),
                    ])
                    .style(Style::default().fg(Color::Gray))
                    .block(Block::default().borders(Borders::ALL));
                    f.render_widget(help_text, chunks[2]);
                }
            }
        }
    }

    fn format_step_details<'a>(&self, step: &'a Step) -> Text<'a> {
        match &step.step_type {
            StepType::DelusionTrigger { trigger_number, polarity, location } => {
                Text::from(vec![
                    Line::from(vec![
                        Span::styled("Type: ", Style::default().fg(Color::Yellow)),
                        Span::raw("Delusion Trigger"),
                    ]),
                    Line::from(vec![
                        Span::styled("Trigger Number: ", Style::default().fg(Color::Yellow)),
                        Span::raw(format!("#{}", trigger_number)),
                    ]),
                    Line::from(vec![
                        Span::styled("Polarity: ", Style::default().fg(Color::Yellow)),
                        Span::styled(format!("{:?}", polarity), match polarity {
                            Polarity::Positive => Style::default().fg(Color::Green),
                            Polarity::Negative => Style::default().fg(Color::Red),
                            Polarity::Neutral => Style::default().fg(Color::Blue),
                        }),
                    ]),
                    Line::from(vec![
                        Span::styled("Location: ", Style::default().fg(Color::Yellow)),
                        Span::raw(location),
                    ]),
                ])
            }
            StepType::YesNoPrompts { prompts } => {
                let mut lines = vec![
                    Line::from(vec![
                        Span::styled("Type: ", Style::default().fg(Color::Yellow)),
                        Span::raw("YES/NO Prompts"),
                    ]),
                    Line::from(""),
                ];
                
                for (i, prompt) in prompts.iter().enumerate() {
                    let answer_text = if prompt.answer { "YES" } else { "NO" };
                    let answer_style = if prompt.answer {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::Red)
                    };
                    
                    lines.push(Line::from(vec![
                        Span::styled(format!("{}. ", i + 1), Style::default().fg(Color::Cyan)),
                        Span::raw(&prompt.question),
                    ]));
                    lines.push(Line::from(vec![
                        Span::raw("   Answer: "),
                        Span::styled(answer_text, answer_style),
                    ]));
                    lines.push(Line::from(""));
                }
                
                Text::from(lines)
            }
            StepType::GeneralInstruction { instruction } => {
                Text::from(vec![
                    Line::from(vec![
                        Span::styled("Type: ", Style::default().fg(Color::Yellow)),
                        Span::raw("General Instruction"),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Instructions: ", Style::default().fg(Color::Yellow)),
                    ]),
                    Line::from(instruction.clone()),
                ])
            }
            StepType::Checkpoint { save_point } => {
                Text::from(vec![
                    Line::from(vec![
                        Span::styled("Type: ", Style::default().fg(Color::Yellow)),
                        Span::raw("Save Checkpoint"),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Save Point: ", Style::default().fg(Color::Yellow)),
                        Span::raw(save_point),
                    ]),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Remember to save your game at this point!", Style::default().fg(Color::Magenta)),
                    ]),
                ])
            }
        }
    }

    fn draw_help_popup(&self, f: &mut Frame, area: Rect) {
        let popup_area = centered_rect(60, 50, area);
        f.render_widget(Clear, popup_area);

        let help_text = Text::from(vec![
            Line::from(vec![Span::styled("Help", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
            Line::from(""),
            Line::from("Navigation:"),
            Line::from("  ↑/↓ - Move up/down"),
            Line::from("  Enter - Select/Enter"),
            Line::from("  Esc - Go back"),
            Line::from(""),
            Line::from("Actions:"),
            Line::from("  Space - Toggle step completion"),
            Line::from("  h - Toggle this help"),
            Line::from("  q - Quit application"),
            Line::from(""),
            Line::from("Symbols:"),
            Line::from("  ✓ - Completed"),
            Line::from("  ○ - Not completed"),
            Line::from("  ◐ - Partially completed"),
            Line::from("  🔒 - Prerequisites not met"),
            Line::from(""),
            Line::from("Press h or Esc to close this help."),
        ]);

        let help_popup = Paragraph::new(help_text)
            .block(Block::default().title("Help").borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        f.render_widget(help_popup, popup_area);
    }

    fn is_route_available(&self, route_idx: usize) -> bool {
        if let Some(route) = self.game_data.routes.get(route_idx) {
            for prereq in &route.prerequisites {
                let prereq_completed = self.game_data.routes.iter()
                    .find(|r| r.name == *prereq)
                    .map(|r| {
                        let route_idx = self.game_data.routes.iter().position(|x| x.name == r.name).unwrap();
                        self.game_data.get_completion_percentage(route_idx) >= 100.0
                    })
                    .unwrap_or(false);
                
                if !prereq_completed {
                    return false;
                }
            }
        }
        true
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn run_app() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let routes = crate::data::create_chaos_head_data();
    let mut game_data = GameData::new();
    game_data.routes = routes;

    let app = App::new(game_data);
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

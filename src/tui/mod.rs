use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Terminal;

use crate::cli::{add, doctor_summary, install_selected, load_skill_summaries, remove, sync, update, SkillSummary};

struct App {
    skills: Vec<SkillSummary>,
    selected: usize,
    status: String,
    show_help: bool,
    confirm_action: Option<PendingAction>,
    doctor_output: Option<String>,
    pending_action_result: Option<String>,
    filter_mode: bool,
    filter_query: String,
    add_mode: bool,
    add_input: String,
    sync_target: String,
    command_mode: bool,
    command_query: String,
}

#[derive(Clone, Copy)]
enum PendingAction {
    Install,
    Update,
    Sync,
    Remove,
    Doctor,
}

impl App {
    fn new(skills: Vec<SkillSummary>) -> Self {
        Self {
            skills,
            selected: 0,
            status: "q quit • j/k move • / filter • i install • u update • s sync • x remove • d doctor • ? help".to_string(),
            show_help: false,
            confirm_action: None,
            doctor_output: None,
            pending_action_result: None,
            filter_mode: false,
            filter_query: String::new(),
            add_mode: false,
            add_input: String::new(),
            sync_target: "opencode".to_string(),
            command_mode: false,
            command_query: String::new(),
        }
    }

    fn next(&mut self) {
        if !self.skills.is_empty() {
            self.selected = (self.selected + 1) % self.skills.len();
        }
    }

    fn previous(&mut self) {
        if !self.skills.is_empty() {
            self.selected = if self.selected == 0 {
                self.skills.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    fn refresh(&mut self) {
        match load_skill_summaries() {
            Ok(skills) => {
                self.skills = skills;
                self.normalize_selection();
                self.status = "refreshed summaries".to_string();
            }
            Err(error) => {
                self.status = format!("refresh failed: {}", error);
            }
        }
    }

    fn filtered_indices(&self) -> Vec<usize> {
        if self.filter_query.is_empty() {
            return (0..self.skills.len()).collect();
        }

        let needle = self.filter_query.to_lowercase();
        self.skills
            .iter()
            .enumerate()
            .filter(|(_, skill)| {
                skill.name.to_lowercase().contains(&needle)
                    || skill.outdated.to_lowercase().contains(&needle)
                    || skill.statuses.iter().any(|status| status.to_lowercase().contains(&needle))
                    || skill
                        .description
                        .as_ref()
                        .map(|desc| desc.to_lowercase().contains(&needle))
                        .unwrap_or(false)
            })
            .map(|(index, _)| index)
            .collect()
    }

    fn normalize_selection(&mut self) {
        let filtered = self.filtered_indices();
        if filtered.is_empty() {
            self.selected = 0;
        } else if self.selected >= filtered.len() {
            self.selected = filtered.len() - 1;
        }
    }

    fn selected_filtered_skill(&self) -> Option<&SkillSummary> {
        let filtered = self.filtered_indices();
        filtered
            .get(self.selected)
            .and_then(|skill_index| self.skills.get(*skill_index))
    }

    fn selected_filtered_name(&self) -> Option<String> {
        self.selected_filtered_skill().map(|skill| skill.name.clone())
    }

    fn command_items(&self) -> Vec<&'static str> {
        let commands = vec![
            "add",
            "install",
            "update",
            "sync",
            "remove",
            "doctor",
            "refresh",
            "toggle-target",
            "help",
        ];

        if self.command_query.is_empty() {
            commands
        } else {
            let needle = self.command_query.to_lowercase();
            commands
                .into_iter()
                .filter(|command| command.contains(&needle))
                .collect()
        }
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let skills = load_skill_summaries()?;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new(skills);

    let result = run_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(1)])
                .split(frame.area());

            let body = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
                .split(chunks[0]);

            let filtered_indices = app.filtered_indices();
            let items: Vec<ListItem> = filtered_indices
                .iter()
                .filter_map(|index| app.skills.get(*index))
                .map(|skill| {
                    ListItem::new(Line::from(vec![
                        Span::raw(&skill.name),
                        Span::raw(" "),
                        Span::raw(format!("[{}]", skill.outdated)),
                    ]))
                })
                .collect();

            let mut state = ListState::default();
            if !filtered_indices.is_empty() {
                state.select(Some(app.selected));
            }

            let list = List::new(items)
                .block(Block::default().title("Skills").borders(Borders::ALL))
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
            frame.render_stateful_widget(list, body[0], &mut state);

            let detail_text = if let Some(skill) = app.selected_filtered_skill() {
                vec![
                    Line::from(format!("Source: {}", skill.source)),
                    Line::from(format!("Statuses: {}", skill.statuses.join(", "))),
                    Line::from(format!("Outdated: {}", skill.outdated)),
                    Line::from(format!("Lock: {}", skill.lock_summary)),
                    Line::from(format!(
                        "Version: {}",
                        skill.skill_version.clone().unwrap_or_else(|| "unknown".to_string())
                    )),
                    Line::from(format!(
                        "Manifest Version: {}",
                        skill.manifest_version.clone().unwrap_or_else(|| "legacy".to_string())
                    )),
                    Line::from(format!(
                        "Maturity: {}",
                        skill.maturity.clone().unwrap_or_else(|| "legacy".to_string())
                    )),
                    Line::from(format!(
                        "Last Verified: {}",
                        skill.last_verified.clone().unwrap_or_else(|| "n/a".to_string())
                    )),
                    Line::from(""),
                    Line::from(skill.description.clone().unwrap_or_else(|| "No manifest description available".to_string())),
                ]
            } else {
                vec![Line::from("No skills configured")]
            };

            let detail = Paragraph::new(detail_text)
                .block(Block::default().title("Details").borders(Borders::ALL));
            frame.render_widget(detail, body[1]);

            let footer_text = if app.add_mode {
                format!("add mode • sync target: {}", app.sync_target)
            } else if app.filter_mode {
                format!("filter: {}", app.filter_query)
            } else {
                format!("{} • sync target: {}", app.status, app.sync_target)
            };
            let footer = Paragraph::new(footer_text);
            frame.render_widget(footer, chunks[1]);

            if let Some(action) = app.confirm_action {
                render_modal(
                    frame,
                    centered_rect(60, 20, frame.area()),
                    "Confirm Action",
                    format!("Run {}? Press y to confirm, n to cancel.", action_label(action)),
                );
            }

            if app.show_help {
                render_modal(
                    frame,
                    centered_rect(70, 40, frame.area()),
                    "Help",
                    "j/k or ↑/↓: move\n: command palette\na: add skill\n/: filter list\ni: install selected skill\nu: update selected skill\nt: toggle sync target\ns: sync selected skill target\nx: remove selected skill from config\nd: run doctor summary\nr: refresh\n?: toggle help\nq: quit".to_string(),
                );
            }

            if let Some(output) = &app.doctor_output {
                render_modal(
                    frame,
                    centered_rect(80, 60, frame.area()),
                    "Doctor Summary",
                    output.clone(),
                );
            }

            if let Some(output) = &app.pending_action_result {
                render_modal(
                    frame,
                    centered_rect(70, 30, frame.area()),
                    "Action Result",
                    output.clone(),
                );
            }

            if app.command_mode {
                let commands = app.command_items();
                render_modal(
                    frame,
                    centered_rect(60, 35, frame.area()),
                    "Command Palette",
                    format!(
                        ":{}\n\n{}",
                        app.command_query,
                        commands.join("\n")
                    ),
                );
            }

            if app.add_mode {
                render_modal(
                    frame,
                    centered_rect(70, 20, frame.area()),
                    "Add Skill",
                    format!("Enter repo reference (owner/repo or owner/repo/path):\n{}", app.add_input),
                );
            }
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if app.command_mode {
                    match key.code {
                        KeyCode::Esc => {
                            app.command_mode = false;
                            app.command_query.clear();
                            app.status = "command palette cancelled".to_string();
                        }
                        KeyCode::Enter => {
                            run_command(app)?;
                        }
                        KeyCode::Backspace => {
                            app.command_query.pop();
                        }
                        KeyCode::Char(c) => {
                            app.command_query.push(c);
                        }
                        _ => {}
                    }
                    continue;
                }

                if app.add_mode {
                    match key.code {
                        KeyCode::Esc => {
                            app.add_mode = false;
                            app.add_input.clear();
                            app.status = "add cancelled".to_string();
                        }
                        KeyCode::Enter => {
                            let repo = app.add_input.trim().to_string();
                            if repo.is_empty() {
                                app.status = "repo reference cannot be empty".to_string();
                            } else {
                                let runtime = tokio::runtime::Runtime::new()?;
                                runtime.block_on(add(repo.clone(), None, None))?;
                                app.add_mode = false;
                                app.add_input.clear();
                                app.status = format!("added {}", repo);
                                app.pending_action_result = Some(format!("Added {} to skills.toml. Press i to install it, or run install later.", repo));
                                app.refresh();
                            }
                        }
                        KeyCode::Backspace => {
                            app.add_input.pop();
                        }
                        KeyCode::Char(c) => {
                            app.add_input.push(c);
                        }
                        _ => {}
                    }
                    continue;
                }

                if app.pending_action_result.is_some() {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => app.pending_action_result = None,
                        _ => {}
                    }
                    continue;
                }

                if app.filter_mode {
                    match key.code {
                        KeyCode::Esc => {
                            app.filter_mode = false;
                            app.status = "filter cancelled".to_string();
                        }
                        KeyCode::Enter => {
                            app.filter_mode = false;
                            app.normalize_selection();
                            app.status = if app.filter_query.is_empty() {
                                "filter cleared".to_string()
                            } else {
                                format!("filtered by '{}'", app.filter_query)
                            };
                        }
                        KeyCode::Backspace => {
                            app.filter_query.pop();
                            app.normalize_selection();
                        }
                        KeyCode::Char(c) => {
                            app.filter_query.push(c);
                            app.normalize_selection();
                        }
                        _ => {}
                    }
                    continue;
                }

                if app.doctor_output.is_some() {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => app.doctor_output = None,
                        _ => {}
                    }
                    continue;
                }

                if app.show_help {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('?') => app.show_help = false,
                        _ => {}
                    }
                    continue;
                }

                if let Some(action) = app.confirm_action {
                    match key.code {
                        KeyCode::Char('y') => {
                            execute_action(app, action)?;
                            app.confirm_action = None;
                        }
                        KeyCode::Char('n') | KeyCode::Esc => {
                            app.confirm_action = None;
                            app.status = "action cancelled".to_string();
                        }
                        _ => {}
                    }
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('j') | KeyCode::Down => app.next(),
                    KeyCode::Char('k') | KeyCode::Up => app.previous(),
                    KeyCode::Char('r') => app.refresh(),
                    KeyCode::Char(':') => {
                        app.command_mode = true;
                        app.command_query.clear();
                        app.status = "command palette".to_string();
                    }
                    KeyCode::Char('a') => {
                        app.add_mode = true;
                        app.add_input.clear();
                        app.status = "enter repo to add".to_string();
                    }
                    KeyCode::Char('/') => {
                        app.filter_mode = true;
                        app.filter_query.clear();
                        app.status = "type to filter skills".to_string();
                    }
                    KeyCode::Char('?') => app.show_help = true,
                    KeyCode::Char('i') => app.confirm_action = Some(PendingAction::Install),
                    KeyCode::Char('u') => app.confirm_action = Some(PendingAction::Update),
                    KeyCode::Char('t') => {
                        app.sync_target = if app.sync_target == "opencode" {
                            "claude".to_string()
                        } else {
                            "opencode".to_string()
                        };
                        app.status = format!("sync target set to {}", app.sync_target);
                    }
                    KeyCode::Char('s') => app.confirm_action = Some(PendingAction::Sync),
                    KeyCode::Char('x') => app.confirm_action = Some(PendingAction::Remove),
                    KeyCode::Char('d') => app.confirm_action = Some(PendingAction::Doctor),
                    _ => {}
                }
            }
        }
    }
}

fn run_command(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let command = app.command_query.trim().to_lowercase();
    app.command_mode = false;

    match command.as_str() {
        "" => app.status = "empty command".to_string(),
        "add" => {
            app.add_mode = true;
            app.add_input.clear();
            app.status = "enter repo to add".to_string();
        }
        "install" => {
            app.confirm_action = Some(PendingAction::Install);
            app.status = "confirm install".to_string();
        }
        "update" => {
            app.confirm_action = Some(PendingAction::Update);
            app.status = "confirm update".to_string();
        }
        "sync" => {
            app.confirm_action = Some(PendingAction::Sync);
            app.status = format!("confirm sync to {}", app.sync_target);
        }
        "remove" => {
            app.confirm_action = Some(PendingAction::Remove);
            app.status = "confirm remove".to_string();
        }
        "doctor" => {
            app.confirm_action = Some(PendingAction::Doctor);
            app.status = "confirm doctor".to_string();
        }
        "refresh" => app.refresh(),
        "toggle-target" => {
            app.sync_target = if app.sync_target == "opencode" {
                "claude".to_string()
            } else {
                "opencode".to_string()
            };
            app.status = format!("sync target set to {}", app.sync_target);
        }
        "help" => app.show_help = true,
        other => {
            app.pending_action_result = Some(format!("Unknown command: {}", other));
            app.status = "unknown command".to_string();
        }
    }

    app.command_query.clear();
    Ok(())
}

fn execute_action(app: &mut App, action: PendingAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        PendingAction::Install => {
            if let Some(name) = app.selected_filtered_name() {
                let runtime = tokio::runtime::Runtime::new()?;
                runtime.block_on(install_selected(Some(name.clone()), false, false))?;
                app.status = format!("installed {}", name);
                app.pending_action_result = Some(format!("Installed {} and refreshed cached state.", name));
                app.refresh();
            }
        }
        PendingAction::Update => {
            if let Some(name) = app.selected_filtered_name() {
                let runtime = tokio::runtime::Runtime::new()?;
                runtime.block_on(update(Some(name.clone())))?;
                app.status = format!("updated {}", name);
                app.pending_action_result = Some(format!("Updated {} and refreshed state.", name));
                app.refresh();
            }
        }
        PendingAction::Sync => {
            let runtime = tokio::runtime::Runtime::new()?;
            runtime.block_on(sync(app.sync_target.clone(), None))?;
            app.status = format!("synced skills to {} target", app.sync_target);
            app.pending_action_result = Some(format!("Synced configured skills to the {} target.", app.sync_target));
            app.refresh();
        }
        PendingAction::Remove => {
            if let Some(name) = app.selected_filtered_name() {
                let runtime = tokio::runtime::Runtime::new()?;
                runtime.block_on(remove(name.clone()))?;
                app.status = format!("removed {}", name);
                app.pending_action_result = Some(format!("Removed {} from configuration.", name));
                app.refresh();
            }
        }
        PendingAction::Doctor => {
            app.doctor_output = Some(tokio::runtime::Runtime::new()?.block_on(doctor_summary())?);
            app.status = "doctor summary ready".to_string();
        }
    }

    Ok(())
}

fn action_label(action: PendingAction) -> &'static str {
    match action {
        PendingAction::Install => "install",
        PendingAction::Update => "update",
        PendingAction::Sync => "sync",
        PendingAction::Remove => "remove",
        PendingAction::Doctor => "doctor",
    }
}

fn render_modal(frame: &mut ratatui::Frame<'_>, area: Rect, title: &str, body: String) {
    frame.render_widget(Clear, area);
    let modal = Paragraph::new(body)
        .block(Block::default().title(title).borders(Borders::ALL))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    frame.render_widget(modal, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, rect: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(rect);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

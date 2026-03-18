use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Terminal;

use crate::cli::{api, SkillSummary};

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
    detail_scroll: u16,
    modal_scroll: u16,
    add_mode: bool,
    add_input: String,
    create_mode: bool,
    create_input: String,
    sync_target: String,
    command_mode: bool,
    command_query: String,
}

#[derive(Clone, Copy)]
enum PendingAction {
    Enable,
    Disable,
    Unsync,
    Resync,
    Freeze,
    Thaw,
    Info,
    Outdated,
    Clean,
    Install,
    Update,
    Sync,
    Remove,
    Doctor,
}

pub(crate) trait ActionExecutor {
    fn add_skill(&self, repo: String) -> Result<String, Box<dyn std::error::Error>>;
    fn install_skill(&self, name: Option<String>) -> Result<(), Box<dyn std::error::Error>>;
    fn update_skill(&self, name: Option<String>) -> Result<(), Box<dyn std::error::Error>>;
    fn sync_skills(&self, target: String) -> Result<String, Box<dyn std::error::Error>>;
    fn remove_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>>;
    fn doctor_summary_text(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn enable_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>>;
    fn disable_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>>;
    fn unsync_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>>;
    fn resync_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>>;
    fn freeze_skills(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn thaw_skills(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn clean_generated(&self, all: bool) -> Result<(), Box<dyn std::error::Error>>;
    fn info_skill(&self, name: String) -> Result<String, Box<dyn std::error::Error>>;
    fn outdated_skills(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn create_skill(
        &self,
        name: String,
        output_dir: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>>;
}

impl ActionExecutor for api::TuiActionExecutor {
    fn add_skill(&self, repo: String) -> Result<String, Box<dyn std::error::Error>> {
        api::TuiActionExecutor::add_skill(self, repo)
    }

    fn install_skill(&self, name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        api::TuiActionExecutor::install_skill(self, name)
    }

    fn update_skill(&self, name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        api::TuiActionExecutor::update_skill(self, name)
    }

    fn sync_skills(&self, target: String) -> Result<String, Box<dyn std::error::Error>> {
        api::TuiActionExecutor::sync_skills(self, target)
    }

    fn remove_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        api::TuiActionExecutor::remove_skill(self, name)
    }

    fn doctor_summary_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        api::TuiActionExecutor::doctor_summary_text(self)
    }

    fn enable_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        api::TuiActionExecutor::enable_skill(self, name)
    }

    fn disable_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        api::TuiActionExecutor::disable_skill(self, name)
    }

    fn unsync_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        api::TuiActionExecutor::unsync_skill(self, name)
    }

    fn resync_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        api::TuiActionExecutor::resync_skill(self, name)
    }

    fn freeze_skills(&self) -> Result<(), Box<dyn std::error::Error>> {
        api::TuiActionExecutor::freeze_skills(self)
    }

    fn thaw_skills(&self) -> Result<(), Box<dyn std::error::Error>> {
        api::TuiActionExecutor::thaw_skills(self)
    }

    fn clean_generated(&self, all: bool) -> Result<(), Box<dyn std::error::Error>> {
        api::TuiActionExecutor::clean_generated(self, all)
    }

    fn info_skill(&self, name: String) -> Result<String, Box<dyn std::error::Error>> {
        api::TuiActionExecutor::info_skill(self, name)
    }

    fn outdated_skills(&self) -> Result<String, Box<dyn std::error::Error>> {
        api::TuiActionExecutor::outdated_skills(self)
    }

    fn create_skill(
        &self,
        name: String,
        output_dir: Option<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        api::TuiActionExecutor::create_skill(self, name, output_dir)
    }
}

impl App {
    fn new(skills: Vec<SkillSummary>) -> Self {
        Self {
            skills,
            selected: 0,
            status: "q quit • j/k move • / filter • : commands • create -> add -> install -> sync • i install • u update • s sync • x remove • d doctor • ? help".to_string(),
            show_help: false,
            confirm_action: None,
            doctor_output: None,
            pending_action_result: None,
            filter_mode: false,
            filter_query: String::new(),
            detail_scroll: 0,
            modal_scroll: 0,
            add_mode: false,
            add_input: String::new(),
            create_mode: false,
            create_input: String::new(),
            sync_target: "opencode".to_string(),
            command_mode: false,
            command_query: String::new(),
        }
    }

    fn next(&mut self) {
        if !self.skills.is_empty() {
            self.selected = (self.selected + 1) % self.skills.len();
            self.detail_scroll = 0;
        }
    }

    fn previous(&mut self) {
        if !self.skills.is_empty() {
            self.selected = if self.selected == 0 {
                self.skills.len() - 1
            } else {
                self.selected - 1
            };
            self.detail_scroll = 0;
        }
    }

    fn refresh(&mut self) {
        match api::load_skill_summaries() {
            Ok(skills) => {
                self.skills = skills;
                self.normalize_selection();
                self.detail_scroll = 0;
                self.status = "refreshed summaries".to_string();
            }
            Err(error) => {
                self.status = format!("refresh failed: {}", error);
            }
        }
    }

    fn filtered_indices(&self) -> Vec<usize> {
        let filter = parse_filter_query(&self.filter_query);

        if filter.is_empty() {
            return (0..self.skills.len()).collect();
        }

        self.skills
            .iter()
            .enumerate()
            .filter(|(_, skill)| skill_matches_filter(skill, &filter))
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
        self.selected_filtered_skill()
            .map(|skill| skill.name.clone())
    }

    fn command_items(&self) -> Vec<&'static str> {
        let commands = vec![
            "create",
            "add",
            "enable",
            "disable",
            "unsync",
            "resync",
            "install",
            "update",
            "sync",
            "remove",
            "freeze",
            "thaw",
            "info",
            "outdated",
            "clean",
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

#[derive(Debug, Default, PartialEq, Eq)]
struct ParsedFilter {
    text_terms: Vec<String>,
    source: Option<String>,
    status: Option<String>,
}

impl ParsedFilter {
    fn is_empty(&self) -> bool {
        self.text_terms.is_empty() && self.source.is_none() && self.status.is_none()
    }
}

fn parse_filter_query(query: &str) -> ParsedFilter {
    let mut parsed = ParsedFilter::default();

    for token in query.split_whitespace() {
        if let Some(source) = token.strip_prefix("source:") {
            if !source.is_empty() {
                parsed.source = Some(source.to_lowercase());
            }
            continue;
        }

        if let Some(status) = token.strip_prefix("status:") {
            if !status.is_empty() {
                parsed.status = Some(status.to_lowercase());
            }
            continue;
        }

        parsed.text_terms.push(token.to_lowercase());
    }

    parsed
}

fn skill_matches_filter(skill: &SkillSummary, filter: &ParsedFilter) -> bool {
    if let Some(source) = &filter.source {
        if !skill.source.to_lowercase().contains(source) {
            return false;
        }
    }

    if let Some(status) = &filter.status {
        if !skill
            .statuses
            .iter()
            .any(|entry| entry.to_lowercase() == *status)
        {
            return false;
        }
    }

    filter.text_terms.iter().all(|needle| {
        skill.name.to_lowercase().contains(needle)
            || skill.source.to_lowercase().contains(needle)
            || skill.outdated.to_lowercase().contains(needle)
            || skill
                .statuses
                .iter()
                .any(|status: &String| status.to_lowercase().contains(needle))
            || skill
                .description
                .as_ref()
                .map(|desc: &String| desc.to_lowercase().contains(needle))
                .unwrap_or(false)
    })
}

pub(crate) fn run(action_executor: &impl ActionExecutor) -> Result<(), Box<dyn std::error::Error>> {
    let skills = api::load_skill_summaries()?;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new(skills);

    let result = run_loop(&mut terminal, &mut app, action_executor);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    action_executor: &impl ActionExecutor,
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
                    let max_width = body[0].width.saturating_sub(4) as usize;
                    ListItem::new(Line::from(format_skill_row(skill, max_width)))
                })
                .collect();

            let mut state = ListState::default();
            if !filtered_indices.is_empty() {
                state.select(Some(app.selected));
            }

            let list = List::new(items)
                .block(Block::default().title("Skills").borders(Borders::ALL))
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD))
                .highlight_symbol("▶ ");
            frame.render_stateful_widget(list, body[0], &mut state);

            let detail_text = if let Some(skill) = app.selected_filtered_skill() {
                vec![
                    Line::from(vec![Span::styled("Source: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(skill.source.clone())]),
                    Line::from(vec![Span::styled("Enabled: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(skill.enabled.to_string())]),
                    Line::from(vec![Span::styled("Statuses: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(skill.statuses.join(", "))]),
                    Line::from(vec![Span::styled("Outdated: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(skill.outdated.clone())]),
                    Line::from(vec![Span::styled("Lock: ", Style::default().add_modifier(Modifier::BOLD)), Span::raw(skill.lock_summary.clone())]),
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
                    Line::from(Span::styled("Description", Style::default().add_modifier(Modifier::BOLD))),
                    Line::from(skill.description.clone().unwrap_or_else(|| "No manifest description available".to_string())),
                ]
            } else {
                vec![Line::from("No skills configured")]
            };

            let detail = Paragraph::new(detail_text)
                .block(Block::default().title("Details").borders(Borders::ALL))
                .wrap(Wrap { trim: true })
                .scroll((app.detail_scroll, 0));
            frame.render_widget(detail, body[1]);

            let footer_text = if app.add_mode {
                format!("add mode • sync target: {}", app.sync_target)
            } else if app.filter_mode {
                format!("filter: {} • sync target: {}", app.filter_query, app.sync_target)
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
                    confirmation_message(action, &app.sync_target),
                    app.modal_scroll,
                );
            }

            if app.show_help {
                render_modal(
                    frame,
                    centered_rect(70, 40, frame.area()),
                    "Help",
                    "j/k or ↑/↓: move\n: command palette\ncreate: shows local package flow guidance (create -> add -> install -> sync)\na: add source to config after create\ne: enable selected skill\nD: disable selected skill\nn: unsync selected skill from runtime targets\nR: resync selected skill to runtime targets\n/: filter list (supports source:<github|local|version> status:<configured|disabled|unsynced|installed|cached|locked>)\ni: install selected skill locally\nu: update selected skill source\nt: cycle runtime target (opencode/claude)\ns: sync configured skills to current target\ncustom sync paths stay in CLI: skillmine sync --path <dir>\nx: remove selected skill from config\nd: run doctor summary\nr: refresh\n?: toggle help\nq: quit".to_string(),
                    app.modal_scroll,
                );
            }

            if let Some(output) = &app.doctor_output {
                render_modal(
                    frame,
                    centered_rect(80, 60, frame.area()),
                    "Doctor Summary",
                    output.clone(),
                    app.modal_scroll,
                );
            }

            if let Some(output) = &app.pending_action_result {
                render_modal(
                    frame,
                    centered_rect(70, 30, frame.area()),
                    "Action Result",
                    output.clone(),
                    app.modal_scroll,
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
                    app.modal_scroll,
                );
            }

            if app.add_mode {
                render_modal(
                    frame,
                    centered_rect(70, 20, frame.area()),
                    "Add Skill",
                    format!("Enter skill source (GitHub owner/repo[/path] or local path):\n{}", app.add_input),
                    app.modal_scroll,
                );
            }

            if app.create_mode {
                render_modal(
                    frame,
                    centered_rect(70, 20, frame.area()),
                    "Create Skill",
                    format!("Enter new skill name:\n{}", app.create_input),
                    app.modal_scroll,
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
                            app.modal_scroll = 0;
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
                            app.modal_scroll = 0;
                            app.status = "add cancelled".to_string();
                        }
                        KeyCode::Enter => {
                            let repo = app.add_input.trim().to_string();
                            if repo.is_empty() {
                                app.status = "skill source cannot be empty".to_string();
                            } else {
                                let report = action_executor.add_skill(repo.clone())?;
                                app.add_mode = false;
                                app.add_input.clear();
                                app.modal_scroll = 0;
                                app.status = format!("added source {}", repo);
                                app.pending_action_result = Some(report);
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

                if app.create_mode {
                    match key.code {
                        KeyCode::Esc => {
                            app.create_mode = false;
                            app.create_input.clear();
                            app.modal_scroll = 0;
                            app.status = "create cancelled".to_string();
                        }
                        KeyCode::Enter => {
                            let name = app.create_input.trim().to_string();
                            if name.is_empty() {
                                app.status = "skill name cannot be empty".to_string();
                            } else {
                                let report = action_executor.create_skill(name.clone(), None)?;
                                app.create_mode = false;
                                app.create_input.clear();
                                app.modal_scroll = 0;
                                app.status = format!("created {}", name);
                                app.pending_action_result = Some(report);
                            }
                        }
                        KeyCode::Backspace => {
                            app.create_input.pop();
                        }
                        KeyCode::Char(c) => {
                            app.create_input.push(c);
                        }
                        _ => {}
                    }
                    continue;
                }

                if app.pending_action_result.is_some() {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                            app.modal_scroll = 0;
                            app.pending_action_result = None
                        }
                        KeyCode::Up => app.modal_scroll = app.modal_scroll.saturating_sub(1),
                        KeyCode::Down => app.modal_scroll = app.modal_scroll.saturating_add(1),
                        KeyCode::PageUp => app.modal_scroll = app.modal_scroll.saturating_sub(10),
                        KeyCode::PageDown => app.modal_scroll = app.modal_scroll.saturating_add(10),
                        _ => {}
                    }
                    continue;
                }

                if app.filter_mode {
                    match key.code {
                        KeyCode::Esc => {
                            app.filter_mode = false;
                            app.modal_scroll = 0;
                            app.status = "filter cancelled".to_string();
                        }
                        KeyCode::Enter => {
                            app.filter_mode = false;
                            app.normalize_selection();
                            app.detail_scroll = 0;
                            app.status = if app.filter_query.is_empty() {
                                "filter cleared".to_string()
                            } else {
                                format!("filtered by '{}'", app.filter_query)
                            };
                        }
                        KeyCode::Backspace => {
                            app.filter_query.pop();
                            app.normalize_selection();
                            app.detail_scroll = 0;
                        }
                        KeyCode::Char(c) => {
                            app.filter_query.push(c);
                            app.normalize_selection();
                            app.detail_scroll = 0;
                        }
                        _ => {}
                    }
                    continue;
                }

                if app.doctor_output.is_some() {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                            app.modal_scroll = 0;
                            app.doctor_output = None
                        }
                        KeyCode::Up => app.modal_scroll = app.modal_scroll.saturating_sub(1),
                        KeyCode::Down => app.modal_scroll = app.modal_scroll.saturating_add(1),
                        KeyCode::PageUp => app.modal_scroll = app.modal_scroll.saturating_sub(10),
                        KeyCode::PageDown => app.modal_scroll = app.modal_scroll.saturating_add(10),
                        _ => {}
                    }
                    continue;
                }

                if app.show_help {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('?') => {
                            app.modal_scroll = 0;
                            app.show_help = false
                        }
                        KeyCode::Up => app.modal_scroll = app.modal_scroll.saturating_sub(1),
                        KeyCode::Down => app.modal_scroll = app.modal_scroll.saturating_add(1),
                        KeyCode::PageUp => app.modal_scroll = app.modal_scroll.saturating_sub(10),
                        KeyCode::PageDown => app.modal_scroll = app.modal_scroll.saturating_add(10),
                        _ => {}
                    }
                    continue;
                }

                if let Some(action) = app.confirm_action {
                    match key.code {
                        KeyCode::Char('y') => {
                            execute_action(app, action, action_executor)?;
                            app.modal_scroll = 0;
                            app.confirm_action = None;
                        }
                        KeyCode::Char('n') | KeyCode::Esc => {
                            app.confirm_action = None;
                            app.modal_scroll = 0;
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
                        app.modal_scroll = 0;
                        app.status = "command palette".to_string();
                    }
                    KeyCode::Char('a') => {
                        app.add_mode = true;
                        app.add_input.clear();
                        app.modal_scroll = 0;
                        app.status = "enter skill source to add".to_string();
                    }
                    KeyCode::Char('e') => {
                        app.modal_scroll = 0;
                        app.confirm_action = Some(PendingAction::Enable)
                    }
                    KeyCode::Char('D') => {
                        app.modal_scroll = 0;
                        app.confirm_action = Some(PendingAction::Disable)
                    }
                    KeyCode::Char('n') => {
                        app.modal_scroll = 0;
                        app.confirm_action = Some(PendingAction::Unsync)
                    }
                    KeyCode::Char('R') => {
                        app.modal_scroll = 0;
                        app.confirm_action = Some(PendingAction::Resync)
                    }
                    KeyCode::Char('/') => {
                        enter_filter_mode(app);
                    }
                    KeyCode::Char('?') => {
                        app.modal_scroll = 0;
                        app.show_help = true
                    }
                    KeyCode::Char('i') => {
                        app.modal_scroll = 0;
                        app.confirm_action = Some(PendingAction::Install)
                    }
                    KeyCode::Char('u') => {
                        app.modal_scroll = 0;
                        app.confirm_action = Some(PendingAction::Update)
                    }
                    KeyCode::Char('t') => {
                        app.sync_target = next_sync_target(&app.sync_target);
                        app.status = format!("sync target set to {}", app.sync_target);
                    }
                    KeyCode::Char('s') => {
                        app.modal_scroll = 0;
                        app.confirm_action = Some(PendingAction::Sync)
                    }
                    KeyCode::Char('x') => {
                        app.modal_scroll = 0;
                        app.confirm_action = Some(PendingAction::Remove)
                    }
                    KeyCode::Char('d') => {
                        app.modal_scroll = 0;
                        app.confirm_action = Some(PendingAction::Doctor)
                    }
                    KeyCode::PageUp => app.detail_scroll = app.detail_scroll.saturating_sub(10),
                    KeyCode::PageDown => app.detail_scroll = app.detail_scroll.saturating_add(10),
                    _ => {}
                }
            }
        }
    }
}

fn run_command(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let command = app.command_query.trim().to_lowercase();
    app.command_mode = false;
    app.modal_scroll = 0;

    match command.as_str() {
        "" => app.status = "empty command".to_string(),
        "add" => {
            app.add_mode = true;
            app.add_input.clear();
            app.status = "enter skill source to add".to_string();
        }
        "enable" => {
            app.confirm_action = Some(PendingAction::Enable);
            app.status = "confirm enable".to_string();
        }
        "disable" => {
            app.confirm_action = Some(PendingAction::Disable);
            app.status = "confirm disable".to_string();
        }
        "unsync" => {
            app.confirm_action = Some(PendingAction::Unsync);
            app.status = "confirm unsync".to_string();
        }
        "resync" => {
            app.confirm_action = Some(PendingAction::Resync);
            app.status = "confirm resync".to_string();
        }
        "create" => {
            app.create_mode = true;
            app.create_input.clear();
            app.status = "enter skill name to create".to_string();
        }
        "freeze" => {
            app.confirm_action = Some(PendingAction::Freeze);
            app.status = "confirm freeze".to_string();
        }
        "thaw" => {
            app.confirm_action = Some(PendingAction::Thaw);
            app.status = "confirm thaw".to_string();
        }
        "info" => {
            app.confirm_action = Some(PendingAction::Info);
            app.status = "confirm info".to_string();
        }
        "outdated" => {
            app.confirm_action = Some(PendingAction::Outdated);
            app.status = "confirm outdated".to_string();
        }
        "clean" => {
            app.confirm_action = Some(PendingAction::Clean);
            app.status = "confirm clean".to_string();
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
            app.status = format!("confirm sync to {} runtime target", app.sync_target);
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
            app.sync_target = next_sync_target(&app.sync_target);
            app.status = format!("sync target set to {}", app.sync_target);
        }
        "help" => app.show_help = true,
        "filter" => enter_filter_mode(app),
        other => {
            app.pending_action_result = Some(format!("Unknown command: {}", other));
            app.status = "unknown command".to_string();
        }
    }

    app.command_query.clear();
    Ok(())
}

fn execute_action(
    app: &mut App,
    action: PendingAction,
    action_executor: &impl ActionExecutor,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        PendingAction::Enable => {
            if let Some(name) = app.selected_filtered_name() {
                action_executor.enable_skill(name.clone())?;
                app.status = format!("enabled {}", name);
                app.pending_action_result =
                    Some(format!("Enabled {} for managed lifecycle.", name));
                app.refresh();
            }
        }
        PendingAction::Disable => {
            if let Some(name) = app.selected_filtered_name() {
                action_executor.disable_skill(name.clone())?;
                app.status = format!("disabled {}", name);
                app.pending_action_result = Some(format!("Disabled {} in configuration.", name));
                app.refresh();
            }
        }
        PendingAction::Unsync => {
            if let Some(name) = app.selected_filtered_name() {
                action_executor.unsync_skill(name.clone())?;
                app.status = format!("unsynced {}", name);
                app.pending_action_result =
                    Some(format!("Unsynced {} from runtime targets.", name));
                app.refresh();
            }
        }
        PendingAction::Resync => {
            if let Some(name) = app.selected_filtered_name() {
                action_executor.resync_skill(name.clone())?;
                app.status = format!("resynced {}", name);
                app.pending_action_result = Some(format!("Resynced {} to runtime targets.", name));
                app.refresh();
            }
        }
        PendingAction::Install => {
            if let Some(name) = app.selected_filtered_name() {
                action_executor.install_skill(Some(name.clone()))?;
                app.status = format!("installed {} locally", name);
                app.pending_action_result =
                    Some(format!("Installed {} into local managed state.", name));
                app.refresh();
            }
        }
        PendingAction::Update => {
            if let Some(name) = app.selected_filtered_name() {
                action_executor.update_skill(Some(name.clone()))?;
                app.status = format!("updated {}", name);
                app.pending_action_result = Some(format!("Updated {} and refreshed state.", name));
                app.refresh();
            }
        }
        PendingAction::Sync => {
            let report = action_executor.sync_skills(app.sync_target.clone())?;
            app.status = format!(
                "synced configured skills to {} runtime target",
                app.sync_target
            );
            app.pending_action_result = Some(report);
            app.refresh();
        }
        PendingAction::Remove => {
            if let Some(name) = app.selected_filtered_name() {
                action_executor.remove_skill(name.clone())?;
                app.status = format!("removed {}", name);
                app.pending_action_result = Some(format!("Removed {} from configuration.", name));
                app.refresh();
            }
        }
        PendingAction::Doctor => {
            app.doctor_output = Some(action_executor.doctor_summary_text()?);
            app.status = "doctor summary ready".to_string();
        }
        PendingAction::Freeze => {
            action_executor.freeze_skills()?;
            app.status = "froze managed state".to_string();
            app.pending_action_result =
                Some("Wrote lockfile from current managed state.".to_string());
            app.refresh();
        }
        PendingAction::Thaw => {
            action_executor.thaw_skills()?;
            app.status = "thawed lockfile state".to_string();
            app.pending_action_result =
                Some("Applied lockfile state back into configuration.".to_string());
            app.refresh();
        }
        PendingAction::Info => {
            if let Some(name) = app.selected_filtered_name() {
                let report = action_executor.info_skill(name.clone())?;
                app.status = format!("info ready for {}", name);
                app.pending_action_result = Some(report);
            }
        }
        PendingAction::Outdated => {
            let report = action_executor.outdated_skills()?;
            app.status = "outdated report ready".to_string();
            app.pending_action_result = Some(report);
        }
        PendingAction::Clean => {
            action_executor.clean_generated(false)?;
            app.status = "cleaned generated state".to_string();
            app.pending_action_result = Some("Removed generated temporary state.".to_string());
        }
    }

    Ok(())
}

fn action_label(action: PendingAction) -> &'static str {
    match action {
        PendingAction::Enable => "enable",
        PendingAction::Disable => "disable",
        PendingAction::Unsync => "unsync",
        PendingAction::Resync => "resync",
        PendingAction::Install => "install",
        PendingAction::Update => "update",
        PendingAction::Sync => "sync",
        PendingAction::Remove => "remove",
        PendingAction::Doctor => "doctor",
        PendingAction::Freeze => "freeze",
        PendingAction::Thaw => "thaw",
        PendingAction::Info => "info",
        PendingAction::Outdated => "outdated",
        PendingAction::Clean => "clean",
    }
}

fn next_sync_target(current: &str) -> String {
    match current {
        "opencode" => "claude".to_string(),
        _ => "opencode".to_string(),
    }
}

fn confirmation_message(action: PendingAction, sync_target: &str) -> String {
    match action {
        PendingAction::Enable => "Run enable? Press y to confirm, n to cancel.".to_string(),
        PendingAction::Disable => "Run disable? Press y to confirm, n to cancel.".to_string(),
        PendingAction::Unsync => "Run unsync? Press y to confirm, n to cancel.".to_string(),
        PendingAction::Resync => "Run resync? Press y to confirm, n to cancel.".to_string(),
        PendingAction::Sync => format!(
            "Run sync to {} target? Press y to confirm, n to cancel.",
            sync_target
        ),
        PendingAction::Freeze => "Run freeze? Press y to confirm, n to cancel.".to_string(),
        PendingAction::Thaw => "Run thaw? Press y to confirm, n to cancel.".to_string(),
        PendingAction::Info => "Run info? Press y to confirm, n to cancel.".to_string(),
        PendingAction::Outdated => "Run outdated? Press y to confirm, n to cancel.".to_string(),
        PendingAction::Clean => "Run clean? Press y to confirm, n to cancel.".to_string(),
        _ => format!(
            "Run {}? Press y to confirm, n to cancel.",
            action_label(action)
        ),
    }
}

fn enter_filter_mode(app: &mut App) {
    app.filter_mode = true;
    app.detail_scroll = 0;
    app.status = if app.filter_query.is_empty() {
        "type to filter skills".to_string()
    } else {
        "edit filter query".to_string()
    };
}

fn truncate_with_ellipsis(input: &str, max_width: usize) -> String {
    if max_width == 0 {
        return String::new();
    }
    let chars: Vec<char> = input.chars().collect();
    if chars.len() <= max_width {
        return input.to_string();
    }
    if max_width == 1 {
        return "…".to_string();
    }
    chars.into_iter().take(max_width - 1).collect::<String>() + "…"
}

fn format_skill_row(skill: &SkillSummary, max_width: usize) -> String {
    let disabled = if skill.enabled { "" } else { " [disabled]" };
    let raw = format!("{}{} [{}]", skill.name, disabled, skill.outdated);
    truncate_with_ellipsis(&raw, max_width)
}

fn render_modal(
    frame: &mut ratatui::Frame<'_>,
    area: Rect,
    title: &str,
    body: String,
    scroll: u16,
) {
    frame.render_widget(Clear, area);
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);
    let modal = Paragraph::new(body)
        .block(Block::default().title(title).borders(Borders::ALL))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .scroll((scroll, 0));
    frame.render_widget(modal, inner[0]);
    frame.render_widget(
        Paragraph::new("↑/↓ scroll • PgUp/PgDn scroll • Esc close").alignment(Alignment::Center),
        inner[1],
    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    struct FakeExecutor {
        calls: RefCell<Vec<String>>,
        doctor_output: String,
    }

    impl FakeExecutor {
        fn with_doctor_output(output: &str) -> Self {
            Self {
                calls: RefCell::new(Vec::new()),
                doctor_output: output.to_string(),
            }
        }

        fn calls(&self) -> Vec<String> {
            self.calls.borrow().clone()
        }
    }

    impl ActionExecutor for FakeExecutor {
        fn add_skill(&self, repo: String) -> Result<String, Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push(format!("add:{repo}"));
            Ok(format!(
                "Added source '{}' to configuration. Next: install to prepare it locally.",
                repo
            ))
        }

        fn install_skill(&self, name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push(format!(
                "install:{}",
                name.unwrap_or_else(|| "all".to_string())
            ));
            Ok(())
        }

        fn update_skill(&self, name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push(format!(
                "update:{}",
                name.unwrap_or_else(|| "all".to_string())
            ));
            Ok(())
        }

        fn sync_skills(&self, target: String) -> Result<String, Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push(format!("sync:{target}"));
            Ok(format!("Sync complete:\n  1 synced to /tmp/{target}"))
        }

        fn remove_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push(format!("remove:{name}"));
            Ok(())
        }

        fn doctor_summary_text(&self) -> Result<String, Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push("doctor".to_string());
            Ok(self.doctor_output.clone())
        }

        fn enable_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push(format!("enable:{name}"));
            Ok(())
        }

        fn disable_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push(format!("disable:{name}"));
            Ok(())
        }

        fn unsync_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push(format!("unsync:{name}"));
            Ok(())
        }

        fn resync_skill(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push(format!("resync:{name}"));
            Ok(())
        }

        fn freeze_skills(&self) -> Result<(), Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push("freeze".to_string());
            Ok(())
        }

        fn thaw_skills(&self) -> Result<(), Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push("thaw".to_string());
            Ok(())
        }

        fn clean_generated(&self, all: bool) -> Result<(), Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push(format!("clean:{all}"));
            Ok(())
        }

        fn info_skill(&self, name: String) -> Result<String, Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push(format!("info:{name}"));
            Ok(format!(
                "Skill: {name}\nSource: local:/tmp/{name}\nEnabled: true\nOutdated: up-to-date"
            ))
        }

        fn outdated_skills(&self) -> Result<String, Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push("outdated".to_string());
            Ok("demo: up-to-date".to_string())
        }

        fn create_skill(
            &self,
            name: String,
            output_dir: Option<String>,
        ) -> Result<String, Box<dyn std::error::Error>> {
            self.calls.borrow_mut().push(format!(
                "create:{}:{}",
                name,
                output_dir.unwrap_or_else(|| "cwd".to_string())
            ));
            Ok(format!(
                "Created skill package at /tmp/{}\nNext:\n  1. skillmine add /tmp/{}\n  2. skillmine install\n  3. skillmine sync --target=opencode",
                name, name
            ))
        }
    }

    fn sample_skill(name: &str) -> SkillSummary {
        sample_skill_with(name, "local", vec!["configured"])
    }

    fn sample_skill_with(name: &str, source: &str, statuses: Vec<&str>) -> SkillSummary {
        SkillSummary {
            name: name.to_string(),
            source: source.to_string(),
            enabled: true,
            statuses: statuses.into_iter().map(str::to_string).collect(),
            outdated: "up-to-date".to_string(),
            lock_summary: "not-locked".to_string(),
            manifest_version: None,
            skill_version: None,
            maturity: None,
            last_verified: None,
            description: Some("demo skill".to_string()),
        }
    }

    #[test]
    fn tui_doctor_action_does_not_nest_runtime() {
        let mut app = App::new(vec![]);
        let executor = FakeExecutor::with_doctor_output("PASS config validation");

        let result = execute_action(&mut app, PendingAction::Doctor, &executor);

        assert!(result.is_ok());
        assert_eq!(app.doctor_output.as_deref(), Some("PASS config validation"));
        assert_eq!(app.status, "doctor summary ready");
        assert_eq!(executor.calls(), vec!["doctor"]);
    }

    #[test]
    fn tui_action_install_runs_selected_skill() {
        let mut app = App::new(vec![sample_skill("demo")]);
        let executor = FakeExecutor::default();

        let result = execute_action(&mut app, PendingAction::Install, &executor);

        assert!(result.is_ok());
        assert_eq!(app.status, "refreshed summaries");
        assert_eq!(
            app.pending_action_result.as_deref(),
            Some("Installed demo into local managed state.")
        );
        assert_eq!(executor.calls(), vec!["install:demo"]);
    }

    #[test]
    fn add_mode_uses_executor_report_text() {
        let executor = FakeExecutor::default();
        let report = executor
            .add_skill("/tmp/opencode-skill-local-demo".to_string())
            .unwrap();

        assert_eq!(
            report,
            "Added source '/tmp/opencode-skill-local-demo' to configuration. Next: install to prepare it locally."
        );
    }

    #[test]
    fn entering_filter_mode_preserves_existing_query() {
        let mut app = App::new(vec![sample_skill("demo")]);
        app.filter_query = "source:local smoke".to_string();

        enter_filter_mode(&mut app);

        assert!(app.filter_mode);
        assert_eq!(app.filter_query, "source:local smoke");
        assert_eq!(app.status, "edit filter query");
    }

    #[test]
    fn skill_list_rows_are_truncated_with_ellipsis() {
        let skill = sample_skill_with(
            "opencode-skill-super-long-name-for-terminal-width",
            "local:/tmp/demo",
            vec!["configured"],
        );

        let row = format_skill_row(&skill, 24);

        assert!(row.ends_with('…'));
        assert!(row.chars().count() <= 24);
    }

    #[test]
    fn tui_action_update_sync_and_remove_use_executor_boundary() {
        let executor = FakeExecutor::default();

        let mut update_app = App::new(vec![sample_skill("demo")]);
        assert!(execute_action(&mut update_app, PendingAction::Update, &executor).is_ok());

        let mut sync_app = App::new(vec![sample_skill("demo")]);
        assert!(execute_action(&mut sync_app, PendingAction::Sync, &executor).is_ok());

        let mut remove_app = App::new(vec![sample_skill("demo")]);
        assert!(execute_action(&mut remove_app, PendingAction::Remove, &executor).is_ok());

        assert_eq!(
            executor.calls(),
            vec!["update:demo", "sync:opencode", "remove:demo"]
        );
    }

    #[test]
    fn tui_action_remove_without_selection_is_nonfatal() {
        let mut app = App::new(vec![]);
        let executor = FakeExecutor::default();

        let result = execute_action(&mut app, PendingAction::Remove, &executor);

        assert!(result.is_ok());
        assert!(executor.calls().is_empty());
        assert!(app.pending_action_result.is_none());
        assert!(app.doctor_output.is_none());
    }

    #[test]
    fn filter_query_supports_structured_source_tokens() {
        let mut app = App::new(vec![
            sample_skill_with("gh-demo", "github:owner/repo", vec!["configured", "locked"]),
            sample_skill_with("local-demo", "local:/tmp/demo", vec!["configured"]),
        ]);
        app.filter_query = "source:github".to_string();

        let filtered = app.filtered_indices();

        assert_eq!(filtered, vec![0]);
    }

    #[test]
    fn filter_query_supports_structured_status_tokens() {
        let mut app = App::new(vec![
            sample_skill_with(
                "disabled-demo",
                "local:/tmp/demo",
                vec!["configured", "disabled"],
            ),
            sample_skill_with(
                "ready-demo",
                "github:owner/repo",
                vec!["configured", "locked"],
            ),
        ]);
        app.filter_query = "status:disabled".to_string();

        let filtered = app.filtered_indices();

        assert_eq!(filtered, vec![0]);
    }

    #[test]
    fn filter_query_combines_structured_and_text_terms() {
        let mut app = App::new(vec![
            sample_skill_with(
                "alpha-gh",
                "github:owner/repo",
                vec!["configured", "locked"],
            ),
            sample_skill_with("beta-gh", "github:owner/repo", vec!["configured", "locked"]),
        ]);
        app.filter_query = "source:github alpha".to_string();

        let filtered = app.filtered_indices();

        assert_eq!(filtered, vec![0]);
    }

    #[test]
    fn sync_confirmation_message_includes_target() {
        assert_eq!(
            confirmation_message(PendingAction::Sync, "claude"),
            "Run sync to claude target? Press y to confirm, n to cancel."
        );
    }

    #[test]
    fn sync_target_cycles_through_supported_targets() {
        assert_eq!(next_sync_target("opencode"), "claude");
        assert_eq!(next_sync_target("claude"), "opencode");
    }

    #[test]
    fn selection_navigation_resets_detail_scroll() {
        let mut app = App::new(vec![sample_skill("one"), sample_skill("two")]);
        app.detail_scroll = 5;

        app.next();

        assert_eq!(app.detail_scroll, 0);
    }

    #[test]
    fn sync_action_uses_executor_report_text() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Sync, &executor);

        assert!(result.is_ok());
        assert_eq!(
            app.pending_action_result.as_deref(),
            Some("Sync complete:\n  1 synced to /tmp/opencode")
        );
    }

    #[test]
    fn command_palette_includes_create_and_help_mentions_create_flow() {
        let app = App::new(vec![sample_skill("demo")]);

        let commands = app.command_items();

        assert!(commands.contains(&"create"));
        assert!(app.status.contains("create"));
    }

    #[test]
    fn help_text_mentions_cli_only_custom_sync_paths() {
        let app = App::new(vec![sample_skill("demo")]);

        assert!(app.status.contains("create"));
        assert!(
            "j/k or ↑/↓: move\n: command palette\ncreate: shows local package flow guidance (create -> add -> install -> sync)\na: add source to config after create\ne: enable selected skill\nD: disable selected skill\nn: unsync selected skill from runtime targets\nR: resync selected skill to runtime targets\n/: filter list (supports source:<github|local|version> status:<configured|disabled|unsynced|installed|cached|locked>)\ni: install selected skill locally\nu: update selected skill source\nt: cycle runtime target (opencode/claude)\ns: sync configured skills to current target\ncustom sync paths stay in CLI: skillmine sync --path <dir>\nx: remove selected skill from config\nd: run doctor summary\nr: refresh\n?: toggle help\nq: quit"
                .contains("skillmine sync --path <dir>")
        );
    }

    #[test]
    fn run_command_create_opens_add_guidance_result() {
        let mut app = App::new(vec![sample_skill("demo")]);
        app.command_query = "create".to_string();

        run_command(&mut app).unwrap();

        assert_eq!(app.status, "enter skill name to create");
        assert!(app.create_mode);
    }

    #[test]
    fn command_palette_includes_cli_parity_commands() {
        let app = App::new(vec![sample_skill("demo")]);

        let commands = app.command_items();

        assert!(commands.contains(&"enable"));
        assert!(commands.contains(&"disable"));
        assert!(commands.contains(&"unsync"));
        assert!(commands.contains(&"resync"));
        assert!(commands.contains(&"freeze"));
        assert!(commands.contains(&"thaw"));
        assert!(commands.contains(&"info"));
        assert!(commands.contains(&"outdated"));
        assert!(commands.contains(&"clean"));
    }

    #[test]
    fn run_command_enable_sets_confirm_action_and_status() {
        let mut app = App::new(vec![sample_skill("demo")]);
        app.command_query = "enable".to_string();

        run_command(&mut app).unwrap();

        assert_eq!(app.confirm_action.map(action_label), Some("enable"));
        assert_eq!(app.status, "confirm enable");
    }

    #[test]
    fn run_command_freeze_sets_confirm_action_and_status() {
        let mut app = App::new(vec![sample_skill("demo")]);
        app.command_query = "freeze".to_string();

        run_command(&mut app).unwrap();

        assert_eq!(app.confirm_action.map(action_label), Some("freeze"));
        assert_eq!(app.status, "confirm freeze");
    }

    #[test]
    fn run_command_info_sets_confirm_action_and_status() {
        let mut app = App::new(vec![sample_skill("demo")]);
        app.command_query = "info".to_string();

        run_command(&mut app).unwrap();

        assert_eq!(app.confirm_action.map(action_label), Some("info"));
        assert_eq!(app.status, "confirm info");
    }

    #[test]
    fn tui_action_enable_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Enable, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["enable:demo"]);
        assert_eq!(
            app.pending_action_result.as_deref(),
            Some("Enabled demo for managed lifecycle.")
        );
    }

    #[test]
    fn tui_action_unsync_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Unsync, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["unsync:demo"]);
        assert_eq!(
            app.pending_action_result.as_deref(),
            Some("Unsynced demo from runtime targets.")
        );
    }

    #[test]
    fn run_command_disable_sets_confirm_action_and_status() {
        let mut app = App::new(vec![sample_skill("demo")]);
        app.command_query = "disable".to_string();

        run_command(&mut app).unwrap();

        assert_eq!(app.confirm_action.map(action_label), Some("disable"));
        assert_eq!(app.status, "confirm disable");
    }

    #[test]
    fn run_command_resync_sets_confirm_action_and_status() {
        let mut app = App::new(vec![sample_skill("demo")]);
        app.command_query = "resync".to_string();

        run_command(&mut app).unwrap();

        assert_eq!(app.confirm_action.map(action_label), Some("resync"));
        assert_eq!(app.status, "confirm resync");
    }

    #[test]
    fn tui_action_disable_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Disable, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["disable:demo"]);
        assert_eq!(
            app.pending_action_result.as_deref(),
            Some("Disabled demo in configuration.")
        );
    }

    #[test]
    fn tui_action_resync_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Resync, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["resync:demo"]);
        assert_eq!(
            app.pending_action_result.as_deref(),
            Some("Resynced demo to runtime targets.")
        );
    }

    #[test]
    fn tui_action_freeze_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Freeze, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["freeze"]);
        assert_eq!(
            app.pending_action_result.as_deref(),
            Some("Wrote lockfile from current managed state.")
        );
    }

    #[test]
    fn tui_action_thaw_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Thaw, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["thaw"]);
        assert_eq!(
            app.pending_action_result.as_deref(),
            Some("Applied lockfile state back into configuration.")
        );
    }

    #[test]
    fn tui_action_clean_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Clean, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["clean:false"]);
        assert_eq!(
            app.pending_action_result.as_deref(),
            Some("Removed generated temporary state.")
        );
    }

    #[test]
    fn tui_action_info_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Info, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["info:demo"]);
        assert_eq!(app.status, "info ready for demo");
        assert!(app
            .pending_action_result
            .as_deref()
            .unwrap()
            .contains("Skill: demo"));
    }

    #[test]
    fn tui_action_outdated_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Outdated, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["outdated"]);
        assert_eq!(app.status, "outdated report ready");
        assert_eq!(
            app.pending_action_result.as_deref(),
            Some("demo: up-to-date")
        );
    }

    #[test]
    fn create_mode_uses_executor_report_text() {
        let executor = FakeExecutor::default();
        let report = executor.create_skill("my-skill".to_string(), None).unwrap();

        assert!(report.contains("Created skill package at /tmp/my-skill"));
        assert!(report.contains("skillmine add /tmp/my-skill"));
        assert!(report.contains("skillmine install"));
        assert!(report.contains("skillmine sync --target=opencode"));
    }
}

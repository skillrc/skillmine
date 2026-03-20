use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Terminal;

use crate::cli::{api, SkillSummary};

// ── Theme ─────────────────────────────────────────────────
/// Centralized theme management for consistent styling across the TUI.
/// Uses semantic naming to avoid hardcoded colors scattered throughout the code.
#[derive(Debug, Clone, Copy)]
struct Theme {
    // Semantic colors
    healthy: Color,
    warning: Color,
    error: Color,
    info: Color,
    muted: Color,
    accent: Color,
    // UI element colors
    title: Color,
    border: Color,
    selected_bg: Color,
    selected_fg: Color,
    success_title: Color,
    error_title: Color,
    // Footer and modal colors
    footer_bg: Color,
    footer_fg: Color,
    footer_key: Color,
    modal_border: Color,
    modal_shadow: Color,
    section_border: Color,
    highlight: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            healthy: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            info: Color::Cyan,
            muted: Color::DarkGray,
            accent: Color::Magenta,
            title: Color::White,
            border: Color::DarkGray,
            selected_bg: Color::DarkGray,
            selected_fg: Color::White,
            success_title: Color::Green,
            error_title: Color::Red,
            footer_bg: Color::Rgb(30, 30, 30),
            footer_fg: Color::Gray,
            footer_key: Color::Cyan,
            modal_border: Color::Rgb(100, 100, 100),
            modal_shadow: Color::Black,
            section_border: Color::Rgb(60, 60, 60),
            highlight: Color::Rgb(255, 165, 0),
        }
    }
}

impl Theme {
    /// Style for healthy/success states
    fn healthy_style(&self) -> Style {
        Style::default().fg(self.healthy)
    }

    /// Style for error states
    fn error_style(&self) -> Style {
        Style::default().fg(self.error)
    }

    /// Style for informational elements
    fn info_style(&self) -> Style {
        Style::default().fg(self.info)
    }

    /// Style for accent elements
    fn accent_style(&self) -> Style {
        Style::default().fg(self.accent)
    }

    /// Style for muted/secondary text
    fn muted_style(&self) -> Style {
        Style::default().fg(self.muted)
    }

    /// Style for bold titles
    fn title_style(&self) -> Style {
        Style::default().fg(self.title).add_modifier(Modifier::BOLD)
    }

    /// Style for borders
    fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    /// Style for section headers in detail view
    fn section_style(&self) -> Style {
        Style::default().fg(self.info).add_modifier(Modifier::BOLD)
    }

    /// Style for labels
    fn label_style(&self) -> Style {
        Style::default().add_modifier(Modifier::BOLD)
    }

    /// Get color based on boolean (healthy = true, error = false)
    fn enabled_color(&self, enabled: bool) -> Color {
        if enabled {
            self.healthy
        } else {
            self.error
        }
    }

    /// Get color for outdated status
    fn outdated_color(&self, outdated: &str) -> Color {
        if outdated == "up-to-date" {
            self.healthy
        } else {
            self.warning
        }
    }

    /// Get modal title color based on modal type
    fn modal_title_color(&self, title: &str) -> Color {
        match title {
            "Action Result" => self.success_title,
            "Help" | "Doctor Summary" | "Confirm Action" | "Command Palette" | "Add Skill"
            | "Create Skill" => self.accent,
            _ => self.error_title,
        }
    }

    /// Style for footer background
    fn footer_style(&self) -> Style {
        Style::default().bg(self.footer_bg).fg(self.footer_fg)
    }

    /// Style for footer keybindings
    fn footer_key_style(&self) -> Style {
        Style::default()
            .bg(self.footer_bg)
            .fg(self.footer_key)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for section blocks in detail view
    fn section_block_style(&self) -> Style {
        Style::default().fg(self.section_border)
    }

    /// Style for highlighted/selected elements
    fn highlight_style(&self) -> Style {
        Style::default()
            .fg(self.highlight)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for modal borders
    fn modal_border_style(&self) -> Style {
        Style::default()
            .fg(self.modal_border)
            .add_modifier(Modifier::BOLD)
    }

    /// Enhanced selected style with foreground color
    fn enhanced_selected_style(&self) -> Style {
        Style::default()
            .bg(self.selected_bg)
            .fg(self.selected_fg)
            .add_modifier(Modifier::BOLD)
    }
}

// ── UI Components ──────────────────────────────────────────

/// Component for rendering the skill list with filtering and selection.
/// Encapsulates all list-related rendering logic to keep the main draw loop clean.
struct SkillList<'a> {
    skills: &'a [SkillSummary],
    filtered_indices: Vec<usize>,
    selected: usize,
    theme: Theme,
}

impl<'a> SkillList<'a> {
    /// Create a new SkillList component
    fn new(skills: &'a [SkillSummary], filtered_indices: Vec<usize>, selected: usize) -> Self {
        Self {
            skills,
            filtered_indices,
            selected,
            theme: Theme::default(),
        }
    }

    /// Render the skill list widget
    fn render(&self, frame: &mut ratatui::Frame<'_>, area: Rect) {
        let items: Vec<ListItem> = self
            .filtered_indices
            .iter()
            .enumerate()
            .filter_map(|(display_idx, skill_idx)| {
                self.skills.get(*skill_idx).map(|skill| {
                    let is_selected = display_idx == self.selected;
                    let max_width = area.width.saturating_sub(6) as usize;
                    let mut row = format_skill_row(skill, max_width, &self.theme);

                    if is_selected {
                        let prefix = Span::styled("▶ ", self.theme.highlight_style());
                        row.insert(0, prefix);
                    } else {
                        row.insert(0, Span::raw("  "));
                    }

                    ListItem::new(Line::from(row))
                })
            })
            .collect();

        let mut state = ListState::default();
        if !self.filtered_indices.is_empty() {
            state.select(Some(self.selected));
        }

        let list = List::new(items)
            .block(
                Block::default()
                    .title(Span::styled(
                        format!(
                            "Skills ({}/{}) ",
                            self.filtered_indices.len(),
                            self.skills.len()
                        ),
                        self.theme.title_style(),
                    ))
                    .borders(Borders::ALL)
                    .border_style(self.theme.border_style()),
            )
            .highlight_style(self.theme.enhanced_selected_style())
            .highlight_symbol("");

        frame.render_stateful_widget(list, area, &mut state);
    }
}

/// Component for rendering skill details in the detail panel with bordered sections
struct DetailPanel<'a> {
    skill: Option<&'a SkillSummary>,
    show_empty_guide: bool,
    theme: Theme,
}

impl<'a> DetailPanel<'a> {
    fn for_skill(skill: &'a SkillSummary, _scroll: u16) -> Self {
        Self {
            skill: Some(skill),
            show_empty_guide: false,
            theme: Theme::default(),
        }
    }

    fn empty_guide() -> Self {
        Self {
            skill: None,
            show_empty_guide: true,
            theme: Theme::default(),
        }
    }

    fn no_skills() -> Self {
        Self {
            skill: None,
            show_empty_guide: false,
            theme: Theme::default(),
        }
    }

    fn render(&self, frame: &mut ratatui::Frame<'_>, area: Rect) {
        if let Some(skill) = self.skill {
            self.render_skill_details_with_sections(frame, area, skill);
        } else if self.show_empty_guide {
            self.render_welcome_guide_with_sections(frame, area);
        } else {
            let detail = Paragraph::new("No skills configured").block(
                Block::default()
                    .title(Span::styled("Details", self.theme.title_style()))
                    .borders(Borders::ALL)
                    .border_style(self.theme.border_style()),
            );
            frame.render_widget(detail, area);
        }
    }

    fn render_skill_details_with_sections(
        &self,
        frame: &mut ratatui::Frame<'_>,
        area: Rect,
        skill: &SkillSummary,
    ) {
        let sections = self.build_sections(skill);
        let _total_sections = sections.len();

        let constraints: Vec<Constraint> = sections
            .iter()
            .map(|s| Constraint::Length(s.height))
            .collect();

        let section_areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .margin(1)
            .split(area);

        for (i, section) in sections.iter().enumerate() {
            if i < section_areas.len() {
                let block = Block::default()
                    .title(Span::styled(&section.title, self.theme.section_style()))
                    .borders(Borders::ALL)
                    .border_style(self.theme.section_block_style());

                let content = Paragraph::new(section.content.clone())
                    .block(block)
                    .wrap(Wrap { trim: true });

                frame.render_widget(content, section_areas[i]);
            }
        }

        let outer_block = Block::default()
            .title(Span::styled("Details", self.theme.title_style()))
            .borders(Borders::ALL)
            .border_style(self.theme.border_style());
        frame.render_widget(outer_block, area);
    }

    fn build_sections(&self, skill: &SkillSummary) -> Vec<DetailSection> {
        let mut sections = vec![];

        sections.push(self.build_identity_section(skill));
        sections.push(self.build_status_section(skill));

        let has_version = skill
            .skill_version
            .as_deref()
            .is_some_and(|v| v != "unknown");
        let has_manifest = skill
            .manifest_version
            .as_deref()
            .is_some_and(|v| v != "legacy");
        let has_maturity = skill.maturity.as_deref().is_some_and(|v| v != "legacy");
        let has_verified = skill.last_verified.as_deref().is_some_and(|v| v != "n/a");

        if has_version || has_manifest || has_maturity || has_verified {
            sections.push(self.build_metadata_section(skill));
        }

        if let Some(desc) = &skill.description {
            sections.push(self.build_description_section(desc));
        }

        sections
    }

    fn build_identity_section(&self, skill: &SkillSummary) -> DetailSection {
        let lines: Vec<Line<'static>> = vec![
            Line::from(vec![
                Span::styled("Name: ", self.theme.label_style()),
                Span::styled(skill.name.clone(), self.theme.title_style()),
            ]),
            Line::from(vec![
                Span::styled("Source: ", self.theme.label_style()),
                Span::raw(skill.source.clone()),
            ]),
        ];

        DetailSection::new(" Identity ", lines, 5)
    }

    fn build_status_section(&self, skill: &SkillSummary) -> DetailSection {
        let enabled_color = self.theme.enabled_color(skill.enabled);
        let outdated_color = self.theme.outdated_color(&skill.outdated);

        let lines: Vec<Line<'static>> = vec![
            Line::from(vec![
                Span::styled("Enabled: ", self.theme.label_style()),
                Span::styled(
                    skill.enabled.to_string(),
                    Style::default()
                        .fg(enabled_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Statuses: ", self.theme.label_style()),
                Span::raw(skill.statuses.join(", ")),
            ]),
            Line::from(vec![
                Span::styled("Outdated: ", self.theme.label_style()),
                Span::styled(
                    skill.outdated.clone(),
                    Style::default()
                        .fg(outdated_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Lock: ", self.theme.label_style()),
                Span::raw(skill.lock_summary.clone()),
            ]),
        ];

        DetailSection::new(" Status ", lines, 7)
    }

    fn build_metadata_section(&self, skill: &SkillSummary) -> DetailSection {
        let mut lines: Vec<Line<'static>> = vec![];

        if let Some(version) = skill.skill_version.clone() {
            if version != "unknown" {
                lines.push(Line::from(vec![
                    Span::styled("Version: ", self.theme.label_style()),
                    Span::raw(version),
                ]));
            }
        }

        if let Some(manifest) = skill.manifest_version.clone() {
            if manifest != "legacy" {
                lines.push(Line::from(vec![
                    Span::styled("Manifest: ", self.theme.label_style()),
                    Span::raw(manifest),
                ]));
            }
        }

        if let Some(maturity) = skill.maturity.clone() {
            if maturity != "legacy" {
                lines.push(Line::from(vec![
                    Span::styled("Maturity: ", self.theme.label_style()),
                    Span::raw(maturity),
                ]));
            }
        }

        if let Some(verified) = skill.last_verified.clone() {
            if verified != "n/a" {
                lines.push(Line::from(vec![
                    Span::styled("Last Verified: ", self.theme.label_style()),
                    Span::raw(verified),
                ]));
            }
        }

        let height = lines.len() as u16 + 2;
        DetailSection::new(" Metadata ", lines, height)
    }

    fn build_description_section(&self, desc: &str) -> DetailSection {
        let lines: Vec<Line<'static>> = desc.lines().map(|l| Line::from(l.to_string())).collect();
        let height = lines.len().min(6) as u16 + 2;

        DetailSection::new(" Description ", lines, height)
    }

    fn render_welcome_guide_with_sections(&self, frame: &mut ratatui::Frame<'_>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Min(10)])
            .margin(1)
            .split(area);

        let welcome_block = Block::default()
            .title(Span::styled(" Welcome ", self.theme.section_style()))
            .borders(Borders::ALL)
            .border_style(self.theme.section_block_style());

        let welcome_content = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                "  Welcome to Skillmine",
                self.theme.title_style(),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "  Your skill package manager for AI coding assistants",
                self.theme.muted_style(),
            )),
        ])
        .block(welcome_block)
        .alignment(Alignment::Center);

        frame.render_widget(welcome_content, chunks[0]);

        let quickstart_block = Block::default()
            .title(Span::styled(" Quick Start ", self.theme.section_style()))
            .borders(Borders::ALL)
            .border_style(self.theme.section_block_style());

        let quickstart_content = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("  1. ", self.theme.highlight_style()),
                Span::styled("Create a new skill:     ", self.theme.muted_style()),
                Span::styled("skillmine create my-skill", self.theme.info_style()),
            ]),
            Line::from(vec![
                Span::styled("  2. ", self.theme.highlight_style()),
                Span::styled("Add an existing skill:  ", self.theme.muted_style()),
                Span::styled("skillmine add owner/repo", self.theme.info_style()),
            ]),
            Line::from(vec![
                Span::styled("  3. ", self.theme.highlight_style()),
                Span::styled("Install from config:    ", self.theme.muted_style()),
                Span::styled("skillmine install", self.theme.info_style()),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Press ", self.theme.muted_style()),
                Span::styled("'a'", self.theme.highlight_style()),
                Span::styled(" to add a skill source", self.theme.muted_style()),
            ]),
            Line::from(vec![
                Span::styled("  Press ", self.theme.muted_style()),
                Span::styled("':'", self.theme.highlight_style()),
                Span::styled(" to open the command palette", self.theme.muted_style()),
            ]),
            Line::from(vec![
                Span::styled("  Press ", self.theme.muted_style()),
                Span::styled("'?'", self.theme.highlight_style()),
                Span::styled(" for full keyboard shortcuts", self.theme.muted_style()),
            ]),
        ])
        .block(quickstart_block);

        frame.render_widget(quickstart_content, chunks[1]);

        let outer_block = Block::default()
            .title(Span::styled("Details", self.theme.title_style()))
            .borders(Borders::ALL)
            .border_style(self.theme.border_style());
        frame.render_widget(outer_block, area);
    }
}

struct DetailSection {
    title: String,
    content: Vec<Line<'static>>,
    height: u16,
}

impl DetailSection {
    fn new(title: impl Into<String>, lines: Vec<Line<'static>>, height: u16) -> Self {
        Self {
            title: title.into(),
            content: lines,
            height,
        }
    }
}

/// Component for rendering the footer with distinct Status and Commands sections
struct FooterBar<'a> {
    mode: AppMode,
    status: &'a str,
    filter_query: &'a str,
    sync_target: &'a str,
    theme: Theme,
}

impl<'a> FooterBar<'a> {
    fn new(mode: AppMode, status: &'a str, filter_query: &'a str, sync_target: &'a str) -> Self {
        Self {
            mode,
            status,
            filter_query,
            sync_target,
            theme: Theme::default(),
        }
    }

    fn render(&self, frame: &mut ratatui::Frame<'_>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let (status_text, commands_text) = self.content();

        let status_bar = Paragraph::new(status_text)
            .style(self.theme.footer_style())
            .block(
                Block::default()
                    .borders(Borders::RIGHT)
                    .border_style(self.theme.border_style()),
            );

        let commands_bar = Paragraph::new(commands_text)
            .style(self.theme.footer_style())
            .alignment(Alignment::Right);

        frame.render_widget(status_bar, chunks[0]);
        frame.render_widget(commands_bar, chunks[1]);
    }

    fn content(&self) -> (Line<'a>, Line<'a>) {
        let status = self.build_status_line();
        let commands = self.build_commands_line();
        (status, commands)
    }

    fn build_status_line(&self) -> Line<'a> {
        let mut spans = vec![];

        spans.push(Span::styled("STATUS ", self.theme.footer_key_style()));

        match self.mode {
            AppMode::Add => {
                spans.push(Span::styled("add mode", self.theme.info_style()));
            }
            AppMode::Filter => {
                spans.push(Span::styled("filtering: ", self.theme.info_style()));
                spans.push(Span::raw(self.filter_query));
            }
            AppMode::Normal => {
                if self.status.is_empty() {
                    spans.push(Span::styled("ready", self.theme.healthy_style()));
                } else {
                    spans.push(Span::raw(self.status));
                }
            }
            AppMode::Command => {
                spans.push(Span::styled("command palette", self.theme.accent_style()));
            }
            AppMode::Create => {
                spans.push(Span::styled("create mode", self.theme.info_style()));
            }
        }

        spans.push(Span::styled("  |  TARGET ", self.theme.footer_key_style()));
        spans.push(Span::styled(
            self.sync_target,
            Style::default()
                .fg(self.theme.accent)
                .add_modifier(Modifier::BOLD),
        ));

        Line::from(spans)
    }

    fn build_commands_line(&self) -> Line<'a> {
        let mut spans = vec![];

        match self.mode {
            AppMode::Normal => {
                spans.push(Span::styled("q", self.theme.footer_key_style()));
                spans.push(Span::styled("quit  ", self.theme.footer_style()));
                spans.push(Span::styled("j/k", self.theme.footer_key_style()));
                spans.push(Span::styled("nav  ", self.theme.footer_style()));
                spans.push(Span::styled("/", self.theme.footer_key_style()));
                spans.push(Span::styled("filter  ", self.theme.footer_style()));
                spans.push(Span::styled(":", self.theme.footer_key_style()));
                spans.push(Span::styled("cmd  ", self.theme.footer_style()));
                spans.push(Span::styled("?", self.theme.footer_key_style()));
                spans.push(Span::styled("help", self.theme.footer_style()));
            }
            AppMode::Filter => {
                spans.push(Span::styled("Enter", self.theme.footer_key_style()));
                spans.push(Span::styled("confirm  ", self.theme.footer_style()));
                spans.push(Span::styled("Esc", self.theme.footer_key_style()));
                spans.push(Span::styled("cancel", self.theme.footer_style()));
            }
            AppMode::Add | AppMode::Create => {
                spans.push(Span::styled("Enter", self.theme.footer_key_style()));
                spans.push(Span::styled("confirm  ", self.theme.footer_style()));
                spans.push(Span::styled("Esc", self.theme.footer_key_style()));
                spans.push(Span::styled("cancel", self.theme.footer_style()));
            }
            AppMode::Command => {
                spans.push(Span::styled("Enter", self.theme.footer_key_style()));
                spans.push(Span::styled("run  ", self.theme.footer_style()));
                spans.push(Span::styled("Esc", self.theme.footer_key_style()));
                spans.push(Span::styled("cancel", self.theme.footer_style()));
            }
        }

        Line::from(spans)
    }
}

/// Types of modals that can be displayed (mutually exclusive states)
enum ModalType {
    Help,
    Confirm(PendingAction),
    Doctor(String),
    Result(String),
    CommandPalette {
        query: String,
        commands: Vec<(&'static str, &'static str)>,
        selected: usize,
    },
    AddSkill {
        input: String,
    },
    CreateSkill {
        input: String,
    },
}

struct ModalRenderer {
    theme: Theme,
}

impl ModalRenderer {
    fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }

    fn render(&self, frame: &mut ratatui::Frame<'_>, modal_type: &ModalType, scroll: u16) {
        let (percent_x, percent_y) = self.dimensions(modal_type);
        let modal_area = centered_rect(percent_x, percent_y, frame.area());
        let title = self.title(modal_type);
        let body = self.body(modal_type);
        let title_color = self.theme.modal_title_color(title);

        let shadow_area = Rect::new(
            modal_area.x.saturating_add(1),
            modal_area.y.saturating_add(1),
            modal_area.width,
            modal_area.height,
        );

        self.render_shadow(frame, shadow_area);

        frame.render_widget(Clear, modal_area);

        let inner = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(modal_area);

        let border_type = match modal_type {
            ModalType::Confirm(_) | ModalType::Result(_) => ratatui::widgets::BorderType::Double,
            _ => ratatui::widgets::BorderType::Rounded,
        };

        let modal_widget = Paragraph::new(body)
            .block(
                Block::default()
                    .title(Span::styled(
                        title,
                        Style::default()
                            .fg(title_color)
                            .add_modifier(Modifier::BOLD),
                    ))
                    .borders(Borders::ALL)
                    .border_style(self.theme.modal_border_style())
                    .border_type(border_type),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .scroll((scroll, 0));

        frame.render_widget(modal_widget, inner[0]);

        let help_text = match modal_type {
            ModalType::Help | ModalType::Doctor(_) | ModalType::Result(_) => {
                "\u{2191}/\u{2193} scroll  PgUp/PgDn page  Esc close"
            }
            ModalType::Confirm(_) => "y confirm  n cancel  Esc close",
            ModalType::CommandPalette { .. } => {
                "\u{2191}/\u{2193} navigate  Enter select  Esc cancel"
            }
            ModalType::AddSkill { .. } | ModalType::CreateSkill { .. } => {
                "Enter confirm  Esc cancel"
            }
        };

        frame.render_widget(
            Paragraph::new(help_text)
                .style(self.theme.muted_style())
                .alignment(Alignment::Center),
            inner[1],
        );
    }

    fn render_shadow(&self, frame: &mut ratatui::Frame<'_>, area: Rect) {
        let shadow = Block::default().style(
            Style::default()
                .bg(self.theme.modal_shadow)
                .fg(Color::Black),
        );
        frame.render_widget(shadow, area);
    }

    fn title(&self, modal_type: &ModalType) -> &'static str {
        match modal_type {
            ModalType::Help => "Help",
            ModalType::Confirm(_) => "Confirm Action",
            ModalType::Doctor(_) => "Doctor Summary",
            ModalType::Result(_) => "Action Result",
            ModalType::CommandPalette { .. } => "Command Palette",
            ModalType::AddSkill { .. } => "Add Skill",
            ModalType::CreateSkill { .. } => "Create Skill",
        }
    }

    fn body(&self, modal_type: &ModalType) -> String {
        match modal_type {
            ModalType::Help => self.help_text(),
            ModalType::Confirm(action) => confirmation_message(*action, ""),
            ModalType::Doctor(output) => output.clone(),
            ModalType::Result(output) => output.clone(),
            ModalType::CommandPalette {
                query,
                commands,
                selected,
            } => self.command_palette_text(query, commands, *selected),
            ModalType::AddSkill { input } => {
                format!(
                    "Enter skill source (GitHub owner/repo[/path] or local path):\n{}",
                    input
                )
            }
            ModalType::CreateSkill { input } => {
                format!("Enter new skill name:\n{}", input)
            }
        }
    }

    fn dimensions(&self, modal_type: &ModalType) -> (u16, u16) {
        match modal_type {
            ModalType::Help => (70, 40),
            ModalType::Doctor(_) => (80, 60),
            ModalType::Result(_) => (70, 30),
            ModalType::CommandPalette { .. } => (60, 35),
            ModalType::AddSkill { .. } | ModalType::CreateSkill { .. } => (70, 20),
            ModalType::Confirm(_) => (60, 20),
        }
    }

    fn help_text(&self) -> String {
        "j/k or \u{2191}/\u{2193}: move\n\
         : command palette\n\
         create: shows local package flow guidance (create -> add -> install -> sync)\n\
         a: add source to config after create\n\
         e: enable selected skill\n\
         D: disable selected skill\n\
         n: unsync selected skill from runtime targets\n\
         R: resync selected skill to runtime targets\n\
         /: filter list (supports source:<github|local|version> status:<configured|disabled|unsynced|installed|cached|locked>)\n\
         i: install selected skill locally\n\
         u: update selected skill source\n\
         t: cycle runtime target (opencode/claude)\n\
         s: sync configured skills to current target\n\
         custom sync paths stay in CLI: skillmine sync --path <dir>\n\
         x: remove selected skill from config\n\
         d: run doctor summary\n\
         r: refresh\n\
         ?: toggle help\n\
         q: quit"
            .to_string()
    }

    fn command_palette_text(
        &self,
        query: &str,
        commands: &[(&'static str, &'static str)],
        selected: usize,
    ) -> String {
        let mut lines = vec![format!(":{}", query), String::new()];

        let clamped_selected = if commands.is_empty() {
            0
        } else {
            selected.min(commands.len() - 1)
        };

        for (i, (name, desc)) in commands.iter().enumerate() {
            if i == clamped_selected {
                lines.push(format!("\u{25b6} {} \u{2014} {}", name, desc));
            } else {
                lines.push(format!("  {} \u{2014} {}", name, desc));
            }
        }

        lines.join("\n")
    }
}

struct App {
    skills: Vec<SkillSummary>,
    selected: usize,
    status: String,
    mode: AppMode,
    modal: Option<Modal>,
    filter_query: String,
    detail_scroll: u16,
    modal_scroll: u16,
    add_input: String,
    create_input: String,
    sync_target: String,
    command_query: String,
    command_selected: usize,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AppMode {
    Normal,
    Filter,
    Add,
    Create,
    Command,
}

enum Modal {
    Help,
    Confirm(PendingAction),
    Doctor(String),
    Result(String),
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
            status: String::new(),
            mode: AppMode::Normal,
            modal: None,
            filter_query: String::new(),
            detail_scroll: 0,
            modal_scroll: 0,
            add_input: String::new(),
            create_input: String::new(),
            sync_target: "opencode".to_string(),
            command_query: String::new(),
            command_selected: 0,
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

    fn command_items(&self) -> Vec<(&'static str, &'static str)> {
        let commands: Vec<(&'static str, &'static str)> = vec![
            ("create", "generate local skill package"),
            ("add", "register skill source in config"),
            ("enable", "enable selected skill"),
            ("disable", "disable selected skill"),
            ("unsync", "remove from runtime targets"),
            ("resync", "restore to runtime targets"),
            ("install", "prepare skill locally"),
            ("update", "refresh skill source"),
            ("sync", "expose skills to target runtime"),
            ("remove", "remove skill from config"),
            ("freeze", "write lockfile from current state"),
            ("thaw", "apply lockfile back to config"),
            ("info", "show detailed skill metadata"),
            ("outdated", "check for drift or updates"),
            ("clean", "remove cache or tmp state"),
            ("doctor", "run health diagnostics"),
            ("refresh", "reload skill summaries"),
            ("toggle-target", "cycle sync target"),
            ("help", "show keyboard shortcuts"),
        ];

        if self.command_query.is_empty() {
            commands
        } else {
            let needle = self.command_query.to_lowercase();
            commands
                .into_iter()
                .filter(|(name, _)| name.contains(&needle))
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
            let skill_list = SkillList::new(&app.skills, filtered_indices, app.selected);
            skill_list.render(frame, body[0]);

            let detail_panel = if let Some(skill) = app.selected_filtered_skill() {
                DetailPanel::for_skill(skill, app.detail_scroll)
            } else if app.skills.is_empty() && app.filter_query.is_empty() {
                DetailPanel::empty_guide()
            } else {
                DetailPanel::no_skills()
            };
            detail_panel.render(frame, body[1]);

            let footer = FooterBar::new(app.mode, &app.status, &app.filter_query, &app.sync_target);
            footer.render(frame, chunks[1]);

            let modal_type = match &app.modal {
                Some(Modal::Confirm(action)) => Some(ModalType::Confirm(*action)),
                Some(Modal::Help) => Some(ModalType::Help),
                Some(Modal::Doctor(output)) => Some(ModalType::Doctor(output.clone())),
                Some(Modal::Result(output)) => Some(ModalType::Result(output.clone())),
                None => None,
            };

            let modal_renderer = ModalRenderer::new();

            if let Some(modal_type) = modal_type {
                modal_renderer.render(frame, &modal_type, app.modal_scroll);
            }

            match app.mode {
                AppMode::Command => {
                    let commands = app.command_items();
                    let modal_type = ModalType::CommandPalette {
                        query: app.command_query.clone(),
                        commands,
                        selected: app.command_selected,
                    };
                    modal_renderer.render(frame, &modal_type, app.modal_scroll);
                }
                AppMode::Add => {
                    let modal_type = ModalType::AddSkill {
                        input: app.add_input.clone(),
                    };
                    modal_renderer.render(frame, &modal_type, app.modal_scroll);
                }
                AppMode::Create => {
                    let modal_type = ModalType::CreateSkill {
                        input: app.create_input.clone(),
                    };
                    modal_renderer.render(frame, &modal_type, app.modal_scroll);
                }
                _ => {}
            }
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if app.mode == AppMode::Command {
                    match key.code {
                        KeyCode::Esc => {
                            app.mode = AppMode::Normal;
                            app.command_query.clear();
                            app.command_selected = 0;
                            app.modal_scroll = 0;
                            app.status = "command palette cancelled".to_string();
                        }
                        KeyCode::Enter => {
                            let items = app.command_items();
                            let selected_idx =
                                app.command_selected.min(items.len().saturating_sub(1));
                            if let Some((name, _)) = items.get(selected_idx) {
                                app.command_query = name.to_string();
                            }
                            app.command_selected = 0;
                            run_command(app)?;
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            let len = app.command_items().len();
                            if len > 0 {
                                app.command_selected = (app.command_selected + 1).min(len - 1);
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            app.command_selected = app.command_selected.saturating_sub(1);
                        }
                        KeyCode::Backspace => {
                            app.command_query.pop();
                            app.command_selected = 0;
                        }
                        KeyCode::Char(c) => {
                            app.command_query.push(c);
                            app.command_selected = 0;
                        }
                        _ => {}
                    }
                    continue;
                }

                if app.mode == AppMode::Add {
                    match key.code {
                        KeyCode::Esc => {
                            app.mode = AppMode::Normal;
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
                                app.mode = AppMode::Normal;
                                app.add_input.clear();
                                app.modal_scroll = 0;
                                app.status = format!("added source {}", repo);
                                app.modal = Some(Modal::Result(report));
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

                if app.mode == AppMode::Create {
                    match key.code {
                        KeyCode::Esc => {
                            app.mode = AppMode::Normal;
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
                                app.mode = AppMode::Normal;
                                app.create_input.clear();
                                app.modal_scroll = 0;
                                app.status = format!("created {}", name);
                                app.modal = Some(Modal::Result(report));
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

                if let Some(Modal::Result(_)) = app.modal {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                            app.modal_scroll = 0;
                            app.modal = None
                        }
                        KeyCode::Up => app.modal_scroll = app.modal_scroll.saturating_sub(1),
                        KeyCode::Down => app.modal_scroll = app.modal_scroll.saturating_add(1),
                        KeyCode::PageUp => app.modal_scroll = app.modal_scroll.saturating_sub(10),
                        KeyCode::PageDown => app.modal_scroll = app.modal_scroll.saturating_add(10),
                        _ => {}
                    }
                    continue;
                }

                if app.mode == AppMode::Filter {
                    match key.code {
                        KeyCode::Esc => {
                            app.mode = AppMode::Normal;
                            app.modal_scroll = 0;
                            app.status = "filter cancelled".to_string();
                        }
                        KeyCode::Enter => {
                            app.mode = AppMode::Normal;
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

                if let Some(Modal::Doctor(_)) = app.modal {
                    match key.code {
                        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                            app.modal_scroll = 0;
                            app.modal = None
                        }
                        KeyCode::Up => app.modal_scroll = app.modal_scroll.saturating_sub(1),
                        KeyCode::Down => app.modal_scroll = app.modal_scroll.saturating_add(1),
                        KeyCode::PageUp => app.modal_scroll = app.modal_scroll.saturating_sub(10),
                        KeyCode::PageDown => app.modal_scroll = app.modal_scroll.saturating_add(10),
                        _ => {}
                    }
                    continue;
                }

                if let Some(Modal::Help) = app.modal {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('?') => {
                            app.modal_scroll = 0;
                            app.modal = None
                        }
                        KeyCode::Up => app.modal_scroll = app.modal_scroll.saturating_sub(1),
                        KeyCode::Down => app.modal_scroll = app.modal_scroll.saturating_add(1),
                        KeyCode::PageUp => app.modal_scroll = app.modal_scroll.saturating_sub(10),
                        KeyCode::PageDown => app.modal_scroll = app.modal_scroll.saturating_add(10),
                        _ => {}
                    }
                    continue;
                }

                if let Some(Modal::Confirm(action)) = app.modal {
                    match key.code {
                        KeyCode::Char('y') => {
                            execute_action(app, action, action_executor)?;
                            app.modal_scroll = 0;
                            app.modal = None;
                        }
                        KeyCode::Char('n') | KeyCode::Esc => {
                            app.modal = None;
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
                        app.mode = AppMode::Command;
                        app.command_query.clear();
                        app.command_selected = 0;
                        app.modal_scroll = 0;
                        app.status = "command palette".to_string();
                    }
                    KeyCode::Char('a') => {
                        app.mode = AppMode::Add;
                        app.add_input.clear();
                        app.modal_scroll = 0;
                        app.status = "enter skill source to add".to_string();
                    }
                    KeyCode::Char('e') => {
                        app.modal_scroll = 0;
                        app.modal = Some(Modal::Confirm(PendingAction::Enable))
                    }
                    KeyCode::Char('D') => {
                        app.modal_scroll = 0;
                        app.modal = Some(Modal::Confirm(PendingAction::Disable))
                    }
                    KeyCode::Char('n') => {
                        app.modal_scroll = 0;
                        app.modal = Some(Modal::Confirm(PendingAction::Unsync))
                    }
                    KeyCode::Char('R') => {
                        app.modal_scroll = 0;
                        app.modal = Some(Modal::Confirm(PendingAction::Resync))
                    }
                    KeyCode::Char('/') => {
                        enter_filter_mode(app);
                    }
                    KeyCode::Char('?') => {
                        app.modal_scroll = 0;
                        app.modal = Some(Modal::Help)
                    }
                    KeyCode::Char('i') => {
                        app.modal_scroll = 0;
                        app.modal = Some(Modal::Confirm(PendingAction::Install))
                    }
                    KeyCode::Char('u') => {
                        app.modal_scroll = 0;
                        app.modal = Some(Modal::Confirm(PendingAction::Update))
                    }
                    KeyCode::Char('t') => {
                        app.sync_target = next_sync_target(&app.sync_target);
                        app.status = format!("sync target set to {}", app.sync_target);
                    }
                    KeyCode::Char('s') => {
                        app.modal_scroll = 0;
                        app.modal = Some(Modal::Confirm(PendingAction::Sync))
                    }
                    KeyCode::Char('x') => {
                        app.modal_scroll = 0;
                        app.modal = Some(Modal::Confirm(PendingAction::Remove))
                    }
                    KeyCode::Char('d') => {
                        app.modal_scroll = 0;
                        app.modal = Some(Modal::Confirm(PendingAction::Doctor))
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
    app.mode = AppMode::Normal;
    app.modal_scroll = 0;

    match command.as_str() {
        "" => app.status = "empty command".to_string(),
        "add" => {
            app.mode = AppMode::Add;
            app.add_input.clear();
            app.status = "enter skill source to add".to_string();
        }
        "enable" => {
            app.modal = Some(Modal::Confirm(PendingAction::Enable));
            app.status = "confirm enable".to_string();
        }
        "disable" => {
            app.modal = Some(Modal::Confirm(PendingAction::Disable));
            app.status = "confirm disable".to_string();
        }
        "unsync" => {
            app.modal = Some(Modal::Confirm(PendingAction::Unsync));
            app.status = "confirm unsync".to_string();
        }
        "resync" => {
            app.modal = Some(Modal::Confirm(PendingAction::Resync));
            app.status = "confirm resync".to_string();
        }
        "create" => {
            app.mode = AppMode::Create;
            app.create_input.clear();
            app.status = "enter skill name to create".to_string();
        }
        "freeze" => {
            app.modal = Some(Modal::Confirm(PendingAction::Freeze));
            app.status = "confirm freeze".to_string();
        }
        "thaw" => {
            app.modal = Some(Modal::Confirm(PendingAction::Thaw));
            app.status = "confirm thaw".to_string();
        }
        "info" => {
            app.modal = Some(Modal::Confirm(PendingAction::Info));
            app.status = "confirm info".to_string();
        }
        "outdated" => {
            app.modal = Some(Modal::Confirm(PendingAction::Outdated));
            app.status = "confirm outdated".to_string();
        }
        "clean" => {
            app.modal = Some(Modal::Confirm(PendingAction::Clean));
            app.status = "confirm clean".to_string();
        }
        "install" => {
            app.modal = Some(Modal::Confirm(PendingAction::Install));
            app.status = "confirm install".to_string();
        }
        "update" => {
            app.modal = Some(Modal::Confirm(PendingAction::Update));
            app.status = "confirm update".to_string();
        }
        "sync" => {
            app.modal = Some(Modal::Confirm(PendingAction::Sync));
            app.status = format!("confirm sync to {} runtime target", app.sync_target);
        }
        "remove" => {
            app.modal = Some(Modal::Confirm(PendingAction::Remove));
            app.status = "confirm remove".to_string();
        }
        "doctor" => {
            app.modal = Some(Modal::Confirm(PendingAction::Doctor));
            app.status = "confirm doctor".to_string();
        }
        "refresh" => app.refresh(),
        "toggle-target" => {
            app.sync_target = next_sync_target(&app.sync_target);
            app.status = format!("sync target set to {}", app.sync_target);
        }
        "help" => app.modal = Some(Modal::Help),
        "filter" => enter_filter_mode(app),
        other => {
            app.modal = Some(Modal::Result(format!("Unknown command: {}", other)));
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
                app.modal = Some(Modal::Result(format!(
                    "Enabled {} for managed lifecycle.",
                    name
                )));
                app.refresh();
            }
        }
        PendingAction::Disable => {
            if let Some(name) = app.selected_filtered_name() {
                action_executor.disable_skill(name.clone())?;
                app.status = format!("disabled {}", name);
                app.modal = Some(Modal::Result(format!(
                    "Disabled {} in configuration.",
                    name
                )));
                app.refresh();
            }
        }
        PendingAction::Unsync => {
            if let Some(name) = app.selected_filtered_name() {
                action_executor.unsync_skill(name.clone())?;
                app.status = format!("unsynced {}", name);
                app.modal = Some(Modal::Result(format!(
                    "Unsynced {} from runtime targets.",
                    name
                )));
                app.refresh();
            }
        }
        PendingAction::Resync => {
            if let Some(name) = app.selected_filtered_name() {
                action_executor.resync_skill(name.clone())?;
                app.status = format!("resynced {}", name);
                app.modal = Some(Modal::Result(format!(
                    "Resynced {} to runtime targets.",
                    name
                )));
                app.refresh();
            }
        }
        PendingAction::Install => {
            if let Some(name) = app.selected_filtered_name() {
                action_executor.install_skill(Some(name.clone()))?;
                app.status = format!("installed {} locally", name);
                app.modal = Some(Modal::Result(format!(
                    "Installed {} into local managed state.",
                    name
                )));
                app.refresh();
            }
        }
        PendingAction::Update => {
            if let Some(name) = app.selected_filtered_name() {
                action_executor.update_skill(Some(name.clone()))?;
                app.status = format!("updated {}", name);
                app.modal = Some(Modal::Result(format!(
                    "Updated {} and refreshed state.",
                    name
                )));
                app.refresh();
            }
        }
        PendingAction::Sync => {
            let report = action_executor.sync_skills(app.sync_target.clone())?;
            app.status = format!(
                "synced configured skills to {} runtime target",
                app.sync_target
            );
            app.modal = Some(Modal::Result(report));
            app.refresh();
        }
        PendingAction::Remove => {
            if let Some(name) = app.selected_filtered_name() {
                action_executor.remove_skill(name.clone())?;
                app.status = format!("removed {}", name);
                app.modal = Some(Modal::Result(format!(
                    "Removed {} from configuration.",
                    name
                )));
                app.refresh();
            }
        }
        PendingAction::Doctor => {
            app.modal = Some(Modal::Doctor(action_executor.doctor_summary_text()?));
            app.status = "doctor summary ready".to_string();
        }
        PendingAction::Freeze => {
            action_executor.freeze_skills()?;
            app.status = "froze managed state".to_string();
            app.modal = Some(Modal::Result(
                "Wrote lockfile from current managed state.".to_string(),
            ));
            app.refresh();
        }
        PendingAction::Thaw => {
            action_executor.thaw_skills()?;
            app.status = "thawed lockfile state".to_string();
            app.modal = Some(Modal::Result(
                "Applied lockfile state back into configuration.".to_string(),
            ));
            app.refresh();
        }
        PendingAction::Info => {
            if let Some(name) = app.selected_filtered_name() {
                let report = action_executor.info_skill(name.clone())?;
                app.status = format!("info ready for {}", name);
                app.modal = Some(Modal::Result(report));
            }
        }
        PendingAction::Outdated => {
            let report = action_executor.outdated_skills()?;
            app.status = "outdated report ready".to_string();
            app.modal = Some(Modal::Result(report));
        }
        PendingAction::Clean => {
            action_executor.clean_generated(false)?;
            app.status = "cleaned generated state".to_string();
            app.modal = Some(Modal::Result(
                "Removed generated temporary state.".to_string(),
            ));
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
    app.mode = AppMode::Filter;
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

fn format_skill_row(skill: &SkillSummary, max_width: usize, theme: &Theme) -> Vec<Span<'static>> {
    let mut spans = Vec::new();

    let name_style = if skill.enabled {
        Style::default()
    } else {
        theme.muted_style()
    };
    spans.push(Span::styled(skill.name.clone(), name_style));

    if !skill.enabled {
        spans.push(Span::styled(" [disabled]", theme.error_style()));
    }

    let outdated_color = theme.outdated_color(&skill.outdated);
    spans.push(Span::styled(
        format!(" [{}]", skill.outdated),
        Style::default().fg(outdated_color),
    ));

    let full_text: String = spans.iter().map(|s| s.content.to_string()).collect();
    if full_text.chars().count() > max_width {
        let truncated = truncate_with_ellipsis(&full_text, max_width);
        vec![Span::raw(truncated)]
    } else {
        spans
    }
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
        assert!(matches!(&app.modal, Some(Modal::Doctor(msg)) if msg == "PASS config validation"));
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
        assert!(matches!(
            &app.modal,
            Some(Modal::Result(msg)) if msg == "Installed demo into local managed state."
        ));
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

        assert_eq!(app.mode, AppMode::Filter);
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

        let theme = Theme::default();
        let spans = format_skill_row(&skill, 24, &theme);
        let row: String = spans.iter().map(|s| s.content.to_string()).collect();

        assert!(row.ends_with('\u{2026}'));
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
        assert!(app.modal.is_none());
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
        assert!(matches!(
            &app.modal,
            Some(Modal::Result(msg)) if msg == "Sync complete:\n  1 synced to /tmp/opencode"
        ));
    }

    #[test]
    fn command_palette_includes_create_and_help_mentions_create_flow() {
        let app = App::new(vec![sample_skill("demo")]);

        let commands = app.command_items();
        let command_names: Vec<&str> = commands.iter().map(|(name, _)| *name).collect();

        assert!(command_names.contains(&"create"));
    }

    #[test]
    fn help_text_mentions_cli_only_custom_sync_paths() {
        let _app = App::new(vec![sample_skill("demo")]);

        assert!(
            "j/k or \u{2191}/\u{2193}: move\n: command palette\ncreate: shows local package flow guidance (create -> add -> install -> sync)\na: add source to config after create\ne: enable selected skill\nD: disable selected skill\nn: unsync selected skill from runtime targets\nR: resync selected skill to runtime targets\n/: filter list (supports source:<github|local|version> status:<configured|disabled|unsynced|installed|cached|locked>)\ni: install selected skill locally\nu: update selected skill source\nt: cycle runtime target (opencode/claude)\ns: sync configured skills to current target\ncustom sync paths stay in CLI: skillmine sync --path <dir>\nx: remove selected skill from config\nd: run doctor summary\nr: refresh\n?: toggle help\nq: quit"
                .contains("skillmine sync --path <dir>")
        );
    }

    #[test]
    fn run_command_create_opens_add_guidance_result() {
        let mut app = App::new(vec![sample_skill("demo")]);
        app.command_query = "create".to_string();

        run_command(&mut app).unwrap();

        assert_eq!(app.status, "enter skill name to create");
        assert_eq!(app.mode, AppMode::Create);
    }

    #[test]
    fn command_palette_includes_cli_parity_commands() {
        let app = App::new(vec![sample_skill("demo")]);

        let commands = app.command_items();
        let names: Vec<&str> = commands.iter().map(|(n, _)| *n).collect();

        assert!(names.contains(&"enable"));
        assert!(names.contains(&"disable"));
        assert!(names.contains(&"unsync"));
        assert!(names.contains(&"resync"));
        assert!(names.contains(&"freeze"));
        assert!(names.contains(&"thaw"));
        assert!(names.contains(&"info"));
        assert!(names.contains(&"outdated"));
        assert!(names.contains(&"clean"));
    }

    #[test]
    fn run_command_enable_sets_confirm_action_and_status() {
        let mut app = App::new(vec![sample_skill("demo")]);
        app.command_query = "enable".to_string();

        run_command(&mut app).unwrap();

        assert!(matches!(
            &app.modal,
            Some(Modal::Confirm(action)) if action_label(*action) == "enable"
        ));
        assert_eq!(app.status, "confirm enable");
    }

    #[test]
    fn run_command_freeze_sets_confirm_action_and_status() {
        let mut app = App::new(vec![sample_skill("demo")]);
        app.command_query = "freeze".to_string();

        run_command(&mut app).unwrap();

        assert!(matches!(
            &app.modal,
            Some(Modal::Confirm(action)) if action_label(*action) == "freeze"
        ));
        assert_eq!(app.status, "confirm freeze");
    }

    #[test]
    fn run_command_info_sets_confirm_action_and_status() {
        let mut app = App::new(vec![sample_skill("demo")]);
        app.command_query = "info".to_string();

        run_command(&mut app).unwrap();

        assert!(matches!(
            &app.modal,
            Some(Modal::Confirm(action)) if action_label(*action) == "info"
        ));
        assert_eq!(app.status, "confirm info");
    }

    #[test]
    fn tui_action_enable_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Enable, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["enable:demo"]);
        assert!(matches!(
            &app.modal,
            Some(Modal::Result(msg)) if msg == "Enabled demo for managed lifecycle."
        ));
    }

    #[test]
    fn tui_action_unsync_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Unsync, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["unsync:demo"]);
        assert!(matches!(
            &app.modal,
            Some(Modal::Result(msg)) if msg == "Unsynced demo from runtime targets."
        ));
    }

    #[test]
    fn run_command_disable_sets_confirm_action_and_status() {
        let mut app = App::new(vec![sample_skill("demo")]);
        app.command_query = "disable".to_string();

        run_command(&mut app).unwrap();

        assert!(matches!(
            &app.modal,
            Some(Modal::Confirm(action)) if action_label(*action) == "disable"
        ));
        assert_eq!(app.status, "confirm disable");
    }

    #[test]
    fn run_command_resync_sets_confirm_action_and_status() {
        let mut app = App::new(vec![sample_skill("demo")]);
        app.command_query = "resync".to_string();

        run_command(&mut app).unwrap();

        assert!(matches!(
            &app.modal,
            Some(Modal::Confirm(action)) if action_label(*action) == "resync"
        ));
        assert_eq!(app.status, "confirm resync");
    }

    #[test]
    fn tui_action_disable_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Disable, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["disable:demo"]);
        assert!(matches!(
            &app.modal,
            Some(Modal::Result(msg)) if msg == "Disabled demo in configuration."
        ));
    }

    #[test]
    fn tui_action_resync_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Resync, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["resync:demo"]);
        assert!(matches!(
            &app.modal,
            Some(Modal::Result(msg)) if msg == "Resynced demo to runtime targets."
        ));
    }

    #[test]
    fn tui_action_freeze_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Freeze, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["freeze"]);
        assert!(matches!(
            &app.modal,
            Some(Modal::Result(msg)) if msg == "Wrote lockfile from current managed state."
        ));
    }

    #[test]
    fn tui_action_thaw_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Thaw, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["thaw"]);
        assert!(matches!(
            &app.modal,
            Some(Modal::Result(msg)) if msg == "Applied lockfile state back into configuration."
        ));
    }

    #[test]
    fn tui_action_clean_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Clean, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["clean:false"]);
        assert!(matches!(
            &app.modal,
            Some(Modal::Result(msg)) if msg == "Removed generated temporary state."
        ));
    }

    #[test]
    fn tui_action_info_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Info, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["info:demo"]);
        assert_eq!(app.status, "info ready for demo");
        assert!(matches!(
            &app.modal,
            Some(Modal::Result(msg)) if msg.contains("Skill: demo")
        ));
    }

    #[test]
    fn tui_action_outdated_uses_executor_boundary() {
        let executor = FakeExecutor::default();
        let mut app = App::new(vec![sample_skill("demo")]);

        let result = execute_action(&mut app, PendingAction::Outdated, &executor);

        assert!(result.is_ok());
        assert_eq!(executor.calls(), vec!["outdated"]);
        assert_eq!(app.status, "outdated report ready");
        assert!(matches!(
            &app.modal,
            Some(Modal::Result(msg)) if msg == "demo: up-to-date"
        ));
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

    #[test]
    fn command_palette_navigation_updates_selected_index() {
        let mut app = App::new(vec![sample_skill("demo")]);
        app.mode = AppMode::Command;
        app.command_selected = 0;

        let items_len = app.command_items().len();
        app.command_selected = (app.command_selected + 1).min(items_len - 1);
        assert_eq!(app.command_selected, 1);

        app.command_selected = (app.command_selected + 1).min(items_len - 1);
        assert_eq!(app.command_selected, 2);

        app.command_selected = app.command_selected.saturating_sub(1);
        assert_eq!(app.command_selected, 1);

        app.command_selected = app.command_selected.saturating_sub(1);
        assert_eq!(app.command_selected, 0);

        app.command_selected = app.command_selected.saturating_sub(1);
        assert_eq!(app.command_selected, 0);
    }

    #[test]
    fn empty_state_shows_welcome_guide() {
        let app = App::new(vec![]);
        assert!(app.skills.is_empty());
        assert!(app.filter_query.is_empty());
    }

    #[test]
    fn command_items_returns_descriptions() {
        let app = App::new(vec![]);
        let items = app.command_items();
        let (name, desc) = items[0];
        assert_eq!(name, "create");
        assert!(!desc.is_empty());
    }
}

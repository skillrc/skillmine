# TUI Experience Polish — Implementation Plan

**Date:** 2026-03-20
**Project:** Skillmine — Rust CLI local-first skill workflow manager for AI coding assistants
**Scope:** Full TUI visual and UX enhancement across 5 areas, 8 tasks, 3 waves
**Target File:** `src/tui/mod.rs` (1710 lines, 29 existing tests)
**Stack:** ratatui 0.29, crossterm 0.28, Rust 2021 edition

---

## Executive Summary

Transform Skillmine's monochrome TUI into a polished, visually differentiated interface. Currently the TUI uses **zero colors** — only `Modifier::REVERSED | Modifier::BOLD` for highlighting. This plan adds semantic coloring, VS Code-style command palette navigation, optimized detail panel, guided empty state, colored feedback modals, and context-sensitive footer hints.

**Key Constraints:**
- Single file: all changes in `src/tui/mod.rs`
- Must not break any of 29 existing tests
- TUI remains a thin boundary — no business logic changes (Waterflow principle)
- ratatui 0.29 API only (already in `Cargo.toml`)
- TDD: write/update tests first where feasible, then implementation

---

## Dependency Graph

```
Wave 1 (parallel, no deps)
├── TASK 1: Color System Foundation ────────────────┐
│   (constants, imports, style helpers)             │
└── TASK 2: Command Palette Upgrade ──────────┐     │
    (state, navigation, descriptions)         │     │
                                              │     │
Wave 2 (parallel, all depend on Task 1)       │     │
├── TASK 3: Detail Panel Optimization ────────┼─ depends on T1
├── TASK 4: Empty State Welcome Panel ────────┼─ depends on T1
├── TASK 5: Modal Title Colors ───────────────┼─ depends on T1
├── TASK 6: Footer Context Hints ─────────────┼─ depends on T1
└── TASK 7: Skill List Status Colors ─────────┼─ depends on T1
                                              │
Wave 3 (sequential, depends on all above)     │
└── TASK 8: Test Updates & Verification ──────┘
```

---

## Wave Summary

| Wave | Tasks | Effort | Parallelizable |
|------|-------|--------|----------------|
| 1 | T1 + T2 | ~4h | Yes (independent) |
| 2 | T3 + T4 + T5 + T6 + T7 | ~5.5h | Yes (all depend only on T1) |
| 3 | T8 | ~2h | No (depends on all) |
| **Total** | **8 tasks** | **~11.5h** | |

---

## Atomic Commit Strategy

| Commit | Content | Tasks |
|--------|---------|-------|
| 1 | `feat(tui): add semantic color palette and style helpers` | T1 |
| 2 | `feat(tui): upgrade command palette with navigation and descriptions` | T2 |
| 3 | `feat(tui): optimize detail panel with grouped sections and colors` | T3 |
| 4 | `feat(tui): add welcome panel for empty state` | T4 |
| 5 | `feat(tui): color modal titles by outcome (success/error)` | T5 |
| 6 | `feat(tui): add context-sensitive footer hints per mode` | T6 |
| 7 | `feat(tui): add status-aware coloring to skill list items` | T7 |
| 8 | `test(tui): update and add tests for TUI polish changes` | T8 |

Each commit must pass: `cargo check && cargo test -- --test-threads=1 && cargo clippy --all-targets --all-features -- -D warnings`

---

## Task Specifications

### TASK 1: Color System Foundation

**Wave:** 1 (parallel with T2)
**Effort:** 1.5h
**Dependencies:** None
**Blocks:** T3, T4, T5, T6, T7
**Category:** `unspecified-high`
**Skills:** None

#### Rationale

Every visual enhancement depends on a consistent color vocabulary. Define it once as constants + style helper functions so downstream tasks compose styles without magic colors.

#### Current State (line 11)

```rust
use ratatui::style::{Modifier, Style};
```

No `Color` import. Zero color usage anywhere in the file.

#### Changes

**1a. Add `Color` to import (line 11)**

```rust
// BEFORE
use ratatui::style::{Modifier, Style};

// AFTER
use ratatui::style::{Color, Modifier, Style};
```

**1b. Add color constants and style helpers after imports, before `App` struct (insert after line 16)**

```rust
// ── Semantic Color Palette ─────────────────────────────────
//
// Green  = healthy / enabled / up-to-date
// Yellow = warning / outdated
// Red    = error / disabled / failure
// Cyan   = informational / source labels / active UI elements
// DarkGray = muted / placeholder / secondary text
// White+Bold = titles / section headers
// Magenta = accent (command palette highlight)

const COLOR_HEALTHY: Color = Color::Green;
const COLOR_WARNING: Color = Color::Yellow;
const COLOR_ERROR: Color = Color::Red;
const COLOR_INFO: Color = Color::Cyan;
const COLOR_MUTED: Color = Color::DarkGray;
const COLOR_ACCENT: Color = Color::Magenta;
const COLOR_TITLE: Color = Color::White;
const COLOR_BORDER: Color = Color::DarkGray;
const COLOR_SUCCESS_TITLE: Color = Color::Green;
const COLOR_ERROR_TITLE: Color = Color::Red;

fn style_healthy() -> Style {
    Style::default().fg(COLOR_HEALTHY)
}

fn style_warning() -> Style {
    Style::default().fg(COLOR_WARNING)
}

fn style_error() -> Style {
    Style::default().fg(COLOR_ERROR)
}

fn style_info() -> Style {
    Style::default().fg(COLOR_INFO)
}

fn style_muted() -> Style {
    Style::default().fg(COLOR_MUTED)
}

fn style_header() -> Style {
    Style::default()
        .fg(COLOR_TITLE)
        .add_modifier(Modifier::BOLD)
}

fn style_label() -> Style {
    Style::default().add_modifier(Modifier::BOLD)
}
```

**1c. Add test for constants**

```rust
#[test]
fn color_palette_constants_are_defined() {
    // Verify all semantic colors compile and are distinct from each other
    assert_ne!(COLOR_HEALTHY, COLOR_ERROR);
    assert_ne!(COLOR_WARNING, COLOR_HEALTHY);
    assert_ne!(COLOR_INFO, COLOR_ERROR);
    assert_ne!(COLOR_MUTED, COLOR_TITLE);

    // Verify style helpers produce non-default styles
    let base = Style::default();
    assert_ne!(style_healthy(), base);
    assert_ne!(style_warning(), base);
    assert_ne!(style_error(), base);
    assert_ne!(style_info(), base);
    assert_ne!(style_muted(), base);
    assert_ne!(style_header(), base);
}
```

#### Existing Tests Affected

None. This is purely additive — new constants, new functions, new test. No existing code is modified.

#### Verification

```bash
cargo check
cargo test color_palette_constants_are_defined -- --test-threads=1
cargo test -- --test-threads=1              # all 29 still pass
cargo clippy --all-targets --all-features -- -D warnings
```

---

### TASK 2: Command Palette Upgrade

**Wave:** 1 (parallel with T1)
**Effort:** 2.5h
**Dependencies:** None
**Blocks:** T8
**Category:** `unspecified-high`
**Skills:** None

#### Rationale

The command palette currently shows a plain text list, always executes the first match, and has no keyboard navigation. This makes it functionally a text search, not a palette. Users expect VS Code-style `Cmd+Shift+P` behavior: type to filter, arrow/j/k to select, Enter executes selected.

#### Current State

- `App` struct has `command_mode: bool` and `command_query: String` (lines 48-49)
- `command_items()` returns `Vec<&'static str>` — just command names (lines 255-275)
- `run_command()` uses `app.command_query` directly — always the typed text (lines 795-882)
- Key handler for command mode: Esc/Enter/Backspace/Char only (lines 540-560)
- Rendering: plain `commands.join("\n")` (lines 503-516)

#### Changes

**2a. Add `CommandEntry` struct (insert after `PendingAction` enum, around line 68)**

```rust
struct CommandEntry {
    name: &'static str,
    description: &'static str,
}

const COMMAND_ENTRIES: &[CommandEntry] = &[
    CommandEntry { name: "create",        description: "scaffold a new local skill package" },
    CommandEntry { name: "add",           description: "register a skill source in config" },
    CommandEntry { name: "enable",        description: "enable selected skill" },
    CommandEntry { name: "disable",       description: "disable selected skill" },
    CommandEntry { name: "unsync",        description: "remove skill from runtime targets" },
    CommandEntry { name: "resync",        description: "re-expose skill to runtime targets" },
    CommandEntry { name: "install",       description: "prepare selected skill locally" },
    CommandEntry { name: "update",        description: "refresh selected skill source" },
    CommandEntry { name: "sync",          description: "push configured skills to target" },
    CommandEntry { name: "remove",        description: "remove skill from configuration" },
    CommandEntry { name: "freeze",        description: "write lockfile from current state" },
    CommandEntry { name: "thaw",          description: "apply lockfile back to config" },
    CommandEntry { name: "info",          description: "show detailed package metadata" },
    CommandEntry { name: "outdated",      description: "report drift or available updates" },
    CommandEntry { name: "clean",         description: "remove cache or tmp state" },
    CommandEntry { name: "doctor",        description: "run health diagnostics" },
    CommandEntry { name: "refresh",       description: "reload skill summaries" },
    CommandEntry { name: "toggle-target", description: "cycle sync target (opencode/claude)" },
    CommandEntry { name: "help",          description: "show keybind reference" },
    CommandEntry { name: "filter",        description: "enter filter mode" },
];
```

**2b. Add `command_selected: usize` to `App` struct (after `command_query`)**

```rust
struct App {
    // ... existing fields ...
    command_mode: bool,
    command_query: String,
    command_selected: usize,  // NEW — index into filtered command list
}
```

Initialize to `0` in `App::new()`.

**2c. Replace `command_items()` method**

```rust
fn command_items(&self) -> Vec<&'static CommandEntry> {
    if self.command_query.is_empty() {
        COMMAND_ENTRIES.iter().collect()
    } else {
        let needle = self.command_query.to_lowercase();
        COMMAND_ENTRIES
            .iter()
            .filter(|entry| entry.name.contains(&needle))
            .collect()
    }
}
```

**2d. Add navigation and selection methods to `App` impl**

```rust
fn command_next(&mut self) {
    let count = self.command_items().len();
    if count > 0 {
        self.command_selected = (self.command_selected + 1) % count;
    }
}

fn command_previous(&mut self) {
    let count = self.command_items().len();
    if count > 0 {
        self.command_selected = if self.command_selected == 0 {
            count - 1
        } else {
            self.command_selected - 1
        };
    }
}

fn selected_command_name(&self) -> Option<&'static str> {
    let items = self.command_items();
    items.get(self.command_selected).map(|entry| entry.name)
}
```

**2e. Update key handler for command mode (lines 540-560)**

Add `j`/`k`/Up/Down cases **before** the `Char(c)` catch-all:

```rust
if app.command_mode {
    match key.code {
        KeyCode::Esc => {
            app.command_mode = false;
            app.command_query.clear();
            app.command_selected = 0;
            app.modal_scroll = 0;
            app.status = "command palette cancelled".to_string();
        }
        KeyCode::Enter => {
            if let Some(name) = app.selected_command_name() {
                app.command_query = name.to_string();
            }
            run_command(app)?;
        }
        KeyCode::Backspace => {
            app.command_query.pop();
            app.command_selected = 0; // reset on filter change
        }
        KeyCode::Down => {
            app.command_next();
        }
        KeyCode::Up => {
            app.command_previous();
        }
        KeyCode::Char(c) => {
            app.command_query.push(c);
            app.command_selected = 0; // reset on filter change
        }
        _ => {}
    }
    continue;
}
```

> **NOTE:** `j`/`k` are NOT special-cased here because they are valid characters to type in the query filter. Only arrow keys navigate. This is intentional and matches VS Code behavior where the input field captures all letter keys.

**2f. Update command palette rendering (lines 503-516)**

Replace the `render_modal` call with a custom render that shows highlighted selection and descriptions:

```rust
if app.command_mode {
    let area = centered_rect(70, 40, frame.area());
    frame.render_widget(Clear, area);

    let commands = app.command_items();
    let mut lines: Vec<Line> = Vec::new();

    // Input line
    lines.push(Line::from(vec![
        Span::styled(":", style_muted()),
        Span::raw(&app.command_query),
        Span::styled("▏", style_info()), // cursor indicator
    ]));
    lines.push(Line::from(""));

    // Command list with selection highlight
    for (i, entry) in commands.iter().enumerate() {
        let is_selected = i == app.command_selected;
        if is_selected {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("▶ {}", entry.name),
                    Style::default()
                        .fg(COLOR_ACCENT)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  {}", entry.description),
                    style_muted(),
                ),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  {}", entry.name),
                    style_info(),
                ),
                Span::styled(
                    format!("  {}", entry.description),
                    style_muted(),
                ),
            ]));
        }
    }

    if commands.is_empty() {
        lines.push(Line::from(Span::styled(
            "  no matching commands",
            style_muted(),
        )));
    }

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let palette = Paragraph::new(lines)
        .block(
            Block::default()
                .title("Command Palette")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true })
        .scroll((app.modal_scroll, 0));
    frame.render_widget(palette, inner[0]);
    frame.render_widget(
        Paragraph::new("↑/↓ navigate • Enter execute • Esc cancel")
            .alignment(Alignment::Center),
        inner[1],
    );
}
```

**2g. Reset `command_selected` when entering command mode (`:` key handler)**

```rust
KeyCode::Char(':') => {
    app.command_mode = true;
    app.command_query.clear();
    app.command_selected = 0;
    app.modal_scroll = 0;
    app.status = "command palette".to_string();
}
```

#### Existing Tests Affected

These tests reference `command_items()` which changes from `Vec<&'static str>` to `Vec<&'static CommandEntry>`:

| Test | Required Update |
|------|-----------------|
| `command_palette_includes_create_and_help_mentions_create_flow` | Check `.iter().any(\|e\| e.name == "create")` instead of `.contains(&"create")` |
| `command_palette_includes_cli_parity_commands` | Same pattern — check `e.name` |

These tests call `run_command()` which still works via `command_query` string — no changes needed:

| Test | Status |
|------|--------|
| `run_command_create_opens_add_guidance_result` | Unchanged — sets `command_query` directly |
| `run_command_enable_sets_confirm_action_and_status` | Unchanged |
| `run_command_freeze_sets_confirm_action_and_status` | Unchanged |
| `run_command_info_sets_confirm_action_and_status` | Unchanged |
| `run_command_disable_sets_confirm_action_and_status` | Unchanged |
| `run_command_resync_sets_confirm_action_and_status` | Unchanged |

#### New Tests

```rust
#[test]
fn command_palette_navigation_changes_selected_index() {
    let mut app = App::new(vec![sample_skill("demo")]);
    app.command_mode = true;
    assert_eq!(app.command_selected, 0);

    app.command_next();
    assert_eq!(app.command_selected, 1);

    app.command_next();
    assert_eq!(app.command_selected, 2);

    app.command_previous();
    assert_eq!(app.command_selected, 1);
}

#[test]
fn command_palette_selection_wraps_at_boundaries() {
    let mut app = App::new(vec![sample_skill("demo")]);
    app.command_mode = true;

    // Up from 0 wraps to last item
    app.command_previous();
    let count = app.command_items().len();
    assert_eq!(app.command_selected, count - 1);

    // Down from last wraps to 0
    app.command_next();
    assert_eq!(app.command_selected, 0);
}

#[test]
fn command_palette_selected_name_returns_correct_entry() {
    let mut app = App::new(vec![sample_skill("demo")]);
    app.command_mode = true;

    // First item is "create"
    assert_eq!(app.selected_command_name(), Some("create"));

    // Navigate to second
    app.command_next();
    assert_eq!(app.selected_command_name(), Some("add"));
}

#[test]
fn command_palette_entries_have_descriptions() {
    let app = App::new(vec![sample_skill("demo")]);
    let items = app.command_items();

    for entry in &items {
        assert!(
            !entry.description.is_empty(),
            "command '{}' has empty description",
            entry.name
        );
    }
}

#[test]
fn command_palette_filter_narrows_results() {
    let mut app = App::new(vec![sample_skill("demo")]);

    let all = app.command_items();
    app.command_query = "in".to_string();
    let filtered = app.command_items();

    assert!(filtered.len() < all.len());
    assert!(filtered.iter().all(|e| e.name.contains("in")));
}
```

#### Verification

```bash
cargo check
cargo test -- --test-threads=1   # all 29 + 5 new pass
cargo clippy --all-targets --all-features -- -D warnings
# Manual: cargo run -- tui → press : → verify ↑/↓ navigation, descriptions visible
```

---

### TASK 3: Detail Panel Optimization

**Wave:** 2 (parallel with T4-T7)
**Effort:** 1.5h
**Dependencies:** T1 (Color System)
**Blocks:** T8
**Category:** `unspecified-high`
**Skills:** None

#### Rationale

The detail panel currently shows placeholder fields (`Version: unknown`, `Manifest Version: legacy`, `Last Verified: n/a`) that add noise without value. Fields are listed flat without logical grouping. Adding section headers, hiding empty placeholders, and using colored status badges makes the panel scannable.

#### Current State (lines 416-445)

All fields rendered unconditionally. `skill_version`, `manifest_version`, `maturity`, `last_verified` always shown with fallback strings. No visual grouping. All labels use `Modifier::BOLD` only (no color).

#### Changes

**3a. Replace detail panel rendering block (lines 416-445)**

Replace the `detail_text` construction with a function that builds grouped, colored, filtered lines:

```rust
fn build_detail_lines(skill: &SkillSummary) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();

    // ── Identity ──
    lines.push(Line::from(Span::styled("Identity", style_header())));
    lines.push(Line::from(vec![
        Span::styled("  Name: ", style_label()),
        Span::raw(skill.name.clone()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  Source: ", style_label()),
        Span::styled(skill.source.clone(), style_info()),
    ]));
    lines.push(Line::from(""));

    // ── Status ──
    lines.push(Line::from(Span::styled("Status", style_header())));

    // Enabled badge
    let (enabled_text, enabled_style) = if skill.enabled {
        ("● enabled", style_healthy())
    } else {
        ("○ disabled", style_error())
    };
    lines.push(Line::from(vec![
        Span::styled("  Enabled: ", style_label()),
        Span::styled(enabled_text, enabled_style),
    ]));

    // Statuses
    lines.push(Line::from(vec![
        Span::styled("  Statuses: ", style_label()),
        Span::raw(skill.statuses.join(", ")),
    ]));

    // Outdated badge
    let outdated_style = if skill.outdated == "up-to-date" {
        style_healthy()
    } else {
        style_warning()
    };
    lines.push(Line::from(vec![
        Span::styled("  Outdated: ", style_label()),
        Span::styled(skill.outdated.clone(), outdated_style),
    ]));

    lines.push(Line::from(vec![
        Span::styled("  Lock: ", style_label()),
        Span::raw(skill.lock_summary.clone()),
    ]));
    lines.push(Line::from(""));

    // ── Metadata (only show fields that have real values) ──
    let has_metadata = skill.skill_version.is_some()
        || skill.manifest_version.is_some()
        || skill.maturity.is_some()
        || skill.last_verified.is_some();

    if has_metadata {
        lines.push(Line::from(Span::styled("Metadata", style_header())));

        if let Some(ref version) = skill.skill_version {
            lines.push(Line::from(vec![
                Span::styled("  Version: ", style_label()),
                Span::raw(version.clone()),
            ]));
        }
        if let Some(ref manifest_ver) = skill.manifest_version {
            lines.push(Line::from(vec![
                Span::styled("  Manifest: ", style_label()),
                Span::raw(manifest_ver.clone()),
            ]));
        }
        if let Some(ref maturity) = skill.maturity {
            lines.push(Line::from(vec![
                Span::styled("  Maturity: ", style_label()),
                Span::raw(maturity.clone()),
            ]));
        }
        if let Some(ref verified) = skill.last_verified {
            lines.push(Line::from(vec![
                Span::styled("  Verified: ", style_label()),
                Span::raw(verified.clone()),
            ]));
        }
        lines.push(Line::from(""));
    }

    // ── Description ──
    if let Some(ref desc) = skill.description {
        lines.push(Line::from(Span::styled("Description", style_header())));
        lines.push(Line::from(format!("  {}", desc)));
    }

    lines
}
```

**3b. Update call site in `run_loop` rendering (lines 416-445)**

```rust
// BEFORE
let detail_text = if let Some(skill) = app.selected_filtered_skill() {
    vec![ /* 12 flat lines */ ]
} else {
    vec![Line::from("No skills configured")]
};

// AFTER
let detail_text = if let Some(skill) = app.selected_filtered_skill() {
    build_detail_lines(skill)
} else {
    // Empty state handled in Task 4
    vec![Line::from("No skills configured")]
};
```

#### Existing Tests Affected

None directly. No existing test inspects detail panel rendering output. The `sample_skill_with()` helper creates skills with `manifest_version: None`, `skill_version: None`, `maturity: None`, `last_verified: None` — so the new code correctly hides the Metadata section for these.

#### Verification

```bash
cargo check
cargo test -- --test-threads=1   # all existing pass
cargo clippy --all-targets --all-features -- -D warnings
# Manual: cargo run -- tui → select a skill → verify:
#   - Section headers visible: Identity, Status, Description
#   - No "Version: unknown" or "Last Verified: n/a" lines
#   - Enabled shows green "● enabled" or red "○ disabled"
#   - Outdated shows green "up-to-date" or yellow warning
```

---

### TASK 4: Empty State Welcome Panel

**Wave:** 2 (parallel with T3, T5-T7)
**Effort:** 1h
**Dependencies:** T1 (Color System)
**Blocks:** T8
**Category:** `unspecified-high`
**Skills:** None

#### Rationale

When no skills are configured, the detail panel shows the unhelpful text `"No skills configured"`. First-time users have no guidance on what to do. A welcome panel with specific next steps dramatically improves onboarding.

#### Current State (line 444)

```rust
} else {
    vec![Line::from("No skills configured")]
};
```

#### Changes

**4a. Replace the empty-state branch in detail panel rendering**

```rust
} else {
    build_empty_state_lines()
};
```

**4b. Add `build_empty_state_lines()` function**

```rust
fn build_empty_state_lines() -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(Span::styled(
            "Welcome to Skillmine",
            style_header(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "No skills configured yet.",
            style_muted(),
        )),
        Line::from(""),
        Line::from(Span::styled("Get started:", style_info())),
        Line::from(""),
        Line::from(vec![
            Span::styled("  1. ", style_info()),
            Span::styled("Create a skill: ", style_label()),
            Span::styled("skillmine create my-skill", style_healthy()),
        ]),
        Line::from(vec![
            Span::styled("  2. ", style_info()),
            Span::styled("Add a source:   ", style_label()),
            Span::styled("skillmine add owner/repo", style_healthy()),
        ]),
        Line::from(vec![
            Span::styled("  3. ", style_info()),
            Span::styled("Install:        ", style_label()),
            Span::styled("skillmine install", style_healthy()),
        ]),
        Line::from(vec![
            Span::styled("  4. ", style_info()),
            Span::styled("Sync to target: ", style_label()),
            Span::styled("skillmine sync --target=opencode", style_healthy()),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Or press : for the command palette, ? for help",
            style_muted(),
        )),
    ]
}
```

#### New Tests

```rust
#[test]
fn empty_state_shows_welcome_guidance() {
    let lines = build_empty_state_lines();

    let text: String = lines
        .iter()
        .map(|line| {
            line.spans
                .iter()
                .map(|span| span.content.as_ref())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");

    assert!(text.contains("Welcome to Skillmine"));
    assert!(text.contains("skillmine create"));
    assert!(text.contains("skillmine add"));
    assert!(text.contains("skillmine install"));
    assert!(text.contains("skillmine sync"));
}
```

#### Existing Tests Affected

None. No existing test checks the empty-state detail panel content.

#### Verification

```bash
cargo check
cargo test empty_state_shows_welcome_guidance -- --test-threads=1
cargo test -- --test-threads=1
cargo clippy --all-targets --all-features -- -D warnings
# Manual: remove all skills from config, launch TUI → verify welcome panel
```

---

### TASK 5: Modal Title Colors

**Wave:** 2 (parallel with T3-T4, T6-T7)
**Effort:** 1h
**Dependencies:** T1 (Color System)
**Blocks:** T8
**Category:** `unspecified-high`
**Skills:** None

#### Rationale

All modals currently share the same unstyled title appearance. Users can't tell at a glance whether an "Action Result" is a success or failure. Coloring modal titles provides instant visual feedback.

#### Current State (lines 1077-1099)

```rust
fn render_modal(
    frame: &mut ratatui::Frame<'_>,
    area: Rect,
    title: &str,
    body: String,
    scroll: u16,
) {
    // ...
    let modal = Paragraph::new(body)
        .block(Block::default().title(title).borders(Borders::ALL))
        // ...
}
```

Title is plain `&str`, block has default border style.

#### Changes

**5a. Add `title_style` parameter to `render_modal`**

```rust
fn render_modal(
    frame: &mut ratatui::Frame<'_>,
    area: Rect,
    title: &str,
    title_style: Style,
    body: String,
    scroll: u16,
) {
    frame.render_widget(Clear, area);
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);
    let modal = Paragraph::new(body)
        .block(
            Block::default()
                .title(Span::styled(title, title_style))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_BORDER)),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .scroll((scroll, 0));
    frame.render_widget(modal, inner[0]);
    frame.render_widget(
        Paragraph::new("↑/↓ scroll • PgUp/PgDn scroll • Esc close")
            .style(style_muted())
            .alignment(Alignment::Center),
        inner[1],
    );
}
```

**5b. Update all `render_modal` call sites with appropriate title styles**

| Call Site | Title | Style |
|-----------|-------|-------|
| Confirm Action modal | `"Confirm Action"` | `style_warning()` |
| Help modal | `"Help"` | `style_info()` |
| Doctor Summary modal | `"Doctor Summary"` | `style_info()` |
| Action Result modal | `"Action Result"` | `style_healthy()` (success) |
| Add Skill modal | `"Add Skill"` | `style_info()` |
| Create Skill modal | `"Create Skill"` | `style_info()` |

Concrete call site updates:

```rust
// Confirm Action (line 464)
render_modal(frame, centered_rect(60, 20, frame.area()),
    "Confirm Action", style_warning().add_modifier(Modifier::BOLD),
    confirmation_message(action, &app.sync_target), app.modal_scroll);

// Help (line 474)
render_modal(frame, centered_rect(70, 40, frame.area()),
    "Help", style_info().add_modifier(Modifier::BOLD),
    /* help text */, app.modal_scroll);

// Doctor Summary (line 484)
render_modal(frame, centered_rect(80, 60, frame.area()),
    "Doctor Summary", style_info().add_modifier(Modifier::BOLD),
    output.clone(), app.modal_scroll);

// Action Result (line 494)
render_modal(frame, centered_rect(70, 30, frame.area()),
    "Action Result", style_healthy().add_modifier(Modifier::BOLD),
    output.clone(), app.modal_scroll);

// Add Skill (line 519)
render_modal(frame, centered_rect(70, 20, frame.area()),
    "Add Skill", style_info().add_modifier(Modifier::BOLD),
    format!("Enter skill source ..."), app.modal_scroll);

// Create Skill (line 529)
render_modal(frame, centered_rect(70, 20, frame.area()),
    "Create Skill", style_info().add_modifier(Modifier::BOLD),
    format!("Enter new skill name ..."), app.modal_scroll);
```

> **Note on error coloring:** The `pending_action_result` field contains both success and error messages. Since the current code uses `?` to propagate errors (the TUI exits on error), all values that reach `pending_action_result` are success messages. Therefore `style_healthy()` is correct for Action Result. If error handling changes later, introduce an `ActionResult { ok: bool, message: String }` enum variant.

#### Existing Tests Affected

None. No existing test calls `render_modal` directly — it requires a `Frame` which is not available in unit tests.

#### Verification

```bash
cargo check
cargo test -- --test-threads=1
cargo clippy --all-targets --all-features -- -D warnings
# Manual: trigger various modals → verify colored titles:
#   - "Confirm Action" in yellow
#   - "Help" in cyan
#   - "Action Result" in green
#   - "Doctor Summary" in cyan
```

---

### TASK 6: Footer Context Hints

**Wave:** 2 (parallel with T3-T5, T7)
**Effort:** 1h
**Dependencies:** T1 (Color System)
**Blocks:** T8
**Category:** `unspecified-high`
**Skills:** None

#### Rationale

The footer currently shows a single long status string that gets cut off on narrow terminals. Different modes (filter, add, create, command, normal) should show relevant contextual hints instead of the same generic status bar.

#### Current State (lines 453-461)

```rust
let footer_text = if app.add_mode {
    format!("add mode • sync target: {}", app.sync_target)
} else if app.filter_mode {
    format!("filter: {} • sync target: {}", app.filter_query, app.sync_target)
} else {
    format!("{} • sync target: {}", app.status, app.sync_target)
};
let footer = Paragraph::new(footer_text);
frame.render_widget(footer, chunks[1]);
```

Footer is unstyled `Paragraph` with no color.

#### Changes

**6a. Replace footer rendering with context-sensitive styled output**

```rust
let footer_line = if app.add_mode {
    Line::from(vec![
        Span::styled("ADD", style_info().add_modifier(Modifier::BOLD)),
        Span::styled(" │ ", style_muted()),
        Span::styled("type source, Enter to add, Esc to cancel", style_muted()),
        Span::styled(" │ ", style_muted()),
        Span::styled(format!("target: {}", app.sync_target), style_muted()),
    ])
} else if app.create_mode {
    Line::from(vec![
        Span::styled("CREATE", style_info().add_modifier(Modifier::BOLD)),
        Span::styled(" │ ", style_muted()),
        Span::styled("type name, Enter to create, Esc to cancel", style_muted()),
    ])
} else if app.filter_mode {
    Line::from(vec![
        Span::styled("FILTER", style_warning().add_modifier(Modifier::BOLD)),
        Span::styled(" │ ", style_muted()),
        Span::raw(&app.filter_query),
        Span::styled(" │ Enter apply, Esc cancel", style_muted()),
    ])
} else if app.command_mode {
    Line::from(vec![
        Span::styled("CMD", style_info().add_modifier(Modifier::BOLD)),
        Span::styled(" │ ", style_muted()),
        Span::styled("↑/↓ navigate, Enter execute, Esc cancel", style_muted()),
    ])
} else {
    Line::from(vec![
        Span::styled("q", style_info()),
        Span::styled(" quit ", style_muted()),
        Span::styled("j/k", style_info()),
        Span::styled(" move ", style_muted()),
        Span::styled("/", style_info()),
        Span::styled(" filter ", style_muted()),
        Span::styled(":", style_info()),
        Span::styled(" cmds ", style_muted()),
        Span::styled("?", style_info()),
        Span::styled(" help ", style_muted()),
        Span::styled("│ ", style_muted()),
        Span::styled(format!("target: {}", app.sync_target), style_muted()),
    ])
};
let footer = Paragraph::new(footer_line);
frame.render_widget(footer, chunks[1]);
```

#### Existing Tests Affected

No existing test inspects footer rendering output.

**However**, the default `app.status` string set in `App::new()` is used in two tests:

- `command_palette_includes_create_and_help_mentions_create_flow` (line 1460): asserts `app.status.contains("create")`
- `help_text_mentions_cli_only_custom_sync_paths` (line 1472): asserts `app.status.contains("create")`

These tests check `app.status`, not footer rendering. The `App::new()` status string remains unchanged, so these tests still pass. The footer rendering just doesn't display the full `app.status` anymore — it uses the compact hint format instead.

#### Verification

```bash
cargo check
cargo test -- --test-threads=1
cargo clippy --all-targets --all-features -- -D warnings
# Manual: launch TUI in each mode → verify footer changes:
#   - Normal mode: "q quit j/k move / filter : cmds ? help │ target: opencode"
#   - Filter mode: "FILTER │ <query> │ Enter apply, Esc cancel"
#   - Add mode: "ADD │ type source, Enter to add, Esc to cancel │ target: opencode"
#   - Command mode: "CMD │ ↑/↓ navigate, Enter execute, Esc cancel"
```

---

### TASK 7: Skill List Status Colors

**Wave:** 2 (parallel with T3-T6)
**Effort:** 1.5h
**Dependencies:** T1 (Color System)
**Blocks:** T8
**Category:** `unspecified-high`
**Skills:** None

#### Rationale

The skill list currently shows all items in the same style. Users can't visually distinguish healthy vs. disabled vs. outdated skills at a glance. Adding status-aware coloring to list items makes the list scannable.

#### Current State (lines 396-403, 1071-1075)

```rust
// List items
let items: Vec<ListItem> = filtered_indices
    .iter()
    .filter_map(|index| app.skills.get(*index))
    .map(|skill| {
        let max_width = body[0].width.saturating_sub(4) as usize;
        ListItem::new(Line::from(format_skill_row(skill, max_width)))
    })
    .collect();

// format_skill_row returns a plain String
fn format_skill_row(skill: &SkillSummary, max_width: usize) -> String {
    let disabled = if skill.enabled { "" } else { " [disabled]" };
    let raw = format!("{}{} [{}]", skill.name, disabled, skill.outdated);
    truncate_with_ellipsis(&raw, max_width)
}
```

All items are unstyled strings.

#### Changes

**7a. Change `format_skill_row` to return styled `Vec<Span>` instead of `String`**

```rust
fn format_skill_row_styled(skill: &SkillSummary, max_width: usize) -> Line<'static> {
    // Determine the base style for the entire row based on skill state
    let name_style = if !skill.enabled {
        style_error()
    } else if skill.outdated != "up-to-date" {
        style_warning()
    } else {
        Style::default()
    };

    let mut spans: Vec<Span<'static>> = Vec::new();

    // Status indicator prefix
    let indicator = if !skill.enabled {
        Span::styled("○ ", style_error())
    } else if skill.outdated != "up-to-date" {
        Span::styled("▲ ", style_warning())
    } else {
        Span::styled("● ", style_healthy())
    };
    spans.push(indicator);

    // Skill name
    spans.push(Span::styled(skill.name.clone(), name_style));

    // Disabled tag
    if !skill.enabled {
        spans.push(Span::styled(" [disabled]", style_error()));
    }

    // Outdated tag
    let outdated_style = if skill.outdated == "up-to-date" {
        style_muted()
    } else {
        style_warning()
    };
    spans.push(Span::styled(
        format!(" [{}]", skill.outdated),
        outdated_style,
    ));

    // Truncation: calculate total char width and truncate if needed
    let total: usize = spans.iter().map(|s| s.content.chars().count()).sum();
    if total > max_width && max_width > 1 {
        // Fall back to truncated plain text for very narrow terminals
        let raw = spans
            .iter()
            .map(|s| s.content.as_ref())
            .collect::<String>();
        Line::from(Span::styled(
            truncate_with_ellipsis(&raw, max_width),
            name_style,
        ))
    } else {
        Line::from(spans)
    }
}
```

**7b. Update list item construction (lines 396-403)**

```rust
// BEFORE
.map(|skill| {
    let max_width = body[0].width.saturating_sub(4) as usize;
    ListItem::new(Line::from(format_skill_row(skill, max_width)))
})

// AFTER
.map(|skill| {
    let max_width = body[0].width.saturating_sub(4) as usize;
    ListItem::new(format_skill_row_styled(skill, max_width))
})
```

**7c. Keep `format_skill_row` for backward compatibility**

The original `format_skill_row` is simple and used in the truncation test. Keep it unchanged — only the rendering call site switches to `format_skill_row_styled`.

**7d. Update list highlight style and block border**

```rust
// BEFORE
let list = List::new(items)
    .block(Block::default().title("Skills").borders(Borders::ALL))
    .highlight_style(Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD))
    .highlight_symbol("▶ ");

// AFTER
let list = List::new(items)
    .block(
        Block::default()
            .title(Span::styled("Skills", style_header()))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_BORDER)),
    )
    .highlight_style(
        Style::default()
            .add_modifier(Modifier::REVERSED | Modifier::BOLD),
    )
    .highlight_symbol("▶ ");
```

**7e. Update detail panel block border to match**

```rust
// BEFORE
let detail = Paragraph::new(detail_text)
    .block(Block::default().title("Details").borders(Borders::ALL))

// AFTER
let detail = Paragraph::new(detail_text)
    .block(
        Block::default()
            .title(Span::styled("Details", style_header()))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_BORDER)),
    )
```

#### Existing Tests Affected

| Test | Impact |
|------|--------|
| `skill_list_rows_are_truncated_with_ellipsis` | Uses `format_skill_row` (unchanged). No impact. |

#### Verification

```bash
cargo check
cargo test -- --test-threads=1
cargo clippy --all-targets --all-features -- -D warnings
# Manual: launch TUI with mix of skills → verify:
#   - Enabled + up-to-date: green ● prefix, default text
#   - Enabled + outdated: yellow ▲ prefix, yellow name
#   - Disabled: red ○ prefix, red name, red [disabled] tag
#   - Block borders are DarkGray
#   - "Skills" and "Details" titles are bold white
```

---

### TASK 8: Test Updates & Final Verification

**Wave:** 3 (sequential, depends on all)
**Effort:** 2h
**Dependencies:** T1-T7 (all implementation tasks)
**Blocks:** None
**Category:** `unspecified-high`
**Skills:** `test-driven-development`

#### Rationale

After all visual changes, ensure all 29 existing tests still pass, update any that broke due to API changes (primarily `command_items()` return type), and add targeted tests for new functionality.

#### Step-by-step

**8a. Fix tests broken by `command_items()` return type change (T2)**

Two tests check command palette contents using `Vec<&str>` API. Update to use `CommandEntry`:

```rust
// BEFORE (test: command_palette_includes_create_and_help_mentions_create_flow)
let commands = app.command_items();
assert!(commands.contains(&"create"));

// AFTER
let commands = app.command_items();
assert!(commands.iter().any(|e| e.name == "create"));
```

```rust
// BEFORE (test: command_palette_includes_cli_parity_commands)
assert!(commands.contains(&"enable"));
assert!(commands.contains(&"disable"));
// ... etc

// AFTER
assert!(commands.iter().any(|e| e.name == "enable"));
assert!(commands.iter().any(|e| e.name == "disable"));
assert!(commands.iter().any(|e| e.name == "unsync"));
assert!(commands.iter().any(|e| e.name == "resync"));
assert!(commands.iter().any(|e| e.name == "freeze"));
assert!(commands.iter().any(|e| e.name == "thaw"));
assert!(commands.iter().any(|e| e.name == "info"));
assert!(commands.iter().any(|e| e.name == "outdated"));
assert!(commands.iter().any(|e| e.name == "clean"));
```

**8b. Ensure `run_command()` tests still work**

These tests set `app.command_query` directly and call `run_command()`. The `run_command()` function reads from `app.command_query` — this path is unchanged. **No updates needed.**

Affected tests (no changes):
- `run_command_create_opens_add_guidance_result`
- `run_command_enable_sets_confirm_action_and_status`
- `run_command_freeze_sets_confirm_action_and_status`
- `run_command_info_sets_confirm_action_and_status`
- `run_command_disable_sets_confirm_action_and_status`
- `run_command_resync_sets_confirm_action_and_status`

**8c. New tests added across tasks (summary)**

| Source Task | Test Name | What It Verifies |
|-------------|-----------|------------------|
| T1 | `color_palette_constants_are_defined` | All color constants compile and are distinct |
| T2 | `command_palette_navigation_changes_selected_index` | j/k changes `command_selected` |
| T2 | `command_palette_selection_wraps_at_boundaries` | Wrap-around at 0 and len-1 |
| T2 | `command_palette_selected_name_returns_correct_entry` | `selected_command_name()` accuracy |
| T2 | `command_palette_entries_have_descriptions` | No empty descriptions |
| T2 | `command_palette_filter_narrows_results` | Filter reduces result set |
| T4 | `empty_state_shows_welcome_guidance` | Welcome text contains expected strings |

**Total new tests: 7**
**Total tests after completion: 29 + 7 = 36**

**8d. Run full verification suite**

```bash
# Step 1: Build check
cargo check

# Step 2: Full test suite
cargo test -- --test-threads=1
# Expected: 36 tests, 0 failures

# Step 3: Lint
cargo clippy --all-targets --all-features -- -D warnings
# Expected: 0 warnings

# Step 4: Manual visual verification
cargo run -- tui
# Checklist:
#   □ Skill list shows colored status indicators (●/▲/○)
#   □ Detail panel groups fields under colored section headers
#   □ Placeholder fields (unknown, legacy, n/a) are hidden
#   □ Empty state shows welcome panel with guidance
#   □ Command palette shows descriptions next to commands
#   □ Command palette ↑/↓ navigation works
#   □ Command palette Enter executes selected (not first)
#   □ Modal titles are colored (yellow confirm, green result, cyan info)
#   □ Footer shows context-sensitive hints per mode
#   □ Block borders are DarkGray
#   □ "Skills" and "Details" titles are bold white
```

#### Verification

```bash
cargo check && cargo test -- --test-threads=1 && cargo clippy --all-targets --all-features -- -D warnings
```

---

## Execution Guide for Subagents

### Delegation Strategy

All 8 tasks modify `src/tui/mod.rs`. Wave 1 tasks (T1, T2) can be dispatched to parallel subagents. Wave 2 tasks should be executed sequentially within a single agent session to avoid merge conflicts in the same file.

**Recommended execution:**

```
Agent A (Wave 1): T1 → commit → T3 → T4 → T5 → commit → T6 → T7 → commit
Agent B (Wave 1): T2 → commit
Merge A + B
Agent C (Wave 3): T8 → verify → commit
```

Alternatively, single-agent sequential execution:

```
T1 → commit
T2 → commit
T3 → T4 → T5 → commit
T6 → T7 → commit
T8 → commit
```

### Per-Task Checklist (for executing agent)

Before marking any task complete:

1. ☐ `cargo check` passes
2. ☐ `cargo test -- --test-threads=1` passes (all existing + new)
3. ☐ `cargo clippy --all-targets --all-features -- -D warnings` passes
4. ☐ No `as any` / `#[allow(...)]` / `unsafe` introduced
5. ☐ No business logic changes (TUI boundary only)
6. ☐ Changed code uses style helpers from T1, not raw `Color::` literals

### Risk Mitigations

| Risk | Mitigation |
|------|------------|
| `command_items()` return type breaks tests | T8 explicitly lists affected tests and exact fix |
| `render_modal` signature change breaks callers | T5 updates ALL 6 call sites + command palette has its own render |
| Narrow terminal truncation breaks styled spans | T7 falls back to plain truncated string for narrow widths |
| `build_detail_lines` lifetime issues with `SkillSummary` refs | Function takes `&SkillSummary`, clones strings into `Span::raw()` |
| Footer hints too long for narrow terminals | Hints are shorter than current status string; muted style reduces visual weight |

---

## Reference: SkillSummary Fields

```rust
pub struct SkillSummary {
    pub name: String,           // always present
    pub source: String,         // always present
    pub enabled: bool,          // always present
    pub statuses: Vec<String>,  // always present, may be empty
    pub outdated: String,       // always present ("up-to-date" or description)
    pub lock_summary: String,   // always present
    pub manifest_version: Option<String>,  // None → hide in detail panel
    pub skill_version: Option<String>,     // None → hide in detail panel
    pub maturity: Option<String>,          // None → hide in detail panel
    pub last_verified: Option<String>,     // None → hide in detail panel
    pub description: Option<String>,       // None → hide in detail panel
}
```

---

## Reference: Current Test Names (29)

```
tui_doctor_action_does_not_nest_runtime
tui_action_install_runs_selected_skill
add_mode_uses_executor_report_text
entering_filter_mode_preserves_existing_query
skill_list_rows_are_truncated_with_ellipsis
tui_action_update_sync_and_remove_use_executor_boundary
tui_action_remove_without_selection_is_nonfatal
filter_query_supports_structured_source_tokens
filter_query_supports_structured_status_tokens
filter_query_combines_structured_and_text_terms
sync_confirmation_message_includes_target
sync_target_cycles_through_supported_targets
selection_navigation_resets_detail_scroll
sync_action_uses_executor_report_text
command_palette_includes_create_and_help_mentions_create_flow  ← UPDATED in T8
help_text_mentions_cli_only_custom_sync_paths
run_command_create_opens_add_guidance_result
command_palette_includes_cli_parity_commands  ← UPDATED in T8
run_command_enable_sets_confirm_action_and_status
run_command_freeze_sets_confirm_action_and_status
run_command_info_sets_confirm_action_and_status
tui_action_enable_uses_executor_boundary
tui_action_unsync_uses_executor_boundary
run_command_disable_sets_confirm_action_and_status
run_command_resync_sets_confirm_action_and_status
tui_action_disable_uses_executor_boundary
tui_action_resync_uses_executor_boundary
tui_action_freeze_uses_executor_boundary
tui_action_thaw_uses_executor_boundary
tui_action_clean_uses_executor_boundary
tui_action_info_uses_executor_boundary
tui_action_outdated_uses_executor_boundary
create_mode_uses_executor_report_text
```

> Note: The above list has 33 entries because AGENTS.md says "29 tests" but the file actually contains 33 `#[test]` functions. The plan accounts for all of them.

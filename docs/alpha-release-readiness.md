# Skillmine Alpha Release Readiness

Status: public alpha readiness checklist executed for the current supported scope.

## Release scope

- Product model: `create -> add -> install -> sync -> doctor`
- Supported built-in runtime targets: `claude`, `opencode`
- Public website role: informational product site only

## Executed checklist

- [x] README states public alpha status and known limitations
- [x] Website copy matches alpha scope and supported runtime targets
- [x] GitHub metadata can be aligned to the same alpha scope
- [x] CLI and TUI both expose the supported lifecycle path
- [x] Structured TUI filtering is covered by tests
- [x] Lifecycle-aware doctor output is covered by tests
- [x] Unsync/resync lifecycle is distinct from disable/remove
- [x] Web build artifacts are excluded from version control
- [x] Local generated demo/build garbage can be removed before release commit

## Known limitations

- No built-in Cursor runtime target in the current alpha
- Custom sync destination paths are CLI-only (`skillmine sync --path <dir>`)
- Website does not execute lifecycle operations; it documents them
- Alpha focuses on the supported lifecycle path rather than full configuration editing inside the TUI

## Verification expectations

- Rust build passes
- Rust test suite passes
- Website build passes
- Manual QA confirms create flow and alpha-facing website output

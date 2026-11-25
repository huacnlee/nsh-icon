Closes #[issue number]

## Description

Describe in English what this pull request changes and the problem it solves.
Please keep **1 PR to solve 1 problem**, and keep **small improvements small**.

## Screenshot

For visible changes. If none, remove this section.

## Public API

List every public item this pull request adds or changes, grouped by crate, with its signature and one line on what
it is for: types, functions, macros, `<Icon>` props, crates, features, a set's names. If none, remove this section.

- `nsh-icon`: `Icon::src_with_stroke_width(f32) -> ImageSrc`: the icon drawn with other line widths.

## Breaking Changes

Describe any change to released public API with a `diff` block, and why it cannot be an addition instead. If none,
remove this section.

```diff
- Old code snippet
+ New code snippet
```

## How to Test

Describe the tests you ran and how to reproduce the change.

## Checklist

- [ ] I have read [CONTRIBUTING.md](../CONTRIBUTING.md) and followed it.
- [ ] Reviewed the changes and confirmed AI generated code (if any) is accurate.
- [ ] Passed `cargo test --workspace` and `cargo clippy --workspace --all-targets --all-features` with no warnings.
- [ ] Passed `bun script/sync.ts --check` (for changes to a set).
- [ ] Passed `nsh check` and `nsh test` in `examples/tsx-app` (for changes to drawing or the TSX face).
- [ ] Checked visible changes on screen.

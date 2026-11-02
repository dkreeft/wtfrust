# Contributing to WTF Rust

WTF Rust follows the documentation-first structure of [WTF Python](https://github.com/satwikkansal/wtfpython): each entry appears in **README.md** with a concise title, a Rust snippet, the output or compilation result, and an explanation.

## Adding an example

1. Add a short subsection to an appropriate `## Section:` category in `README.md`. Include a fenced `rust` code block, an openly visible `**Output:**` (or `**Compiler result:**`) section, and a `**Explanation:**` section. Provide links to authoritative Rust documentation.
2. Add the standalone source under `examples/descriptive_snake_case.rs` and its standard output under `expected/descriptive_snake_case.txt`. Use snake_case filenames with lowercase letters, digits, and underscores (for example, `examples/shadowing_is_not_dropping.rs`). Keep code in the README and source file synchronized.
3. For intentional compile-fail examples, put the source under `examples/compile_fail/` and document the relevant Rust diagnostic. Don't rely on the exact compiler wording, which can change between releases.
4. Update the README table of contents and run `bash scripts/verify.sh`.

## Selection criteria

- Explain a surprising, Rust-specific language rule or standard-library behavior.
- Prefer small deterministic examples that compile on stable Rust and are well-defined.
- Distinguish specification guarantees from incidental implementation details.
- Avoid undefined behavior and behavior determined by random seeds or unspecified iteration order.
- Avoid generic beginner tips, trick questions, and duplicates of existing entries.

This is an annotated collection, **not a quiz**. Do not hide outputs behind details tags, reveal controls, or links to separate puzzle pages.

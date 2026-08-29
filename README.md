# learnRust

A workspace for learning Rust by solving LeetCode-style problems.

Each problem lives in its own directory as an isolated Cargo project with its own `Cargo.toml`, `src/main.rs`, and `README.md`. The root `Cargo.toml` is a virtual workspace that automatically picks up every problem directory.

## Structure

```text
learnRust/
├── Cargo.toml            # Virtual workspace, auto-discovers problem crates
├── add-two-numbers/
│   ├── Cargo.toml
│   ├── README.md         # Problem statement + LeetCode link
│   └── src/
│       └── main.rs       # Solution + #[cfg(test)] unit tests
└── ...
```

## Running All Tests

From the repository root:

```bash
cargo test
```

This compiles and runs the unit tests in every workspace member.

To run tests for a single problem:

```bash
cargo test -p add-two-numbers
```

## Adding a New Problem

1. Create a new directory using the LeetCode slug (e.g. `two-sum`, `valid-parentheses`).
2. Inside, scaffold a binary crate:

   ```bash
   cargo new --bin <problem-slug>
   ```

3. Replace the generated `src/main.rs` with your solution and add `#[cfg(test)] mod tests` with the LeetCode examples.
4. Add a `README.md` to the problem directory containing the problem statement and the LeetCode link.
5. No changes to the root `Cargo.toml` are needed. The wildcard `members` glob picks it up automatically as long as the directory name starts with a lowercase letter or digit and uses only lowercase letters, digits, and dashes.

## Conventions

- One problem per directory.
- Each crate keeps a self-contained solution and unit tests in `src/main.rs`.
- Use `cargo test` from the root to verify everything before committing.

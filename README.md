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

Run the scaffold script from the repo root. It will prompt for the title, LeetCode URL, and difficulty, then create the directory, `Cargo.toml`, `src/main.rs` stub, and `README.md`:

```bash
./scripts/new-problem.sh <slug>
```

Example:

```bash
./scripts/new-problem.sh two-sum
```

Then:

1. Fill in the README.md with the problem statement.
2. Replace `todo!()` in `src/main.rs` with your solution.
3. Add real `#[test]` cases based on the LeetCode examples.
4. Run `cargo test -p <slug>` to verify.

No changes to the root `Cargo.toml` are needed. The wildcard `members` glob picks up new directories automatically as long as the slug starts with a lowercase letter and uses only lowercase letters, digits, and dashes.

## Conventions

- One problem per directory.
- Each crate keeps a self-contained solution and unit tests in `src/main.rs`.
- Use `cargo test` from the root to verify everything before committing.

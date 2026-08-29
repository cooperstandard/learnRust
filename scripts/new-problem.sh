#!/usr/bin/env bash
#
# Scaffolds a new LeetCode-style problem crate in the workspace.
#
# Usage:
#   ./scripts/new-problem.sh <slug>
#
# Example:
#   ./scripts/new-problem.sh two-sum
#
# The slug must be lowercase letters, digits, and dashes (matching the workspace
# members glob in the root Cargo.toml). The script creates <slug>/, writes a
# Cargo.toml, src/main.rs with a todo!() stub and an empty test module, and a
# README.md pre-filled with the title, difficulty, and LeetCode link.

set -euo pipefail

if [[ $# -lt 1 ]]; then
    echo "Usage: $0 <slug>" >&2
    echo "Example: $0 two-sum" >&2
    exit 1
fi

slug="$1"

if [[ ! "$slug" =~ ^[a-z][a-z0-9-]*$ ]]; then
    echo "Error: slug must start with a lowercase letter and contain only lowercase letters, digits, and dashes." >&2
    exit 1
fi

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "$script_dir/.." && pwd)"
target_dir="$repo_root/$slug"

if [[ -d "$target_dir" ]]; then
    echo "Error: directory '$slug' already exists." >&2
    exit 1
fi

read -r -p "Title (e.g. \"Two Sum\"): " title
read -r -p "LeetCode URL [https://leetcode.com/problems/$slug/]: " url
read -r -p "Difficulty (Easy|Medium|Hard) [Medium]: " difficulty

url="${url:-https://leetcode.com/problems/$slug/}"
difficulty="${difficulty:-Medium}"

if [[ -z "$title" ]]; then
    echo "Error: title cannot be empty." >&2
    exit 1
fi

mkdir -p "$target_dir/src"

cat > "$target_dir/Cargo.toml" <<EOF
[package]
name = "$slug"
version = "0.1.0"
edition = "2021"

[dependencies]
EOF

cat > "$target_dir/src/main.rs" <<'EOF'
pub fn solve() {
    todo!()
}

fn main() {
    solve();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholder() {
        let result = std::panic::catch_unwind(|| solve());
        assert!(result.is_err(), "solve() should panic with todo!() until implemented");
    }
}
EOF

cat > "$target_dir/README.md" <<EOF
# $title

**LeetCode Problem:** [$title]($url)

**Difficulty:** $difficulty

## Problem Statement

_Paste the problem statement here._

## Examples

_Optional: paste examples here._

## Constraints

_Optional: paste constraints here._
EOF

echo ""
echo "Scaffolded: $target_dir"
echo "  - Cargo.toml"
echo "  - src/main.rs     (todo!() stub + placeholder test)"
echo "  - README.md       ($title, $difficulty)"
echo ""
echo "Next steps:"
echo "  1. Fill in the README.md with the problem statement"
echo "  2. Replace todo!() in src/main.rs with your solution"
echo "  3. Add real #[test] cases based on the LeetCode examples"
echo "  4. Run 'cargo test -p $slug' to verify"

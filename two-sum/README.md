# Two Sum

**LeetCode Problem:** [Two Sum](https://leetcode.com/problems/two-sum/)

**Difficulty:** Easy

## Problem Statement

Given a list of integers and a target sum, return the indices of two different
elements whose values add up to the target. Each input has exactly one valid
pair, and the indices may be returned in either order.

## Examples

| Input | Target | Indices |
| --- | --- | --- |
| `[2, 7, 11, 15]` | `9` | `[0, 1]` |
| `[3, 2, 4]` | `6` | `[1, 2]` |
| `[3, 3]` | `6` | `[0, 1]` |

## Constraints

- Use two distinct indices; an element cannot be reused.
- Exactly one pair is guaranteed.

## Solution

[src/main.rs](src/main.rs) uses a one-pass `HashMap` of previously seen values
and their indices. Each value's complement is checked before insertion, so an
element cannot match itself. Complement subtraction is checked to avoid integer
overflow. If no pair exists, the function returns an empty vector.

Expected time complexity is **O(n)**, with **O(n)** extra space.

Run the tests:

```bash
cargo test -p two-sum
```

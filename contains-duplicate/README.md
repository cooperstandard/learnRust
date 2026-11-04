# Contains Duplicate

**LeetCode Problem:** [Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)

**Difficulty:** Easy

## Problem Statement

Given a vector of integers, determine whether any value occurs more than once.
Return `true` if there is a duplicate, or `false` if all values are distinct.

## Examples

| Input | Output |
| --- | --- |
| `[1, 2, 3, 1]` | `true` |
| `[1, 2, 3, 4]` | `false` |
| `[1, 1, 1, 3, 3, 4, 3, 2, 4, 2]` | `true` |

## Constraints

- `1 <= nums.len() <= 100_000`
- `-1_000_000_000 <= nums[i] <= 1_000_000_000`

## Solution

[src/main.rs](src/main.rs) consumes the input with `into_iter()` and uses `any()`
to stop at the first duplicate. `HashSet::insert` returns `false` when a value
is already present, so negating its result detects a duplicate.

Expected time complexity is **O(n)**, with **O(n)** extra space.

Run the tests:

```bash
cargo test -p contains-duplicate
```

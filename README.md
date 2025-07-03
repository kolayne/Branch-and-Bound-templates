# Branch-and-Bound-templates

[![crates.io](https://img.shields.io/crates/v/branch-and-bound.svg)](https://crates.io/crates/branch-and-bound)
[![docs.rs](https://img.shields.io/docsrs/branch-and-bound)](https://docs.rs/branch-and-bound/latest/branch_and_bound/)

A highly generic Branch and Bound / Backtracking library with a flexible API in Rust.

## Description

This library implements generic branch-and-bound and backtracking solver.

Branch-and-bound (and backtracking, which is its special case) is the method of solving an optimization problem by recursively breaking a problem down to subproblems and then solving them. Unlike brute-force, branch-and-bound will discard a subproblem if it discovers that the best potentially obtainable solution to this subproblem is not better than the current best solution (aka incumbent).

To use the library, one shell implement a type that represents a problem (subproblem) and implement the `Subproblem` trait for it.

One can then `solve` an instance of problem using one of the predefined traverse methods (DFS, BFS, BeFS, etc) or use `solve_with_container`, through which custom strategies can be implemented.

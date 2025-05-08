use std::collections::VecDeque;

mod knapsack_common;
use knapsack_common::*;

#[cfg(test)]
mod knapsack_samples;

fn best_candidate(a: KnapsackSubproblem, b: KnapsackSubproblem) -> KnapsackSubproblem {
    if a.collected_val() < b.collected_val() {
        b
    } else {
        a
    }
}

fn solve(problem: KnapsackSubproblem) -> Option<KnapsackSubproblem> {
    let mut queue = VecDeque::<KnapsackSubproblem>::new();
    queue.push_back(problem.clone());

    // Initial problem (empty knapsack with all items left) is the initial incumbent
    let mut incumbent = problem;

    while let Some(subproblem) = queue.pop_front() {
        // Check of the lazy evaluation strategy
        if subproblem.bound() <= incumbent.collected_val() {
            continue;
        }

        if subproblem.capacity_left() == 0 {
            incumbent = best_candidate(incumbent, subproblem);
            continue;
        }

        if subproblem.have_items() {
            let mut child_include = subproblem.clone();
            child_include.include_next();

            let mut child_exclude = subproblem;
            child_exclude.drop_next();

            // Insert with checks of the eager evaluation strategy
            if child_include.bound() > incumbent.collected_val() {
                queue.push_back(child_include);
            }
            if child_exclude.bound() > incumbent.collected_val() {
                queue.push_back(child_exclude);
            }
        } else {
            incumbent = best_candidate(incumbent, subproblem);
        }
    }

    Some(incumbent)
}

fn main() {
    let i = |w, p| Item {
        weight: w,
        price: p,
    };

    // Just an arbitrary example I made up
    let problem = KnapsackSubproblem::new(9, vec![i(6, 5), i(1, 1), i(2, 2), i(4, 4)]);

    let packed = solve(problem).expect("Solution always exists, empty knapsack in the worst case");
    println!("Solved: {:#?}", packed.into_items());
}

use std::collections::VecDeque;

mod knapsack_common;
use knapsack_common::*;

#[cfg(test)]
mod knapsack_samples;

fn best_candidate(a: Option<KnapsackSubproblem>, b: KnapsackSubproblem) -> KnapsackSubproblem {
    match a {
        None => b,
        Some(incumbent) => {
            if incumbent.collected_val() < b.collected_val() {
                b
            } else {
                incumbent
            }
        }
    }
}

fn solve(problem: KnapsackSubproblem) -> Option<KnapsackSubproblem> {
    let mut queue = VecDeque::<KnapsackSubproblem>::new();
    queue.push_back(problem);

    let mut best: Option<KnapsackSubproblem> = None;

    while let Some(subproblem) = queue.pop_front() {
        if subproblem.capacity_left() == 0 {
            best = Some(best_candidate(best, subproblem));
            continue;
        }

        if subproblem.have_items() {
            let mut child_include = subproblem.clone();
            child_include.include_next();
            queue.push_back(child_include);

            let mut child_exclude = subproblem;
            child_exclude.drop_next();
            queue.push_back(child_exclude);
        } else {
            best = Some(best_candidate(best, subproblem));
        }
    }

    best
}

fn main() {
    let i = |w, p| Item {
        weight: w,
        price: p,
    };

    // Just an arbitrary example I made up
    let problem = KnapsackSubproblem::new(9, vec![i(6, 5), i(1, 1), i(2, 2), i(4, 4)]);

    if let Some(packed) = solve(problem) {
        println!("Solved: {:#?}", packed.into_items());
    } else {
        println!("No solution!");
    }
}

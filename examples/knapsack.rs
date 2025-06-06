use branch_and_bound::{Subproblem, SubproblemResolution};

mod knapsack_core;
use knapsack_core::*;

#[cfg(test)]
mod knapsack_samples;

impl Subproblem for KnapsackSubproblem {
    type Score = u64;

    fn branch_or_evaluate(&mut self) -> SubproblemResolution<Self, Self::Score> {
        if self.capacity_left() == 0 {
            return SubproblemResolution::Solved(self.collected_val());
        }

        if self.have_items() {
            let mut child_include = self.clone();
            child_include.include_next();

            let dummy = KnapsackSubproblem::new(0, vec![]);
            let mut child_exclude = std::mem::replace(self, dummy); // Avoid copying: reuse `self`
            child_exclude.drop_next();

            SubproblemResolution::Branched(Box::new([child_include, child_exclude].into_iter()))
        } else {
            SubproblemResolution::Solved(self.collected_val())
        }
    }

    fn bound(&self) -> Self::Score {
        self.bound()
    }
}

fn solve(problem: KnapsackSubproblem) -> Option<KnapsackSubproblem> {
    branch_and_bound::solve(problem, branch_and_bound::TraverseMethod::Greedy)
}

fn main() {
    examples_main();
}

use branch_and_bound::{Subproblem, SubproblemResolution};

mod knapsack_common;
use knapsack_common::*;

impl Subproblem for Knapsack {
    type Score = u32;

    fn branch_or_evaluate(&mut self) -> SubproblemResolution<Self, Self::Score> {
        if self.capacity_left() == 0 {
            return SubproblemResolution::Solved(self.collected_val());
        }

        if self.have_items() {
            let mut child_include = self.clone();
            child_include.include_next();

            let dummy = Knapsack::new(0, vec![]);
            let mut child_exclude = std::mem::replace(self, dummy); // Avoid copying: reuse `self`
            child_exclude.drop_next();

            SubproblemResolution::Branched(Box::new([child_include, child_exclude].into_iter()))
        } else {
            SubproblemResolution::Solved(self.collected_val())
        }
    }

    fn bound(&self) -> Self::Score {
        // If I was to add items greadily (items are ordered by ratio) and perfectly fill
        // the knapsack, that would be the best solution.
        //
        // The heuristic is as follows: I try to use a bit more than
        // the capacity of the knapsack and when that is filled, I claim that
        // that's the best we could possibly get
        // (because that's the best we could possibly get with a slightly larger knapsack).

        let mut val = self.collected_val();
        let mut capacity = self.capacity_left();
        for item in self.future_items() {
            if item.weight < capacity {
                val += item.price;
                capacity -= item.weight;
            } else {
                // Exceeding the capacity with this item
                return val + item.price;
            }
        }
        val
    }
}

fn main() {
    todo!();
}

use branch_and_bound::{Subproblem, SubproblemResolution};

#[derive(Clone, Debug)]
struct Item {
    price: u32,
    weight: u32,
}

#[derive(Default, Clone)]
struct Knapsack {
    /// Value currently acquired by the knapsack
    val: u32,
    /// Capacity left in the knapsack
    capacity: u32,
    /// Items to try to put (sorted by the `.price/.weight` ratio in ascending order)
    items_left: Vec<Item>,
    /// Items in the knapsack (only needed to restore the answer)
    items_in: Vec<Item>,
}

impl Knapsack {
    pub fn drop_too_heavy(&mut self) {
        while let Some(item) = self.items_left.last() {
            if item.weight > self.capacity {
                self.items_left.pop();
            } else {
                break;
            }
        }
    }

    pub fn drop_last(&mut self) {
        debug_assert!(!self.items_left.is_empty());
        self.items_left.pop();
    }

    pub fn include_last(&mut self) {
        debug_assert!(!self.items_left.is_empty());
        let item = self.items_left.last().unwrap();
        self.val += item.price;
        self.capacity -= item.weight;
        self.items_in.push(self.items_left.pop().unwrap());
    }

    pub fn have_items(&self) -> bool {
        !self.items_left.is_empty()
    }
}

impl Subproblem for Knapsack {
    type Score = u32;

    fn branch_or_evaluate(&mut self) -> SubproblemResolution<Self, Self::Score> {
        if self.capacity == 0 {
            return SubproblemResolution::Solved(self.val);
        }

        self.drop_too_heavy();

        if self.have_items() {
            let mut child_include = self.clone();
            child_include.include_last();

            let mut child_exclude: Knapsack = Default::default();
            std::mem::swap(self, &mut child_exclude); // Avoid copying: swap with `self` instead
            child_exclude.drop_last();

            SubproblemResolution::Branched(Box::new([child_include, child_exclude].into_iter()))
        } else {
            SubproblemResolution::Solved(self.val)
        }
    }

    fn bound(&self) -> Self::Score {
        // If I add items greadily and arrive exactly at the zero capacity,
        // that's the best solution.
        // The heuristic is as follows: I try to use a bit more than
        // the capacity of the knapsack and when that is filled, I claim that
        // that's the best we could possibly get
        // (because that's the best we could possibly get with a larger knapsack).

        let mut val = self.val;
        let mut capacity = self.capacity;
        for item in self.items_left.iter().rev() {
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Item {
    pub price: u64,
    pub weight: u64,
}

#[derive(Clone)]
pub struct KnapsackSubproblem {
    /// Value currently acquired by the knapsack
    val: u64,
    /// Capacity left in the knapsack
    capacity_left: u64,
    /// Items to try to put (sorted by the `.price/.weight` ratio in ascending order).
    ///
    /// Internal invariant: if not empty, the last item's weight must be less than capacity
    /// (otherwise gets popped). That is to make sure that all methods (`.drop_next()`,
    /// `.include_next()`, `.future_items()`, `.have_items()`) are consistent.
    items_left: Vec<Item>,
    /// Items in the knapsack (only needed to restore the answer)
    items_in: Vec<Item>,
}

impl KnapsackSubproblem {
    /// Creates a knapsack with `items` to be inserted.
    ///
    /// Items will be sorted according to their `.price / .weight` ratio,
    /// better items to be included earlier.
    ///
    /// Note: items with weight exceeding capacity are never kept in `KnapsackProblem`.
    pub fn new(capacity: u64, items: Vec<Item>) -> Self {
        let mut res = Self {
            val: 0,
            capacity_left: capacity,
            items_left: items,
            items_in: vec![],
        };
        // Sort by ratio in _ascending_ order (last is best)
        res.items_left.sort_by(|item1, item2| {
            (item1.price * item2.weight).cmp(&(item2.price * item1.weight))
        });
        res.pop_too_heavy();
        res
    }

    fn pop_too_heavy(&mut self) {
        while let Some(item) = self.items_left.last() {
            if item.weight > self.capacity_left {
                self.items_left.pop();
            } else {
                break;
            }
        }
    }

    /// Drops the next item that could have been included.
    pub fn drop_next(&mut self) {
        debug_assert!(!self.items_left.is_empty());
        self.items_left.pop();
        self.pop_too_heavy();
    }

    /// Includes the next item to be included. Drops items
    /// with weight exceeding the new capacity left.
    pub fn include_next(&mut self) {
        debug_assert!(!self.items_left.is_empty());
        let item = self.items_left.last().unwrap();
        self.val += item.price;
        self.capacity_left -= item.weight;
        self.items_in.push(self.items_left.pop().unwrap());
        self.pop_too_heavy();
    }

    pub fn have_items(&self) -> bool {
        !self.items_left.is_empty()
    }

    pub fn capacity_left(&self) -> u64 {
        self.capacity_left
    }

    pub fn collected_val(&self) -> u64 {
        self.val
    }

    /// Future items to be included, in the order they may be included
    /// (i.e., descending order of the `.price / .weight` ratio).
    ///
    /// Note: items with weight exceeding capacity are never kept in `KnapsackProblem`.
    pub fn future_items(&self) -> impl Iterator<Item = &Item> {
        self.items_left
            .iter()
            .rev()
            .filter(|item| item.weight <= self.capacity_left)
    }

    /// Converts a `KnapsackSubproblem` into the set of items that are in the knapsack.
    pub fn into_items(self) -> Vec<Item> {
        self.items_in
    }
}

#[cfg(test)]
mod test {
    use super::super::knapsack_samples as samples;
    use super::*;
    use std::collections::HashSet;

    fn run_test(capacity: u64, items: Vec<Item>, expected: HashSet<Item>) {
        let problem = KnapsackSubproblem::new(capacity, items);
        let solution = super::super::solve(problem).unwrap().into_items();
        let solution = HashSet::<Item>::from_iter(solution.into_iter());
        assert_eq!(solution, expected);
    }

    #[test]
    fn fsu_test_1() {
        run_test(samples::capacity1, samples::items1(), samples::expected1());
    }

    #[test]
    fn fsu_test_2() {
        run_test(samples::capacity2, samples::items2(), samples::expected2());
    }

    #[test]
    fn fsu_test_3() {
        run_test(samples::capacity3, samples::items3(), samples::expected3());
    }

    #[test]
    fn fsu_test_4() {
        run_test(samples::capacity4, samples::items4(), samples::expected4());
    }

    #[test]
    fn fsu_test_5() {
        run_test(samples::capacity5, samples::items5(), samples::expected5());
    }

    #[test]
    fn fsu_test_6() {
        run_test(samples::capacity6, samples::items6(), samples::expected6());
    }

    #[test]
    fn fsu_test_7() {
        run_test(samples::capacity7, samples::items7(), samples::expected7());
    }

    #[test]
    fn fsu_test_8() {
        run_test(samples::capacity8, samples::items8(), samples::expected8());
    }
}

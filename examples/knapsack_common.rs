#[derive(Clone, Debug)]
pub struct Item {
    pub price: u32,
    pub weight: u32,
}

#[derive(Clone)]
pub struct KnapsackSubproblem {
    /// Value currently acquired by the knapsack
    val: u32,
    /// Capacity left in the knapsack
    capacity_left: u32,
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
    pub fn new(capacity: u32, items: Vec<Item>) -> Self {
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

    pub fn capacity_left(&self) -> u32 {
        self.capacity_left
    }

    pub fn collected_val(&self) -> u32 {
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

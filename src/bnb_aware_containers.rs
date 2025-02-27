use crate::Subproblem;

/// A container for subproblem objects, which is used
/// to store unvisited nodes of the subproblem tree.
///
/// A container provides an interface to push and pop
/// items and:
/// 1. Defines order in which elements will be popped;
/// 2. May implement additional features, such as early stopping,
///    deciding not to push/return some elements based on the value
///    of the incumbent, etc.
pub trait BnbAwareContainer<S: Subproblem> {
    /// Add `item` to the container.
    /// `score` is the score of the current incumbent (if any). The
    /// container may decide not to add an item if it's known to be
    /// worse than the incumbent ("eager" evaluation strategy).
    fn push_with_incumbent(&mut self, item: S, score: Option<&S::Score>);

    /// Get an item from the container.
    /// `score` is the score of the current incumbent (if any). The
    /// container may decide to skip items that are known to be
    /// worse than the incumbent ("lazy" evaluation strategy).
    ///
    /// Returns `None` iff the container is exhausted.
    fn pop_with_incumbent(&mut self, score: Option<&S::Score>) -> Option<S>;
}

/// This implementation for `Vec` is an implementation of the extra-eager strategy:
/// it checks against the incumbent both when pushing and when popping.
/// I suppose, it's not efficient!
/// TODO: analyze this on examples and provide more flexible options.
impl<S: Subproblem> BnbAwareContainer<S> for Vec<S> {
    fn push_with_incumbent(&mut self, item: S, score: Option<&S::Score>) {
        if score.is_none() || score.unwrap() < &item.bound() {
            self.push(item)
        }
    }

    fn pop_with_incumbent(&mut self, score: Option<&S::Score>) -> Option<S> {
        while let Some(item) = self.pop() {
            if score.is_none() || score.unwrap() < &item.bound() {
                return Some(item);
            }
        }
        None
    }
}

/// This implementation for `VecDeque` is an implementation of the extra-eager
/// strategy: it checks against the incumbent both when pushing and when
/// popping.
/// I suppose, it's not efficient!
/// TODO: analyze this on examples and provide more flexible options.
impl<S: Subproblem> BnbAwareContainer<S> for std::collections::VecDeque<S> {
    fn push_with_incumbent(&mut self, item: S, score: Option<&S::Score>) {
        if score.is_none() || score.unwrap() < &item.bound() {
            self.push_front(item)
        }
    }

    fn pop_with_incumbent(&mut self, score: Option<&S::Score>) -> Option<S> {
        while let Some(item) = self.pop_back() {
            if score.is_none() || score.unwrap() < &item.bound() {
                return Some(item);
            }
        }
        None
    }
}

/// This implementation for `VecDeque` is an implementation of the extra-eager
/// strategy: it checks against the incumbent both when pushing and when
/// popping.
/// We can't remove the lazy evaluation part here (because then BeFS would
/// make no sense: we want it to terminate early) but the eager part may
/// be removed, which might make it more efficient.
/// TODO: analyze this on examples and provide more flexible options.
impl<S: Subproblem, Cmp: compare::Compare<S>> BnbAwareContainer<S>
    for binary_heap_plus::BinaryHeap<S, Cmp>
{
    fn push_with_incumbent(&mut self, item: S, score: Option<&<S as Subproblem>::Score>) {
        if score.is_none() || score.unwrap() < &item.bound() {
            self.push(item);
        }
    }

    fn pop_with_incumbent(&mut self, score: Option<&<S as Subproblem>::Score>) -> Option<S> {
        // If the first (i.e., best) item is definitely worse than the current best solution,
        // there's no point in looking any further: the rest of candidates are worse anyway
        if let Some(item) = self.pop() {
            if score.is_none() || score.unwrap() < &item.bound() {
                return Some(item);
            }
        }

        None
    }
}

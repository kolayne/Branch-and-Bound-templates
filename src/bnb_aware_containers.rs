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

/// Wrapper around `binary_heap_plus::BinaryHeap`.
/// Used for best-first search and custom-order search.
pub(super) struct BinaryHeapExt<Node, Cmp> {
    /// The node container
    pub heap: binary_heap_plus::BinaryHeap<Node, Cmp>,
    /// If `true`, the heap behaves as if in best-first search:
    /// if the candidate `.bound()` is less than or equal to
    /// the incumbent's objective score, no more elements will
    /// be popped, so the algorithm will terminate early.
    pub stop_early: bool,
}

// TODO: it  seems like it makes more sense to also create (private)
// wrapper types for `Vec` and `VecDeque` and implement `BnbAwareContainer`
// for them rather than the standard containers. I see two reasons for that:
//
// 1. This would provide better encapsulation: I see the implementations
//    of standard search orders as a private implementation detail, however,
//    a user can now call `solve_with_container` on a vector and it will
//    work according to an algorithm that we internally implement.
//
// 2. This way, it would take less effort for a lazy user to customize
//    an algorithm: they could just implement `BnbAwareContainer` on a
//    standard type like `Vec` and have it work, without having to create
//    a wrapper type (currently, that's not possible because
//    `BnbAwareContainer`) is already implemented for `Vec`.

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

/// This implementation for `BinaryHeapExt` is an implementation of the extra-eager
/// strategy: it checks against the incumbent both when pushing and when
/// popping.
/// We can't remove the lazy evaluation part here (because then BeFS would
/// make no sense: we want it to terminate early) but the eager part may
/// be removed, which might make it more efficient.
/// TODO: analyze this on examples and provide more flexible options.
impl<S: Subproblem, Cmp: compare::Compare<S>> BnbAwareContainer<S> for BinaryHeapExt<S, Cmp> {
    fn push_with_incumbent(&mut self, item: S, score: Option<&<S as Subproblem>::Score>) {
        if score.is_none() || score.unwrap() < &item.bound() {
            self.heap.push(item);
        }
    }

    fn pop_with_incumbent(&mut self, score: Option<&<S as Subproblem>::Score>) -> Option<S> {
        // If the first (i.e., best) item is definitely worse than the current best solution,
        // there's no point in looking any further: the rest of candidates are worse anyway
        while let Some(item) = self.heap.pop() {
            if score.is_none() || score.unwrap() < &item.bound() {
                return Some(item);
            }

            // If this candidate is not good enough and `self.stop_early`,
            // assuming no candidate will be good enough.
            if self.stop_early {
                break;
            }
        }

        None
    }
}

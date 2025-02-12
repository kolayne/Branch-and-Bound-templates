use crate::{Subproblem, SubproblemResolution};

/// `OrderedCandidate` encapsulates a `Node` (which has to be `Subproblem`)
/// and defines an order on it.
pub trait OrderedCandidate: Ord + Subproblem {
    type Node: Subproblem;

    /// Create an `OrderedCandidate`.
    /// This method should be used for the root node.
    fn new(root: Self::Node) -> Self;
    /// Peeks at the encapsulated node
    fn node(&self) -> &Self::Node;
    /// Consumes the candidate and returns the node
    fn into_node(self) -> Self::Node;
}

/// `BoundOrderedCandidate` implements `{Partial,}Eq` and `{Partial,}Ord`
/// based on the value of `node.bound()`.
///
/// Note: two `BoundOrderedCandidate`s wrapping different nodes with the boundary
/// will compare equal!
pub(crate) struct BoundOrderedCandidate<Node: Subproblem<Score = Score>, Score: Ord> {
    node: Node,
    bound: Score,
}

/// `DepthOrderedCandidate` implements `{Partial,}Eq` and `{Partial,}Ord`
/// based on the depth in the tree (higher in the tree is less; root is the lowest).
///
/// Note: two `DepthOrderedCandidate`s wrapping different nodes on the same level
/// of the tree will compare equal!
pub(crate) struct DepthOrderedCandidate<Node: Subproblem<Score = Score>, Score: Ord> {
    node: Node,
    depth: u64,
}

impl<Score: Ord, Node: Subproblem<Score = Score>> PartialEq for BoundOrderedCandidate<Node, Score> {
    fn eq(&self, other: &Self) -> bool {
        self.bound == other.bound
    }
}

impl<Score: Ord, Node: Subproblem<Score = Score>> Eq for BoundOrderedCandidate<Node, Score> {}

impl<Score: Ord, Node: Subproblem<Score = Score>> PartialOrd
    for BoundOrderedCandidate<Node, Score>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Score: Ord, Node: Subproblem<Score = Score>> Ord for BoundOrderedCandidate<Node, Score> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bound.cmp(&other.bound)
    }
}

impl<Score, Node> Subproblem for BoundOrderedCandidate<Node, Score>
where
    Score: Ord,
    Node: Subproblem<Score = Score> + 'static,
{
    type Score = Score;

    fn branch_or_evaluate(&self) -> crate::SubproblemResolution<Self, Self::Score> {
        use SubproblemResolution::{Branched, Solved};

        match self.node.branch_or_evaluate() {
            Solved(score) => Solved(score),
            // Just wrap all `Node`s into `Self`. Won't compile without clojure (why??)
            Branched(subproblems) => Branched(Box::new(subproblems.map(|node| Self::new(node)))),
        }
    }

    fn bound(&self) -> Self::Score {
        self.node.bound()
    }
}

impl<Score, Node> OrderedCandidate for BoundOrderedCandidate<Node, Score>
where
    Score: Ord,
    Node: Subproblem<Score = Score> + 'static,
{
    type Node = Node;

    fn new(root: Self::Node) -> Self {
        Self {
            bound: root.bound(),
            node: root,
        }
    }

    fn node(&self) -> &Self::Node {
        &self.node
    }

    fn into_node(self) -> Self::Node {
        self.node
    }
}

impl<Score: Ord, Node: Subproblem<Score = Score>> PartialEq for DepthOrderedCandidate<Node, Score> {
    fn eq(&self, other: &Self) -> bool {
        self.depth == other.depth
    }
}

impl<Score: Ord, Node: Subproblem<Score = Score>> Eq for DepthOrderedCandidate<Node, Score> {}

impl<Score: Ord, Node: Subproblem<Score = Score>> PartialOrd
    for DepthOrderedCandidate<Node, Score>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Score: Ord, Node: Subproblem<Score = Score>> Ord for DepthOrderedCandidate<Node, Score> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.depth.cmp(&other.depth)
    }
}

impl<Score, Node> Subproblem for DepthOrderedCandidate<Node, Score>
where
    Score: Ord,
    Node: Subproblem<Score = Score> + 'static,
{
    type Score = Score;

    fn branch_or_evaluate(&self) -> SubproblemResolution<Self, Self::Score> {
        use SubproblemResolution::{Branched, Solved};

        let depth: u64 = self.depth;
        match self.node.branch_or_evaluate() {
            Solved(score) => Solved(score),
            Branched(subproblems) => Branched(Box::new(subproblems.map(move |subnode| Self {
                node: subnode,
                depth: depth + 1,
            }))),
        }
    }

    fn bound(&self) -> Self::Score {
        self.node.bound()
    }
}

impl<Score, Node> OrderedCandidate for DepthOrderedCandidate<Node, Score>
where
    Score: Ord,
    Node: Subproblem<Score = Score> + 'static,
{
    type Node = Node;

    fn new(root: Self::Node) -> Self {
        Self {
            node: root,
            depth: 0,
        }
    }

    fn node(&self) -> &Self::Node {
        &self.node
    }

    fn into_node(self) -> Self::Node {
        self.node
    }
}

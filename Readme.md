# PQGrams
by Cathal Garvey, Copyright 2017, Licensed under AGPLv3 or later.

## About
PQ-Grams are a method for efficiently evaluating tree structure/content similarity,
for tree structures that can be abstracted as nested (Label, Children) pairs.
Given this premise, a single PQ-Gram is the previous-P ancestor labels (including
the present node), and the next-Q child labels. A PQ-Gram profile is the set of
all PQ-Grams in a tree, including "Filler" nodes that fill in the left and right
of each set of children, and the top of the tree for ancestors.

These PQ-Grams can then be used similarly to n-Grams or shingles from NLP, to
evaluate similarity between trees by set-union and set-difference metrics.
The original usage performed a set-difference-like operation to calculate
approximate tree edit distance.

PQ-Grams are not implemented widely, which is a shame. After reviewing and hacking
on [PyGram][pygram] a bit, I decided I'd like to implement this in Rust for speed and
portability. Ironically, the port of PyGram to Python3 allowed me to add efficiency
features that I haven't yet implemented in Rust, such as LRU caching, but I hope
to get around to this later.

[pygram]: https://github.com/TylerGoeringer/PyGram

## Next
I'd like to build Rust/Python bindings to this that accept LXML trees, to evaluate
the possibility of using PQGrams for structure comparisons on HTML fragments.
There is some risk that the PQGram profile generation process over large documents
could get costly, so I may have to revisit the implementation and consider adding
iterator interfaces; we'll see if that's a significant problem.

## Usage
A generic Tree is provided that can be used with the builder-pattern to build
trees quickly. Look at the tests in `lib.rs` for an example of use.

The intended use, though, is to implement the `LabelledTree` trait for your
tree-type, choosing a rational and comparable Label for each node, and to pass
that implementation to this code. For example, a HTML tree might implement
`LabelledTree` by extracting HTML tags and casting them to `u8` (because the
set of HTML tags is small and `u8` would be space-conserving over strings), or
a JSON-walking tree might extract object-keys as labels, and give non-container
values deterministic value-based labels.

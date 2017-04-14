mod bdeque;
mod pqgrams;
mod default_tree;
pub use default_tree::Tree;
pub use pqgrams::{pqgram_distance, ValidGramElement, LabelledTree, PQGram, Node, pqgram_profile, flatten_profile, pqgram_distance_with_fn};


#[cfg(test)]
mod tests {
    use super::default_tree::Tree;
    use super::{pqgram_distance, pqgram_profile, flatten_profile};

    // Utility function
    fn f64_round_2dp(n: f64) -> f64 {
        (n * 100.).round() / 100.
    }

    fn build_known_tree_1() -> Tree<String> {
        Tree::new_str("a")
                .add_node(Tree::new_str("a")
                            .add_node(Tree::new_str("e"))
                            .add_node(Tree::new_str("b")))
                .add_node(Tree::new_str("b"))
                .add_node(Tree::new_str("c"))
    }

    fn known_profile_1() -> Vec<Vec<String>> {
        vec![
            vec!["*", "a", "*", "*", "a"],
            vec!["*", "a", "*", "a", "b"],
            vec!["*", "a", "a", "b", "c"],
            vec!["*", "a", "b", "c", "*"],
            vec!["*", "a", "c", "*", "*"],
            vec!["a", "a", "*", "*", "e"],
            vec!["a", "a", "*", "e", "b"],
            vec!["a", "a", "b", "*", "*"],
            vec!["a", "a", "e", "b", "*"],
            vec!["a", "b", "*", "*", "*"],
            vec!["a", "b", "*", "*", "*"],
            vec!["a", "c", "*", "*", "*"],
            vec!["a", "e", "*", "*", "*"]
            ].iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect()
    }

    fn build_known_tree_2() -> Tree<String> {
        Tree::new_str("a")
                .add_node(Tree::new_str("a")
                            .add_node(Tree::new_str("e"))
                            .add_node(Tree::new_str("b")))
                .add_node(Tree::new_str("b"))
                .add_node(Tree::new_str("x"))
    }

    #[test]
    fn test_pqgram_profile() {
        assert_eq!(known_profile_1(),
                   flatten_profile(&pqgram_profile(build_known_tree_1(), 2, 3, true), "*".to_string()))
    }

    #[test]
    fn test_pqgram_distance() {
        let tree_1 = build_known_tree_1();
        let tree_2 = tree_1.clone();
        let tree_3 = build_known_tree_2();
        let prof1 = pqgram_profile(tree_1, 2, 3, false);
        let prof2 = pqgram_profile(tree_2, 2, 3, false);
        let prof3 = pqgram_profile(tree_3, 2, 3, false);
        let dist12 = pqgram_distance::<String, Tree<String>>(&prof1, &prof2, None);
        let dist13 = pqgram_distance::<String, Tree<String>>(&prof1, &prof3, None);
        assert_eq!(f64_round_2dp(dist12), 0.);    // Same
        assert_eq!(f64_round_2dp(dist13), 0.31);  // Differ by 0.31
    }

    #[test]
    fn test_pqgram_distance_when_sorted() {
        let tree_1 = build_known_tree_1();
        let tree_2 = tree_1.clone();
        let tree_3 = build_known_tree_2();
        let prof1 = pqgram_profile(tree_1, 2, 3, true);
        let prof2 = pqgram_profile(tree_2, 2, 3, true);
        let prof3 = pqgram_profile(tree_3, 2, 3, true);
        let dist12 = pqgram_distance::<String, Tree<String>>(&prof1, &prof2, None);
        let dist13 = pqgram_distance::<String, Tree<String>>(&prof1, &prof3, None);
        assert_eq!(f64_round_2dp(dist12), 0.);    // Same
        assert_eq!(f64_round_2dp(dist13), 0.31);  // Differ by 0.31
    }
}

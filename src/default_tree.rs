use pqgrams::{Node, ValidGramElement, LabelledTree};

#[derive(Debug, Clone)]
pub struct Tree<T: ValidGramElement> {
    pub label: T,
    pub children: Box<Vec<Tree<T>>>,
}

impl ValidGramElement for String {}
impl ValidGramElement for i8 {}
impl ValidGramElement for u8 {}
impl ValidGramElement for i16 {}
impl ValidGramElement for u16 {}
impl ValidGramElement for i32 {}
impl ValidGramElement for u32 {}
impl ValidGramElement for i64 {}
impl ValidGramElement for u64 {}


impl<T: ValidGramElement> LabelledTree<T> for Tree<T> {
    fn label(&self) -> Node<T> {
        return Node::Label(self.label.to_owned())
    }
    fn children(&self) -> Vec<&Tree<T>> {
        self.children.iter().map(|c| c as &Tree<T>).collect()
    }
}

impl<T: ValidGramElement> Tree<T> {
    pub fn new(label: T) -> Tree<T> {
        Tree{label: label, children: Box::new(vec![])}
    }

    // TODO: Add random tree feature, assists testing.

    /// Builder-pattern tree building helper. This returns self,
    /// so you can use it with Tree::new() to build nested trees
    /// ergonomically.
    pub fn add_node(mut self, child: Tree<T>) -> Tree<T> {
        self.children.push(child);
        self
    }
}

impl Tree<String> {
    /// Minor convenience to save the use of to_string on everything
    /// when building trees.
    pub fn new_str(label: &str) -> Tree<String> {
        Tree{label: label.to_string(), children: Box::new(vec![])}
    }
}

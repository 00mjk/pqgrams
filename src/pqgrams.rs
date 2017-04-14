use std::fmt;
use std::cmp;
use std::default;
use bdeque::BDeque;


/// ValidGramElement contains all the traits required of a PQGram member element.
/// This approach DRYs up the code and keeps it tidy, but requires implementors
/// to add an empty `impl ValidGramElement for T {}` block before impl'ing
/// LabelledTree for the Tree container.
pub trait ValidGramElement: fmt::Debug + cmp::Ord + Clone + default::Default {}

/// A single tree node that may form part of a 'gram. PQGrams include
/// filler labels for absent nodes in either dimension (usual notation is '*')
/// so this enum lets the PQgram profile contain either while allowing literal
/// '*' as a label.
#[derive(Copy,Clone,Debug,PartialEq,PartialOrd,Eq,Ord)]
pub enum Node<L: ValidGramElement> {
    Filler,
    Label(L),
}

/// A single 'gram in a profile.
#[derive(Clone,Debug,PartialEq,PartialOrd,Eq,Ord)]
pub struct PQGram<L: ValidGramElement> {
    ancestors: Vec<Node<L>>,
    siblings: Vec<Node<L>>,
}

impl<L: ValidGramElement> PQGram<L> {
    /// Build a PQGram from ancestors and siblings (ps & qs)
    pub fn new(ps: Vec<Node<L>>, qs: Vec<Node<L>>) -> PQGram<L> {
        PQGram{ancestors: ps, siblings: qs}
    }

    /// Concatenate ancestor and sibling nodes, replacing "Filler" nodes
    /// with the clones of filler_as. By convention string filler nodes
    /// might be represented "*" (as in the paper).
    pub fn concat(&self, filler_as: L) -> Vec<L> {
        let mut bits: Vec<L> = Vec::new();  // TODO; sized
        for a in self.ancestors.iter().chain(self.siblings.iter()) {
            bits.push(match a {
                &Node::Label(ref v) => v.clone(),
                &Node::Filler => filler_as.clone(),
            })
        };
        bits
    }
}

/// Implement this for a tree to let it be PQGrammed.
pub trait LabelledTree<L: ValidGramElement> {
    fn label(&self) -> Node<L>;
    fn children(&self) -> Vec<&Self>;
}

fn _profile_subtree<L, T>(subtree: &T, p: usize, q: usize, ancestors: &mut BDeque<Node<L>>) -> Vec<PQGram<L>>
    where L: ValidGramElement, T: LabelledTree<L>
{
    ancestors.push_back(subtree.label());
    let mut siblings = BDeque::<Node<L>>::new(q);
    siblings.fill_with(Node::Filler);
    let mut pqgrams = Vec::<PQGram<L>>::new();
    if subtree.children().iter().count() == 0 {
        pqgrams.push(PQGram::new(ancestors.copy_state(), siblings.copy_state()));
    } else {
        for child in subtree.children() {
            siblings.push_back(child.label());
            pqgrams.push(PQGram::new(ancestors.copy_state(), siblings.copy_state()));
            for grandchild in _profile_subtree(child, p, q, &mut ancestors.clone()) {
                pqgrams.push(grandchild)
            }
        }
        for _ in 0..q-1 {
            siblings.push_back(Node::Filler);
            pqgrams.push(PQGram::new(ancestors.copy_state(), siblings.copy_state()))
        }
    }
    pqgrams
}

/// Build a PQGram vector profile
pub fn pqgram_profile<L, T>(tree: T, p: usize, q: usize, sort: bool) -> Vec<PQGram<L>>
    where L: ValidGramElement, T: LabelledTree<L>
{
    let mut ancestors = BDeque::<Node<L>>::new(p);
    ancestors.fill_with(Node::Filler);
    let mut prof = _profile_subtree(&tree, p, q, &mut ancestors);
    if sort { prof.sort() }
    prof
}

/// PQGrams are nested structures of ancestors and siblings, but their intended use
/// is usually as flat vectors of constant length. This converts all PQGram elements
/// in a profile into flat vectors.
pub fn flatten_profile<L: ValidGramElement>(profile: &Vec<PQGram<L>>, filler_as: L) -> Vec<Vec<L>> {
    profile.into_iter()
           .map(|gram| gram.concat(filler_as.clone()))
           .collect()
}

/// Expects that the pqgram profiles be sorted. distance_function should return how *close* two grams are,
/// as a float between 0 and 1.
pub fn pqgram_profile_intersection<L, T>(left: &Vec<PQGram<L>>, right: &Vec<PQGram<L>>, alt_filler_value: Option<L>, distance_function: Box<Fn(&PQGram<L>, &PQGram<L>, L)->(f64, cmp::Ordering)>) -> f64
    where L: ValidGramElement, T: LabelledTree<L>
{
    let mut intersection: f64 = 0.;
    let mut i: usize = 0;
    let mut j: usize = 0;
    let maxi = left.len();
    let maxj = right.len();
    let filler = if let Some(l) = alt_filler_value { l } else { L::default() };
    while i < maxi && j < maxj {
        let ref ig = left[i];
        let ref jg = right[j];
        let (distance, order) = distance_function(&ig, &jg, filler.clone());
        intersection += distance;
        match order {
            cmp::Ordering::Equal => {
                i += 1;
                j += 1;
            },
            cmp::Ordering::Less => i += 1,
            cmp::Ordering::Greater => j += 1,
        }
    }
    intersection
}

/// This is the default gram edit distance function. It simply concatenates ancestor + sibling
/// vecs for each PQGram, then returns (1, Equal) if they are identical, and (0, Less || Greater)
/// if they are different. There are no intermediate values. This logic is borrowed from PyGram,
/// and more meaningful results might be possible with more accurate measures of gram-edit distance.
pub fn default_gram_edit_distance<'a, L: 'a>(left: &PQGram<L>, right: &PQGram<L>, filler_value: L) -> (f64, cmp::Ordering)
    where L: ValidGramElement
{
    let iter_compare = left.concat(filler_value.clone())
                           .into_iter()
                           .zip(right.concat(filler_value.clone()).into_iter())
                           .map(|(l, r)| l.partial_cmp(&r)
                                        .expect(format!("Ordering not possible for l, r: {:?}, {:?}", l, r).as_ref()));
    for ordering in iter_compare {
        match ordering {
            cmp::Ordering::Equal => continue,
            _ => return (0., ordering)
        }
    };
    (1., cmp::Ordering::Equal)
}

/// Given two sorted PQGram profiles, return a float value representing their distance, using
/// the provided distance function to provide a 0..1 measure of similarity between PQGrams.
/// If alt_filler_value is None, then the Default for type L is used to fill in Node::Filler
/// elements in the PQGrams before ordering. When the Default for L is a value that also occurs
/// in a valid tree (often the case!), you should provide an Value here that does not occur
/// in the tree.
pub fn pqgram_distance_with_fn<L: 'static, T>(left: &Vec<PQGram<L>>, right: &Vec<PQGram<L>>, alt_filler_value: Option<L>, distance_function: Box<Fn(&PQGram<L>, &PQGram<L>, L)->(f64, cmp::Ordering)>) -> f64
    where L: ValidGramElement, T: LabelledTree<L>
{
    let union = (left.len() + right.len()) as f64;  // TODO: this is copied from the Python, but surely it should be the length of the set-union?
    let intersection: f64 = pqgram_profile_intersection::<L,T>(left, right, alt_filler_value, distance_function);
    return 1. - 2. * (intersection / union)
}

/// Calculates PQGram distance between two profiles, using the default_gram_edit_distance function.
/// All notes for pqgram_distance_with_fn apply here, particularly with respect to alt_filler_value!
pub fn pqgram_distance<L: 'static, T>(left: &Vec<PQGram<L>>, right: &Vec<PQGram<L>>, alt_filler_value: Option<L>) -> f64
    where L: ValidGramElement, T: LabelledTree<L>
{
    pqgram_distance_with_fn::<L,T>(left, right, alt_filler_value, Box::new(default_gram_edit_distance))
}

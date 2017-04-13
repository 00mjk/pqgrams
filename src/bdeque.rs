use std::collections::vec_deque;

/// Bounded Deque - behaves like Python's deque with a specified maxlen
#[derive(Clone)]
pub struct BDeque<T: Clone> {
    maxlen: usize,
    state: vec_deque::VecDeque<T>,
}

impl<T: Clone> BDeque<T> {
    pub fn new(maxlen: usize) -> BDeque<T> {
        let vd = vec_deque::VecDeque::with_capacity(maxlen);
        BDeque{maxlen: maxlen, state: vd}
    }

    pub fn push_back(&mut self, item: T) -> Option<T> {
        let i = if self.state.len() == self.maxlen {
            self.state.pop_front()
        } else {
            None
        };
        self.state.push_back(item);
        return i
    }

    pub fn fill_with(&mut self, item: T)
    {
        for _ in 0..self.maxlen {
            self.push_back(item.clone());
        }
    }

    pub fn copy_state(&self) -> Vec<T> {
        let mut v: Vec<T> = Vec::with_capacity(self.maxlen);
        for i in self.state.iter() {
            v.push(i.clone())
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::BDeque;
    #[test]
    fn test_bdeque() {
        let mut bd = BDeque::new(3);
        assert_eq!(bd.copy_state(), vec![]);
        bd.push_back(1);
        assert_eq!(bd.copy_state(), vec![1]);
        bd.push_back(2);
        assert_eq!(bd.copy_state(), vec![1,2]);
        bd.push_back(3);
        assert_eq!(bd.copy_state(), vec![1,2,3]);
        let one = bd.push_back(4);
        assert_eq!(bd.copy_state(), vec![2,3,4]);
        assert_eq!(one, Some(1));
        bd.fill_with(0);
        assert_eq!(bd.copy_state(), vec![0,0,0]);
    }
}

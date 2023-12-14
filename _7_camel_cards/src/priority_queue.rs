use log::debug;
use std::collections::LinkedList;

#[derive(Debug)]
pub struct Pq<T: Ord + Eq + std::fmt::Debug> {
    data: Vec<LinkedList<T>>,
}

impl<T: Ord + Eq + std::fmt::Debug> Pq<T> {
    pub fn new(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(LinkedList::new());
        }
        Self { data: v }
    }

    fn left_idx(idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_idx(idx: usize) -> usize {
        idx * 2 + 2
    }

    fn sign_parent_idx(idx: usize) -> i32 {
        let i = idx as i32;
        (i - 1) / 2
    }

    fn parent_idx(idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left(&self, idx: usize) -> Option<&LinkedList<T>> {
        self.data.get(Self::left_idx(idx))
    }

    fn left_mut(&mut self, idx: usize) -> &mut LinkedList<T> {
        self.data
            .get_mut(Self::left_idx(idx))
            .expect("mut left node")
    }

    fn right(&self, idx: usize) -> Option<&LinkedList<T>> {
        self.data.get(Self::right_idx(idx))
    }

    fn right_mut(&mut self, idx: usize) -> &mut LinkedList<T> {
        self.data
            .get_mut(Self::right_idx(idx))
            .expect("mut right node")
    }

    pub fn insert(&mut self, item: T) {
        let mut idx = 0;
        loop {
            let ll = self.data.get_mut(idx).expect("can't get ll in insert");
            if ll.is_empty() {
                ll.push_back(item);
                break;
            }

            let el = ll.front().expect("can't get first elem of ll");
            if item == *el {
                ll.push_back(item);
                break;
            }

            if self.left(idx).unwrap().is_empty() {
                idx = Self::left_idx(idx);
            } else if self.right(idx).unwrap().is_empty() {
                idx = Self::right_idx(idx);
            } else {
                idx = Self::left_idx(idx);
            }
        }

        self.shift_up(idx);
        debug!("after insert {:?}", self);
    }

    fn shift_up(&mut self, mut idx: usize) {
        while idx != 0 {
            let el1 = self
                .data
                .get(idx)
                .expect("can't get el1 shift_up")
                .front()
                .expect("can't get front");
            let el2 = self
                .data
                .get(Self::parent_idx(idx))
                .expect("can't get el2 shift_up")
                .front()
                .expect("can't get front");
            if el1 < el2 {
                self.data.swap(idx, Self::parent_idx(idx));
            }
            idx = Self::parent_idx(idx);
        }
    }

    pub fn get(&mut self) -> Option<LinkedList<T>> {
        if self.data[0].is_empty() {
            return None;
        }

        let mut idx = 0;
        loop {
            let to_left = self.left(idx).is_some() && !self.left(idx).unwrap().is_empty();
            let to_right = self.right(idx).is_some() && !self.right(idx).unwrap().is_empty();
            if to_left {
                idx = Self::left_idx(idx);
            } else if to_right {
                idx = Self::right_idx(idx);
            } else {
                break;
            }
        }

        debug!("before swap {:?}", self);
        self.data.swap(0, idx);
        let to_ret = std::mem::take(&mut self.data[idx]);
        self.shift_down(0);
        debug!("after swap and take {:?}", self);
        Some(to_ret)
    }

    fn shift_down(&mut self, mut idx: usize) {
        loop {
            if self.data[idx].is_empty() {
                return;
            }

            let el = self.data[idx].front().unwrap();
            let mut replace_with = idx;
            // replace with smaller child
            let left = self.left(idx);
            let replace_with_left =
                left.is_some() && !left.unwrap().is_empty() && left.unwrap().front().unwrap() < el;
            let right = self.right(idx);
            let replace_with_right = right.is_some()
                && !right.unwrap().is_empty()
                && right.unwrap().front().unwrap() < el;
            if replace_with_left {
                replace_with = Self::left_idx(idx);
            } else if replace_with_right {
                replace_with = Self::right_idx(idx);
            } else {
                break;
            }
            // debug!("replace {} with {}", idx, replace_with);
            // let left_empty = self.left(idx).is_none() || self.left(idx).unwrap().is_empty();
            // let right_empty = self.right(idx).is_none() || self.right(idx).unwrap().is_empty();
            // if left_empty && right_empty {
            //     break;
            // }
            self.data.swap(idx, replace_with);
            idx = replace_with;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parent_idx() {
        let left = Pq::<i32>::left_idx(1);
        let right = Pq::<i32>::right_idx(1);
        assert_eq!(Pq::<i32>::parent_idx(left), Pq::<i32>::parent_idx(right));
    }

    #[test]
    fn check_many() {
        let mut pq = Pq::new(10);
        pq.insert(10);
        pq.insert(1);
        pq.insert(20);
        pq.insert(2);
        pq.insert(33);
        pq.insert(10);
        println!("{:?}", pq);
        for i in [1, 2, 10, 20, 33] {
            let ll = pq.get().unwrap();
            println!("{:?}", pq);
            assert!(ll.front().is_some(), "can't get ll with {} {:?}", i, pq);
            assert_eq!(*ll.front().unwrap(), i, "can't compare with {} {:?}", i, pq);
        }
    }

    #[test]
    fn check_simple() {
        let mut pq = Pq::new(1);
        pq.insert(1);
        println!("{:?}", pq);
        assert_eq!(*pq.get().unwrap().front().unwrap(), 1);

        let mut pq = Pq::new(2);
        pq.insert(2);
        pq.insert(1);
        println!("{:?}", pq);
        assert_eq!(*pq.get().unwrap().front().unwrap(), 1);
    }
}

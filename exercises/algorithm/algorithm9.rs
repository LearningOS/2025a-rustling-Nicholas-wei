/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        // 大概是将一个元素根据self.comparator放置到正确的位置
        // 在放置之前先假设放置的对象本身已经是一个正确的heap

        // 首先放置元素到末尾
        self.items.push(value);
        self.count += 1;

        // 接着循环和上层父元素比较并判断大小顺序
        let mut index = self.count;
        while index > 1 && (self.comparator)(&self.items[index], &self.items[self.parent_idx(index)]) {
            let par_index = self.parent_idx(index);
            self.items.swap(par_index, index);
            index = self.parent_idx(index);
        }

    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.count == 0 {
            return None;
        }else{
            // 移除顶部元素，把最后一个元素移动到堆顶部，之后开始sift down
            let result = self.items[1].clone();
            let last_item = self.items.pop().unwrap();
            self.count -= 1;

            if self.count > 0 {
                self.items[1] = last_item;
            }

            let mut start_index = 1;
            let mut smallest = 1;
            loop {
                let left = self.left_child_idx(start_index);
                let right = self.right_child_idx(start_index);

                if left < self.count && (self.comparator)(&self.items[left] , &self.items[start_index]) {
                    smallest = left;
                }
                if right < self.count && (self.comparator)(&self.items[right] , &self.items[smallest]) {
                    smallest = right;
                }

                if smallest == start_index {
                    break;
                }
                
                // 交换
                self.items.swap(smallest, start_index);
                start_index = smallest;
            }

            return Some(result);
        }
		
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
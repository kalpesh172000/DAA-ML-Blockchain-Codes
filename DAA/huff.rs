use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;

// A Huffman tree node
#[derive(Clone)]
struct MinHeapNode {
    data: char,
    freq: u32,
    left: Option<Rc<RefCell<MinHeapNode>>>,
    right: Option<Rc<RefCell<MinHeapNode>>>,
}

impl MinHeapNode {
    fn new(data: char, freq: u32) -> Rc<RefCell<MinHeapNode>> {
        Rc::new(
            RefCell::new(MinHeapNode {
                data,
                freq,
                left: None,
                right: None,
            })
        )
    }
}

// For comparison of two heap nodes (needed in min heap)
impl Ord for MinHeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip the order to make it a min-heap
        other.freq.cmp(&self.freq)
    }
}

impl PartialOrd for MinHeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MinHeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl Eq for MinHeapNode {}

fn print_codes(node: Rc<RefCell<MinHeapNode>>, code: String) {
    let node = node.borrow();

    if node.left.is_none() && node.right.is_none() && node.data != '$' {
        println!("{}: {}", node.data, code);
        return;
    }
    if let Some(ref left) = node.left {
        print_codes(left.clone(), code.clone() + "0");
    }
    if let Some(ref right) = node.right {
        print_codes(right.clone(), code.clone() + "1");
    }
}

fn huffman_codes(arr: &[char], freq: &[u32]) {
    let mut min_heap = BinaryHeap::new();

    for i in 0..arr.len() {
        min_heap.push(MinHeapNode::new(arr[i], freq[i]));
    }

    while min_heap.len() > 1 {
        let left = min_heap.pop().unwrap();
        let right = min_heap.pop().unwrap();

        let top = MinHeapNode::new('$', left.borrow().freq + right.borrow().freq);

        top.borrow_mut().left = Some(left);
        top.borrow_mut().right = Some(right);

        min_heap.push(top);
    }

    print_codes(min_heap.pop().unwrap(), "".to_string());
}

fn main(){
    let arr = ['a', 'b', 'c', 'd', 'e', 'f'];
    let freq = [5, 9, 12, 13, 16, 45];
    huffman_codes(&arr,&freq);
}
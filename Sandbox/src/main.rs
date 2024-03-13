use std::{mem, ops::Deref};

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

// 
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution{

}

impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut ptr  = &mut head;

        while  list1.is_some() && list2.is_some() {
            let l1 = &mut list1;
            let l2 = &mut list2;
            
            let mut temp = if l1.as_ref().unwrap().val  <= l2.as_ref().unwrap().val {l1} else {l2};

            mem::swap(&mut ptr, &mut  temp);
            mem::swap( temp, &mut ptr.as_mut().unwrap().next);
            ptr = &mut ptr.as_mut().unwrap().next;
        }

        mem::swap(ptr,  if list1.is_some() {&mut list1} else {&mut list2});
        head
    }
}



fn main() {
    println!("Hello, world!");
}

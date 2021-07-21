// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // if l1.is_none() || l2.is_none() {
        //   return l1.or(l2);
        // }
        // let mut result :Option<Box<ListNode>>;
        // loop{
        //   match (l1,l2){
        //     (None,None) => break;
        //     (Some(v1), Some(v2))=>{
        //       l1 = l1.and_then(|)
        //     }
        //   }
        // }
        // l1;
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

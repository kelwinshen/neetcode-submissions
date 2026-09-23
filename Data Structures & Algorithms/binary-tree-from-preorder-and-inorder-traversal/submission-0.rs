use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }

/* =============================================================================
HUMAN DEBUGGING TRACE (How the Robot thinks)
=============================================================================
Let's trace: preorder = [1, 2, 3], inorder = [2, 1, 3]

Initial Setup:
- We build our inorder_map: {2: index 0, 1: index 1, 3: index 2}
- We start `preorder_index` at 0.
- We call build(left: 0, right: 2) -> This means our "pile of blocks" is the whole array.

--- ROBOT 1 STARTS (left: 0, right: 2) ---
1. Looks at preorder[0]. The boss is `1`. 
2. Moves preorder_index forward to 1.
3. Asks HashMap: "Where is boss `1` in the inorder array?" -> Map says: Index 1.
4. Robot 1 splits its pile. 
   - Left pile is everything before index 1 (so just index 0).
   - Right pile is everything after index 1 (so just index 2).
5. Robot 1 pauses and hires Robot 2 to build the left pile...

    --- ROBOT 2 STARTS (left: 0, right: 0) ---
    1. Looks at preorder[1]. The boss is `2`.
    2. Moves preorder_index forward to 2.
    3. Asks HashMap: "Where is boss `2`?" -> Map says: Index 0.
    4. Robot 2 splits its pile.
       - Left pile is everything before index 0 (Empty!)
       - Right pile is everything after index 0 up to its boundary (Empty!)
    5. Because left/right piles are empty, Robot 2 is DONE. 
       It hands the finished block `2` back up to Robot 1.
    ------------------------------------------

6. Robot 1 takes block `2` and snaps it onto its LEFT side! (1.left = 2)
7. Now Robot 1 needs its right side. It pauses and hires Robot 3 to build the right pile...

    --- ROBOT 3 STARTS (left: 2, right: 2) ---
    1. Looks at preorder[2]. The boss is `3`.
    2. Moves preorder_index forward to 3.
    3. Asks HashMap: "Where is boss `3`?" -> Map says: Index 2.
    4. Robot 3 splits its pile. Both sides are empty!
    5. Robot 3 is DONE. Hands block `3` back to Robot 1.
    ------------------------------------------

8. Robot 1 takes block `3` and snaps it onto its RIGHT side! (1.right = 3)
9. Robot 1 is completely finished! It returns the full tree to us.
=============================================================================
*/

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() { 
            return None; 
        }

        // STEP 1: Build the "Brain" (HashMap)
        // We do this ONCE so the robot can instantly find where a boss sits in the 
        // inorder array without having to search for it using a slow loop.
        let mut inorder_map = HashMap::new();
        for (i, &val) in inorder.iter().enumerate() {
            inorder_map.insert(val, i);
        }

        // STEP 2: The Boss Counter
        // This keeps track of which boss is next in line in the preorder array.
        // It must be mutable so every robot can move the line forward when they grab a boss.
        let mut preorder_index = 0;
        
        // STEP 3: Hire the first Robot!
        // We tell it: "Your pile of blocks is the entire inorder array (from 0 to the end)."
        Self::build(
            &preorder, 
            &inorder_map, 
            &mut preorder_index, 
            0, 
            inorder.len() - 1
        )
    }

    // THE ROBOT INSTRUCTION MANUAL (Recursive Helper)
    fn build(
        preorder: &Vec<i32>,
        inorder_map: &HashMap<i32, usize>,
        preorder_index: &mut usize,
        left: usize,  // The starting boundary of this robot's pile
        right: usize, // The ending boundary of this robot's pile
    ) -> Option<Rc<RefCell<TreeNode>>> {
        
        // BASE CASE: If our left boundary crosses our right boundary, 
        // it means this pile of blocks is totally empty. Return nothing!
        if left > right {
            return None;
        }

        // 1. GRAB THE BOSS
        // Look at the preorder array to see who the boss of this current pile is.
        let root_val = preorder[*preorder_index];
        
        // Move the line forward so the next robot grabs the NEXT boss.
        *preorder_index += 1;

        // Physically create the Lego block for this boss.
        let root_node = Rc::new(RefCell::new(TreeNode::new(root_val)));

        // 2. FIND THE SPLIT
        // Ask the HashMap: "Where is this boss sitting in the inorder list?"
        let split_idx = *inorder_map.get(&root_val).unwrap();

        // 3. DELEGATE THE LEFT PILE
        // The left pile is everything from our 'left' boundary up to just before the boss.
        // We add `split_idx > left` just to make sure we don't accidentally subtract 1 from 0.
        if split_idx > left {
            // Hire a sub-robot to build the left pile, and snap whatever it hands back onto our left!
            root_node.borrow_mut().left = Self::build(preorder, inorder_map, preorder_index, left, split_idx - 1);
        }

        // 4. DELEGATE THE RIGHT PILE
        // The right pile is everything just after the boss, up to our 'right' boundary.
        if split_idx < right {
            // Hire a sub-robot to build the right pile, and snap whatever it hands back onto our right!
            root_node.borrow_mut().right = Self::build(preorder, inorder_map, preorder_index, split_idx + 1, right);
        }

        // 5. JOB'S DONE
        // Hand the finished, fully connected boss block back up to whoever hired us!
        Some(root_node)
    }
}
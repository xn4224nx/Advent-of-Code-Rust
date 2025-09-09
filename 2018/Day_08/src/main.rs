/*
 * --- Day 8: Memory Maneuver ---
 *
 * The sleigh is much easier to pull than you'd expect for something its weight.
 * Unfortunately, neither you nor the Elves know which way the North Pole is
 * from here.
 *
 * You check your wrist device for anything that might help. It seems to have
 * some kind of navigation system! Activating the navigation system produces
 * more bad news: "Failed to start navigation system. Could not read software
 * license file."
 *
 * The navigation system's license file consists of a list of numbers (your
 * puzzle input). The numbers define a data structure which, when processed,
 * produces some kind of tree that can be used to calculate the license number.
 *
 * The tree is made up of nodes; a single, outermost node forms the tree's root,
 * and it contains all other nodes in the tree (or contains nodes that contain
 * nodes, and so on).
 *
 * Specifically, a node consists of:
 *
 *      -   A header, which is always exactly two numbers:
 *          -   The quantity of child nodes.
 *          -   The quantity of metadata entries.
 *      -   Zero or more child nodes (as specified in the header).
 *      -   One or more metadata entries (as specified in the header).
 *
 * Each child node is itself a node that has its own header, child nodes, and
 * metadata. For example:
 *
 *      2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
 *      A----------------------------------
 *          B----------- C-----------
 *                           D-----
 *
 * In this example, each node of the tree is also marked with an underline
 * starting with a letter for easier identification. In it, there are four
 * nodes:
 *
 *      -   A, which has 2 child nodes (B, C) and 3 metadata entries (1, 1, 2).
 *      -   B, which has 0 child nodes and 3 metadata entries (10, 11, 12).
 *      -   C, which has 1 child node (D) and 1 metadata entry (2).
 *      -   D, which has 0 child nodes and 1 metadata entry (99).
 *
 * The first check done on the license file is to simply add up all of the
 * metadata entries. In this example, that sum is 1+1+2+10+11+12+2+99=138.
 *
 * PART 1:  What is the sum of all metadata entries?
 *
 * The second check is slightly more complicated: you need to find the value of
 * the root node (A in the example above).
 *
 * The value of a node depends on whether it has child nodes.
 *
 * If a node has no child nodes, its value is the sum of its metadata entries.
 * So, the value of node B is 10+11+12=33, and the value of node D is 99.
 *
 * However, if a node does have child nodes, the metadata entries become indexes
 * which refer to those child nodes. A metadata entry of 1 refers to the first
 * child node, 2 to the second, 3 to the third, and so on. The value of this
 * node is the sum of the values of the child nodes referenced by the metadata
 * entries. If a referenced child node does not exist, that reference is
 * skipped. A child node can be referenced multiple time and counts each time it
 * is referenced. A metadata entry of 0 does not refer to any child node.
 *
 * For example, again using the above nodes:
 *
 *      -   Node C has one metadata entry, 2. Because node C has only one child
 *          node, 2 references a child node which does not exist, and so the
 *          value of node C is 0.
 *
 *      -   Node A has three metadata entries: 1, 1, and 2. The 1 references
 *          node A's first child node, B, and the 2 references node A's second
 *          child node, C. Because node B has a value of 33 and node C has a
 *          value of 0, the value of node A is 33+33+0=66.
 *
 * So, in this example, the value of the root node is 66.
 *
 * PART 2:  What is the value of the root node?
 */

pub struct FlatTree {
    pub vals: Vec<usize>,
}

impl FlatTree {
    pub fn new(data_file: &str) -> Self {
        FlatTree {
            vals: std::fs::read_to_string(data_file)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        }
    }

    /// For all the nodes in the tree sum their constituent metadata. Also
    /// calculate the value of the root node.
    pub fn statistics(&self) -> (usize, usize) {
        let mut children = vec![Vec::new()];
        let mut metadata = vec![Vec::new()];

        /* Process the nodes outer to inner, start at the beginning. */
        let mut stack_child = vec![self.vals[0]];
        let mut stack_meta = vec![self.vals[1]];

        /* Keep a record of the current position in the tree. */
        let mut tree_idx = 2;
        let mut meta_sum = 0;
        let mut root_node_value = 0;

        while !stack_child.is_empty() {
            let l_c_idx = stack_child.len() - 1;
            let l_m_idx = stack_meta.len() - 1;

            /* If the current node has nodes below it save its header and move on. */
            if stack_child[l_c_idx] > 0 {
                stack_child[l_c_idx] -= 1;
                stack_child.push(self.vals[tree_idx]);
                stack_meta.push(self.vals[tree_idx + 1]);
                children.push(Vec::new());
                metadata.push(Vec::new());
                tree_idx += 2;

            /* If the node has metadata preserve it. */
            } else if stack_meta[l_m_idx] > 0 {
                meta_sum += self.vals[tree_idx];
                let last_meta = metadata.len() - 1;
                metadata[last_meta].push(self.vals[tree_idx]);
                stack_meta[l_m_idx] -= 1;
                tree_idx += 1;

            /* Deetermine the node value. */
            } else {
                stack_child.pop();
                stack_meta.pop();
                let curr_child = children.pop().unwrap();
                let curr_meta = metadata.pop().unwrap();

                let node_value = if !curr_child.is_empty() {
                    curr_meta
                        .into_iter()
                        .filter(|x| *x >= 1 && *x <= curr_child.len())
                        .map(|x| curr_child[x - 1])
                        .sum()
                } else {
                    curr_meta.into_iter().sum()
                };

                /* Save the node value for the level above. */
                if !children.is_empty() {
                    let last_child = children.len() - 1;
                    children[last_child].push(node_value);
                } else {
                    root_node_value = node_value;
                }
            }
        }
        return (meta_sum, root_node_value);
    }
}

fn main() {
    let (meta_sum, root_node) = FlatTree::new("./data/input_0.txt").statistics();
    println!("Part 1 = {}\nPart 2 = {}\n", meta_sum, root_node);
}

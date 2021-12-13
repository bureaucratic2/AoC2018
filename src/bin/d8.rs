use aoc2018::{load, Result};

fn main() -> Result<()> {
    let s = load(8);
    let nums = s
        .split(' ')
        .map(|digit| digit.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut nodes = Vec::new();
    traversal(&nums, 1, 0, &mut nodes);
    part1(&nodes);
    part2(&nodes);

    Ok(())
}

fn part1(nodes: &[Node]) {
    let sum = nodes
        .iter()
        .map(|node| node.metadata.iter().sum::<usize>())
        .sum::<usize>();
    println!("part1: {}", sum);
}

fn part2(nodes: &[Node]) {
    let value = traversal_value(nodes.last().unwrap(), nodes);
    println!("part2: {}", value);
}

fn traversal(
    nums: &[usize],
    childs: usize,
    mut offset: usize,
    nodes: &mut Vec<Node>,
) -> (usize, Vec<usize>) {
    let origin = offset;
    let mut to_parent = vec![];
    for _ in 0..childs {
        let childs = nums[offset];
        let metas = nums[offset + 1];
        let (childs_len, children_list) = traversal(nums, childs, offset + 2, nodes);
        let start = offset + 2 + childs_len;
        let metadata = nums[start..start + metas].to_vec();
        let node = Node {
            children_list,
            metadata,
        };
        nodes.push(node);
        to_parent.push(nodes.len() - 1);
        offset = start + metas;
    }
    (offset - origin, to_parent)
}

fn traversal_value(parent: &Node, nodes: &[Node]) -> usize {
    if parent.children_list.is_empty() {
        parent.metadata.iter().sum::<usize>()
    } else {
        let children = parent.children_list.len();
        let mut value = 0;
        for index in &parent.metadata {
            if index > &children || index == &0 {
                continue;
            }
            let child = &nodes[parent.children_list[index - 1]];
            value += traversal_value(child, nodes);
        }
        value
    }
}

#[derive(Debug)]
struct Node {
    // store child node index
    children_list: Vec<usize>,
    // store metadata
    metadata: Vec<usize>,
}

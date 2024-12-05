use petgraph::graph::{DiGraph, NodeIndex};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let result1 = solve1(INPUT);
    println!("Part 1:\nresult = {}", result1);

    let result2 = solve2(INPUT);
    println!("Part 2:\nresult = {}", result2);
}

fn parse_edges(input: &str) -> (DiGraph<u32, ()>, Vec<(u32, u32)>) {
    let edge_list: Vec<(u32, u32)> = input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.trim().split('|');
            let depends_on = parts.next().unwrap().trim().parse().unwrap();
            let x = parts.next().unwrap().trim().parse().unwrap();
            (x, depends_on)
        })
        .collect();
    (DiGraph::<u32, ()>::from_edges(edge_list.clone()), edge_list)
}

fn parse_page_lists(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|page| page.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

fn parse_input(input: &str) -> (DiGraph<u32, ()>, Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut parts = input.split("\n\n");
    let (graph, edge_list) = parse_edges(parts.next().unwrap());
    let page_lists = parse_page_lists(parts.next().unwrap());
    (graph, edge_list, page_lists)
}

fn solve1(input: &str) -> u32 {
    let (_, edge_list, page_lists) = parse_input(input);
    page_lists
        .iter()
        .filter(|pages| {
            let subgraph: DiGraph<u32, ()> = DiGraph::from_edges(edge_list.iter().filter(|edge| {
                pages.iter().any(|p| *p == edge.0) && pages.iter().any(|p| *p == edge.1)
            }));
            pages
                .iter()
                .enumerate()
                .take(pages.len() - 1)
                .all(|(i, page)| {
                    std::iter::repeat(NodeIndex::new(*page as usize))
                        .zip(pages.iter().skip(i + 1))
                        .all(|(me, future_page)| {
                            // check me is not depending on future_page
                            !petgraph::algo::has_path_connecting(
                                &subgraph,
                                me,
                                NodeIndex::new(*future_page as usize),
                                None,
                            )
                        })
                })
        })
        .map(|page| page[page.len() / 2])
        .sum()
}

fn solve2(input: &str) -> u32 {
    let (_, edge_list, page_lists) = parse_input(input);
    let incorrectly_ordered: Vec<&Vec<u32>> = page_lists
        .iter()
        .filter(|pages| {
            let subgraph: DiGraph<u32, ()> = DiGraph::from_edges(edge_list.iter().filter(|edge| {
                pages.iter().any(|p| *p == edge.0) && pages.iter().any(|p| *p == edge.1)
            }));
            pages
                .iter()
                .enumerate()
                .take(pages.len() - 1)
                .any(|(i, page)| {
                    std::iter::repeat(NodeIndex::new(*page as usize))
                        .zip(pages.iter().skip(i + 1))
                        .any(|(me, future_page)| {
                            // check some me is depending on future_page
                            petgraph::algo::has_path_connecting(
                                &subgraph,
                                me,
                                NodeIndex::new(*future_page as usize),
                                None,
                            )
                        })
                })
        })
        .collect();
    let reordered = incorrectly_ordered
        .iter()
        .map(|pages| {
            let subgraph: DiGraph<u32, ()> = DiGraph::from_edges(edge_list.iter().filter(|edge| {
                pages.iter().any(|p| *p == edge.0) && pages.iter().any(|p| *p == edge.1)
            }));
            petgraph::algo::toposort(&subgraph, None)
                .unwrap()
                .iter()
                .map(|n| n.index() as u32)
                .filter(|p| pages.contains(p))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    reordered.iter().map(|page| page[page.len() / 2]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
        "#;

    #[test]
    fn test1() {
        assert_eq!(solve1(TEST_INPUT), 143);
    }

    #[test]
    fn test2() {
        assert_eq!(solve2(TEST_INPUT), 123);
    }
}

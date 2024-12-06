use anyhow::anyhow;
use nom::Parser;

#[derive(Debug)]
pub struct Answer {
    pub part_1: i64,
    pub part_2: i64,
}

pub fn solution<'a>(input: &'a str) -> anyhow::Result<Answer> {
    let input = parser::input()
        .parse(input)
        .map_err(|err| anyhow!("failed to parse input: {}", err))?
        .1;

    // print!(pages_in_rules.)
    Ok(Answer {
        part_1: solution::sum_of_middle_page_numbers_of_valid_updates(&input),
        part_2: solution::sum_of_middle_page_numbers_of_fixed_invalid_updates(&input),
    })
}

#[derive(Debug, PartialEq, Eq)]
struct Input {
    page_ordering_rules: Vec<(i64, i64)>,
    updates: Vec<Vec<i64>>,
}

mod parser {
    use super::Input;

    pub type Error<'a> = nom::error::Error<&'a str>;
    pub trait Parser<'a, T> = nom::Parser<&'a str, T, Error<'a>>;

    pub fn input<'a>() -> impl Parser<'a, Input> {
        nom::sequence::separated_pair(
            page_ordering_rules(),
            nom::multi::many1(nom::character::complete::newline),
            updates(),
        )
        .map(|(page_ordering_rules, updates)| Input {
            page_ordering_rules,
            updates,
        })
    }

    fn page_ordering_rules<'a>() -> impl Parser<'a, Vec<(i64, i64)>> {
        nom::multi::separated_list1(nom::character::complete::newline, page_ordering_rule())
    }

    fn page_ordering_rule<'a>() -> impl Parser<'a, (i64, i64)> {
        nom::sequence::separated_pair(
            nom::character::complete::i64,
            nom::character::complete::char('|'),
            nom::character::complete::i64,
        )
    }

    fn updates<'a>() -> impl Parser<'a, Vec<Vec<i64>>> {
        nom::multi::separated_list1(nom::character::complete::newline, update())
    }

    fn update<'a>() -> impl Parser<'a, Vec<i64>> {
        nom::multi::separated_list1(
            nom::character::complete::char(','),
            nom::character::complete::i64,
        )
    }

    #[test]
    fn example() {
        assert_eq!(
            Ok(("", super::example::intermediate())),
            input().parse(super::example::input())
        );
    }
}

mod solution {
    use std::{
        collections::{BTreeMap, BTreeSet, HashMap, HashSet},
        ops::Not,
    };

    use guard::guard;

    use super::Input;

    fn make_disallowed_in_suffix_map(
        page_ordering_rules: &[(i64, i64)],
    ) -> BTreeMap<i64, BTreeSet<i64>> {
        page_ordering_rules
            .iter()
            .fold(BTreeMap::<i64, BTreeSet<i64>>::new(), |mut acc, (l, r)| {
                acc.entry(*r).or_default().insert(*l);
                acc
            })
    }

    fn is_valid_update(
        disallowed_in_suffix_map: &BTreeMap<i64, BTreeSet<i64>>,
        update: &[i64],
    ) -> bool {
        let mut all_disallowed = BTreeSet::<i64>::new();

        for page in update {
            if all_disallowed.contains(page) {
                return false;
            }
            if let Some(disallowed) = disallowed_in_suffix_map.get(page) {
                all_disallowed.append(&mut disallowed.clone());
            }
        }

        return true;
    }

    fn middle_page_number(update: &[i64]) -> i64 {
        update[update.len() / 2]
    }

    pub fn sum_of_middle_page_numbers_of_valid_updates(input: &Input) -> i64 {
        let disallowed_in_suffix_map = make_disallowed_in_suffix_map(&input.page_ordering_rules);
        input
            .updates
            .iter()
            .filter_map(|update| {
                is_valid_update(&disallowed_in_suffix_map, update)
                    .then_some(middle_page_number(update))
            })
            .sum()
    }

    #[derive(Debug, Default)]
    struct Graph {
        edges: HashMap<i64, HashSet<i64>>,
    }

    impl Graph {
        fn with_edges(edges: &[(i64, i64)]) -> Self {
            edges.iter().fold(Self::default(), |mut acc, (src, dest)| {
                acc.add_edge(*src, *dest);
                acc
            })
        }

        fn has_edge(&self, src: i64, dest: i64) -> bool {
            self.edges
                .get(&src)
                .map(|dest_vertices| dest_vertices.contains(&dest))
                .unwrap_or(false)
        }

        fn add_edge(&mut self, src: i64, dest: i64) {
            self.edges.entry(src).or_default().insert(dest);
            self.edges.entry(dest).or_default();
        }

        fn subgraph_with_vertices_subset<'a>(
            &'a self,
            vertices_subset: &HashSet<i64>,
        ) -> SubgraphView<'a> {
            let vertices_subset = vertices_subset
                .intersection(&self.vertices())
                .copied()
                .collect();
            SubgraphView {
                graph: self,
                vertices_subset,
            }
        }

        fn vertices(&self) -> HashSet<i64> {
            self.edges.keys().copied().collect()
        }
    }

    #[derive(Debug)]
    struct SubgraphView<'a> {
        graph: &'a Graph,
        vertices_subset: HashSet<i64>,
    }

    impl<'a> SubgraphView<'a> {
        fn hamiltonian_path(&self) -> Option<Vec<i64>> {
            self.topologically_sort().and_then(|t| {
                t.iter()
                    .zip(t.iter().skip(1))
                    .all(|(src, dest)| self.graph.has_edge(*src, *dest))
                    .then_some(t)
            })
        }

        fn topologically_sort(&self) -> Option<Vec<i64>> {
            let mut result = Vec::<i64>::with_capacity(self.vertices_subset.len());
            let mut marked_vertices = HashSet::<i64>::with_capacity(self.vertices_subset.len());

            loop {
                if let Some(unmarked_vertex) = self
                    .vertices_subset
                    .difference(&marked_vertices)
                    .next()
                    .copied()
                {
                    self.visit(
                        &mut result,
                        &mut marked_vertices,
                        &mut HashSet::new(),
                        unmarked_vertex,
                    )?;
                } else {
                    break;
                }
            }

            result.reverse();
            Some(result)
        }

        fn visit(
            &self,
            result: &mut Vec<i64>,
            marked_vertices: &mut HashSet<i64>,
            tmp_marks_vertices: &mut HashSet<i64>,
            vertex: i64,
        ) -> Option<()> {
            if marked_vertices.contains(&vertex) {
                return Some(());
            }
            // graph has at least one cycle
            if tmp_marks_vertices.contains(&vertex) {
                return None;
            }

            tmp_marks_vertices.insert(vertex);

            if let Some(dest_vertices) = self.graph.edges.get(&vertex) {
                dest_vertices
                    .iter()
                    .filter(|v| self.vertices_subset.contains(v))
                    .try_for_each(|v| {
                        self.visit(result, marked_vertices, tmp_marks_vertices, *v)
                    })?;
            }

            marked_vertices.insert(vertex);
            result.push(vertex);

            Some(())
        }
    }

    fn fix_update(rules_graph: &Graph, update: &[i64]) -> Option<Vec<i64>> {
        let subgraph = rules_graph.subgraph_with_vertices_subset(&update.iter().copied().collect());
        subgraph.hamiltonian_path()
    }

    pub fn sum_of_middle_page_numbers_of_fixed_invalid_updates(input: &Input) -> i64 {
        let disallowed_in_suffix_map = make_disallowed_in_suffix_map(&input.page_ordering_rules);
        let rules_graph = Graph::with_edges(&input.page_ordering_rules);

        input
            .updates
            .iter()
            .filter_map(|update| {
                is_valid_update(&disallowed_in_suffix_map, update)
                    .not()
                    .then(|| {
                        guard! {
                            let Some(fixed_update) = fix_update(&rules_graph, update) else {
                                panic!("INVALID RULE SET")
                            }
                        };
                        middle_page_number(&fixed_update)
                    })
            })
            .sum()
    }

    #[test]
    fn topological_sort_and_hamiltonian_path() {
        let graph = Graph::with_edges([(0, 1), (0, 2), (1, 2), (2, 3), (3, 0)].as_slice());

        assert_eq!(
            None,
            graph
                .subgraph_with_vertices_subset(&graph.vertices())
                .topologically_sort()
        );

        assert_eq!(
            Some(vec![0, 1, 2]),
            graph
                .subgraph_with_vertices_subset(&[0, 1, 2].into_iter().collect())
                .topologically_sort()
        );
        assert_eq!(
            Some(vec![0, 1, 2]),
            graph
                .subgraph_with_vertices_subset(&[0, 1, 2].into_iter().collect())
                .hamiltonian_path()
        );

        let graph = Graph::with_edges([(0, 1), (2, 1)].as_slice());
        assert_eq!(
            Some(vec![0, 2, 1]),
            graph
                .subgraph_with_vertices_subset(&graph.vertices())
                .topologically_sort()
        );
        assert_eq!(
            None,
            graph
                .subgraph_with_vertices_subset(&graph.vertices())
                .hamiltonian_path()
        );
    }

    #[test]
    fn example_is_valid_update() {
        let input = super::example::intermediate();
        let disallowed_in_suffix_map = make_disallowed_in_suffix_map(&input.page_ordering_rules);
        let check_update = |idx: usize, expected_validity: bool| {
            let update = &input.updates[idx];
            let is_valid = is_valid_update(&disallowed_in_suffix_map, update);
            assert_eq!(
                is_valid, expected_validity,
                "idx = {idx}, update = {update:?}"
            )
        };
        check_update(0, true);
        check_update(1, true);
        check_update(2, true);
        check_update(3, false);
        check_update(4, false);
        check_update(5, false);
    }

    #[test]
    fn example_fix_update() {
        let input = super::example::intermediate();
        let graph = Graph::with_edges(&input.page_ordering_rules);
        let disallowed_in_suffix_map = make_disallowed_in_suffix_map(&input.page_ordering_rules);
        let check_fixed_update = |idx: usize, expected_fixed_update: Vec<i64>| {
            let update = &input.updates[idx];
            let fixed_update = fix_update(&graph, update).unwrap();
            assert_eq!(
                fixed_update, expected_fixed_update,
                "idx = {idx}, update = {update:?}"
            );
            assert!(is_valid_update(&disallowed_in_suffix_map, &fixed_update));
        };
        check_fixed_update(3, vec![97, 75, 47, 61, 53]);
        check_fixed_update(4, vec![61, 29, 13]);
        check_fixed_update(5, vec![97, 75, 47, 29, 13]);
    }

    #[test]
    fn example() {
        assert_eq!(
            super::example::output_p_1(),
            sum_of_middle_page_numbers_of_valid_updates(&super::example::intermediate())
        );
        assert_eq!(
            super::example::output_p_2(),
            sum_of_middle_page_numbers_of_fixed_invalid_updates(&super::example::intermediate())
        );
    }
}

#[cfg(test)]
mod example {
    use super::Input;

    pub fn input() -> &'static str {
        include_str!("./examples/day5/example.txt")
    }

    pub fn intermediate() -> Input {
        include!("./examples/day5/intermediate.in")
    }

    pub fn output_p_1() -> i64 {
        143
    }

    pub fn output_p_2() -> i64 {
        123
    }
}

use itertools::Itertools;
use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

pub fn neighbors(
    (x, y): (usize, usize),
    (width, height): (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    const ALL: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    ALL.into_iter().filter_map(move |(dx, dy)| {
        let x2 = x.checked_add_signed(dx);
        let y2 = y.checked_add_signed(dy);
        x2.zip(y2).filter(|&(x2, y2)| x2 < width && y2 < height)
    })
}

pub trait Merge<T>: Sized {
    fn start(&self) -> &T;

    fn merge(self, other: Self) -> Result<Self, (Self, Self)>;
}

impl<T: Ord> Merge<T> for std::ops::Range<T> {
    fn start(&self) -> &T {
        &self.start
    }

    fn merge(self, other: Self) -> Result<Self, (Self, Self)> {
        let (r1, r2) = if self.start <= other.start {
            (self, other)
        } else {
            (other, self)
        };

        if r1.end < r2.start {
            Err((r1, r2))
        } else if r1.end <= r2.end {
            Ok(r1.start..r2.end)
        } else {
            Ok(r1)
        }
    }
}

impl<T: Ord> Merge<T> for std::ops::RangeInclusive<T> {
    fn start(&self) -> &T {
        self.start()
    }

    fn merge(self, other: Self) -> Result<Self, (Self, Self)> {
        let (r1, r2) = if self.start() <= other.start() {
            (self, other)
        } else {
            (other, self)
        };

        if r1.end() < r2.start() {
            Err((r1, r2))
        } else if r1.end() <= r2.end() {
            let start = r1.into_inner().0;
            let end = r2.into_inner().1;
            Ok(start..=end)
        } else {
            Ok(r1)
        }
    }
}

pub fn merge_ranges<T: Ord, R: Merge<T>>(iter: impl IntoIterator<Item = R>) -> Vec<R> {
    let mut ranges: Vec<_> = iter.into_iter().collect();

    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by(|r1, r2| r1.start().cmp(r2.start()));
    let (mut ranges, last) =
        ranges
            .into_iter()
            .fold((Vec::new(), None), |(mut ranges, prev), range| {
                let Some(prev) = prev else {
                    return (ranges, Some(range));
                };
                match prev.merge(range) {
                    Ok(r) => (ranges, Some(r)),
                    Err((r1, r2)) => {
                        ranges.push(r1);
                        (ranges, Some(r2))
                    }
                }
            });
    ranges.extend(last);
    ranges
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn from_char(ch: char) -> Option<Self> {
        match ch.to_ascii_lowercase() {
            'u' => Some(Direction::Up),
            'l' => Some(Direction::Left),
            'r' => Some(Direction::Right),
            'd' => Some(Direction::Down),
            _ => None,
        }
    }

    pub fn deltas(self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
        }
    }

    // For this function, Up is considered no rotation, Down is reverse, and Left and Right are left and right 90 degrees respectively.
    pub fn rotated(self, d: Self) -> Self {
        match d {
            Self::Up => self,
            Self::Left => self.rotated_left(),
            Self::Right => self.rotated_right(),
            Self::Down => self.reversed(),
        }
    }

    pub fn rotated_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
        }
    }

    pub fn rotated_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
        }
    }

    pub fn reversed(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
        }
    }

    pub fn rotate(&mut self, d: Self) {
        *self = self.rotated(d);
    }

    pub fn rotate_left(&mut self) {
        *self = self.rotated_left();
    }

    pub fn rotate_right(&mut self) {
        *self = self.rotated_right();
    }

    pub fn reverse(&mut self) {
        *self = self.reversed();
    }
}

/// `increment` should return true if the value rolled over
pub fn next_lexicographic<T>(slice: &mut [T], increment: impl Fn(&mut T) -> bool) {
    for i in (0..slice.len()).rev() {
        if !increment(&mut slice[i]) {
            break;
        }
    }
}

pub fn traveling_salesman_naive<N, Ty: petgraph::EdgeType>(
    graph: &petgraph::Graph<N, i64, Ty>,
    wrap_around: bool,
) -> (Vec<petgraph::prelude::NodeIndex>, i64) {
    let mut shortest = vec![];
    let mut shortest_length = i64::MAX;

    'outer: for permutation in graph.node_indices().permutations(graph.node_count()) {
        let mut length = 0_i64;
        for window in permutation.windows(2) {
            let &[n1, n2] = window else { panic!() };
            let Some(edge_index) = graph.find_edge(n1, n2) else {
                continue 'outer;
            };
            length += *graph.edge_weight(edge_index).unwrap();
        }

        if wrap_around {
            let (n1, n2) = (permutation[permutation.len() - 1], permutation[0]);
            let Some(edge_index) = graph.find_edge(n1, n2) else {
                continue 'outer;
            };
            length += *graph.edge_weight(edge_index).unwrap();
        }

        if length < shortest_length {
            shortest = permutation;
            shortest_length = length;
        }
    }

    (shortest, shortest_length)
}

pub fn nth_triangle_number(n: u64) -> u64 {
    n * (n + 1) / 2
}

pub fn modular_pow<T>(mut base: T, mut exponent: T, modulus: T) -> T
where
    T: Copy
        + Ord
        + std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Rem<Output = T>
        + std::ops::Div<Output = T>
        + num::traits::Zero
        + num::traits::One,
{
    let zero = T::zero();
    let one = T::one();
    let two = one + one;

    let mut result = one;
    base = base % modulus;
    while exponent > zero {
        if exponent % two == one {
            result = (result * base) % modulus;
        }
        exponent = exponent / two;
        base = (base * base) % modulus;
    }
    result
}

pub fn dijkstra<N, E, F, I, D>(root: N, mut neighbors: F, destination: D) -> Option<(Vec<N>, E)>
where
    N: Clone + Eq + std::hash::Hash,
    E: Clone + Ord + num::traits::Zero + std::ops::Add<Output = E>,
    F: FnMut(&N) -> I,
    I: IntoIterator<Item = (N, E)>,
    D: Fn(&N) -> bool,
{
    struct HeapEntry<K, V>(K, Reverse<V>);

    impl<K, V: Ord> Eq for HeapEntry<K, V> {}
    impl<K, V: Ord> PartialEq<Self> for HeapEntry<K, V> {
        fn eq(&self, other: &Self) -> bool {
            self.1.eq(&other.1)
        }
    }
    impl<K, V: Ord> PartialOrd<Self> for HeapEntry<K, V> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.1.partial_cmp(&other.1)
        }
    }
    impl<K, V: Ord> Ord for HeapEntry<K, V> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.1.cmp(&other.1)
        }
    }

    let mut q = BinaryHeap::from([HeapEntry(root.clone(), Reverse(E::zero()))]);
    let mut dist = rustc_hash::FxHashMap::default();
    dist.insert(root, E::zero());
    let mut prev = rustc_hash::FxHashMap::default();

    let target = loop {
        let Some(HeapEntry(node, Reverse(prior_weight))) = q.pop() else {
            return None;
        };

        if destination(&node) {
            break node;
        }

        for (neighbor_node, neighbor_edge) in neighbors(&node) {
            let total_weight = prior_weight.clone() + neighbor_edge;
            if dist
                .get(&neighbor_node)
                .is_none_or(|weight| total_weight < *weight)
            {
                dist.insert(neighbor_node.clone(), total_weight.clone());
                prev.insert(neighbor_node.clone(), node.clone());
                q.retain(|HeapEntry(n, _)| *n != neighbor_node);
                q.push(HeapEntry(neighbor_node, Reverse(total_weight)));
            }
        }
    };

    if !prev.contains_key(&target) {
        return None;
    }

    let mut path = Vec::new();
    let total_weight = dist.get(&target).unwrap().clone();

    let mut node = target;
    loop {
        let prev_node = prev.get(&node);
        path.push(node);
        let Some(prev_node) = prev_node else { break };
        node = prev_node.clone();
    }

    path.reverse();

    Some((path, total_weight))
}

#[test]
fn test_next_lexicographic() {
    let slice = &mut [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let increment = |d: &mut i32| {
        *d += 1;
        if *d == 10 {
            *d = 0;
            return true;
        }
        false
    };
    next_lexicographic(slice, increment);
    assert_eq!(slice, &[1, 2, 3, 4, 5, 6, 7, 9, 0]);

    next_lexicographic(slice, increment);
    assert_eq!(slice, &[1, 2, 3, 4, 5, 6, 7, 9, 1]);

    let slice = &mut [9, 9, 9, 9, 9, 9, 9];
    next_lexicographic(slice, increment);
    assert_eq!(slice, &[0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_dijkstra() {
    fn neighbors(node: &'static str) -> impl IntoIterator<Item = (&'static str, u32)> {
        match node {
            "A" => vec![("B", 7), ("E", 1)],
            "B" => vec![("A", 7), ("E", 8), ("C", 3)],
            "C" => vec![("B", 3), ("E", 2), ("D", 6)],
            "D" => vec![("E", 7), ("C", 6)],
            "E" => vec![("A", 1), ("B", 8), ("C", 2), ("D", 7)],
            _ => unreachable!(),
        }
    }

    fn destination(node: &'static str) -> bool {
        node == "D"
    }

    let result = dijkstra("A", |s| neighbors(s), |s| destination(s));
    assert!(result.is_some());
    let result = result.unwrap();
    assert_eq!(result.0, vec!["A", "E", "D"]);
    assert_eq!(result.1, 8);
}

#[test]
fn test_range_inclusive_merge_ranges() {
    let ranges = vec![3..=5, 10..=14, 16..=20, 12..=18];

    assert_eq!(merge_ranges(ranges), vec![3..=5, 10..=20]);
}


use num::integer::gcd;
use std::cmp::{max, Ordering, Reverse};
use std::ops::{Sub};
use std::collections::{BinaryHeap, HashMap, HashSet};


#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(Point(x as isize, y as isize)),
                _ => None,
            })
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &HashSet<Point>) -> usize {
    
        
    let mut dist = 0;
    let mut map: HashMap<Point, HashSet<Point>> = HashMap::new();
    for point_a in input.iter() {
        let mut seen: HashSet<(isize, isize)> = HashSet::new();
        for point_b in input.iter() {
            if point_a == point_b {
                continue;
            }

            let x = point_b.0 - point_a.0;
            let y = point_b.1 - point_a.1;
            
            let gcd = gcd(x, y);
            if seen.insert((x / gcd, y / gcd)) {
                map.entry(point_a.clone()).or_default().insert(point_b.clone());
            }
        }

        dist = max(dist, seen.len());
    }
    let (k, v) = map.iter().max_by_key(|(k, v)| v.len()).unwrap();
    println!("Point {:?}", k);
    v.len()
}


#[aoc(day10, part2)]
pub fn solve_part2(input: &HashSet<Point>) -> isize {
    let part1_p = Point(17, 22);

    let mut dist = 0;
    let mut map: HashMap<(isize, isize), BinaryHeap<Reverse<(isize, Point)>>> = HashMap::new();
    for point in input.iter() {
        let o = *point - part1_p;
        if o == Point(0, 0) {
            continue;
        }

        let x = point.0 - part1_p.0;
        let y = point.1 - part1_p.1;
        
        let gcd = gcd(x, y);
        map.entry((x / gcd, y / gcd))
            .or_insert(BinaryHeap::new())
            .push(Reverse((gcd, *point)));
    }

    let mut as_list: Vec<_> = map.iter().map(|(p, i)| (*p, i.clone())).collect();
    as_list.sort_by(|&(a, _), (b, _)| {
        let q = match (a.0 >= 0, a.1 >= 0) {
            (true, false) => 1,
            (true, true) => 2,
            (false, true) => 3,
            (false, false) => 4,
        };
        let q2 = match (b.0 >= 0, b.1 >= 0) {
            (true, false) => 1,
            (true, true) => 2,
            (false, true) => 3,
            (false, false) => 4,
        };
        let a1 = a.0 as f64 / a.1 as f64;
        let b1 = b.0 as f64 / b.1 as f64;
        (q.cmp(&q2)).then(b1.partial_cmp(&a1).unwrap_or(Ordering::Equal))
    });
    let mut c = 0;
    let mut list_ix = 0;
    let mut target = Point(0, 0);
    let mut non_empty_lists = as_list.len();
    while c < 200 && non_empty_lists > 0 {
        let heap = &mut as_list[list_ix].1;
        match heap.pop() {
            Some(Reverse((_, x))) => {
                target = x;
                c += 1;
            }
            None => {
                non_empty_lists -= 1;
            }
        }
        list_ix = (list_ix + 1) % as_list.len();
    }
    target.0 * 100 + target.1
}



#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub struct Point(pub isize, pub isize);

impl Sub for Point {
    type Output = Self;
    fn sub(self, r: Self) -> Self {
        Point(self.0 - r.0, self.1 - r.1)
    }
}


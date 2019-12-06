
use std::collections::HashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;


#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Orbit {
    left: String,
    right: String
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Orbit> {
    input
        .lines()
        .map(|l| {
            let split: Vec<&str> = l.split(")").collect();
            
            let orbit: Orbit = Orbit{left: split[0].to_string(), right: split[1].to_string()};
            orbit
        }).collect()
}



#[aoc(day6, part1)]
pub fn solve_part1(mut input: &Vec<Orbit>) -> i64 {
    
    let mut memory: HashMap<String, Orbit> = HashMap::new();
    for orbit in input {
        match memory.get(&orbit.right[..]) {
            Some(_o) => {
                println!("Double orbit: {:?}", &orbit.right);
            },
            None => {
                
                memory.insert((&orbit.right[..]).to_string(), orbit.clone());
            }
        }
    }
    let mut count = 0; 
    for orbit in input.into_iter() {
        let mut current_orbit: &str = &orbit.left[..];
        loop {
            match memory.get(current_orbit) {
                Some(o) => {
                    current_orbit = &o.left[..];
                    count+=1;
                   
                },
                None => {
                   break;
                }
            }
        }
        count+=1;
    }

    count
}


struct Grid<T> {
    nodes: Vec<Node<T>>,
}
 
struct Node<T> {
    data: T,
    edges: Vec<(usize,usize)>,
}
 
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    node: usize,
    cost: usize,
}
 
// Manually implement Ord so we get a min-heap instead of a max-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
 
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
 
type WeightedEdge = (usize, usize, usize);
 
impl<T> Grid<T> {
    fn new() -> Self {
        Grid { nodes: Vec::new() }
    }
 
    fn add_node(&mut self, data: T) -> usize {
        let node = Node {
            edges: Vec::new(),
            data: data,
        };
        self.nodes.push(node);
        self.nodes.len() - 1
    }
 
    fn create_edges<'a, I>(&mut self, iterator: I) where I: IntoIterator<Item=&'a WeightedEdge> {
        for &(start,end,weight) in iterator.into_iter() {
            self.nodes[start].edges.push((end,weight));
            self.nodes[end].edges.push((start,weight));
        }
 
    }
 
    fn find_path(&self, start: usize, end: usize) -> Option<(Vec<usize>, usize)> {
        let mut dist = vec![(usize::MAX, None); self.nodes.len()];
 
        let mut heap = BinaryHeap::new();
        dist[start] = (0, None);
        heap.push(State {
            node: start,
            cost: 0,
        });
 
        while let Some(State { node, cost }) = heap.pop() {
            if node == end {
                let mut path = Vec::with_capacity(dist.len() / 2);
                let mut current_dist = dist[end];
                path.push(end);
                while let Some(prev) = current_dist.1 {
                    path.push(prev);
                    current_dist = dist[prev];
                }
                path.reverse();
                return Some((path, cost));
            }
 
            if cost > dist[node].0 {
                continue;
            }
            for edge in &self.nodes[node].edges {
                let next = State {
                    node: edge.0,
                    cost: cost + edge.1,
                };
                if next.cost < dist[next.node].0 {
                    dist[next.node] = (next.cost, Some(node));
                    heap.push(next);
                }
            }
        }
        None
    }
}

#[aoc(day6, part2)]
pub fn solve_part2(mut input: &Vec<Orbit>) -> usize {
    let mut grid = Grid::new();
    let mut memory: HashMap<String, usize> = HashMap::new();
    let mut edges: Vec<(usize, usize, usize)> = Vec::new();
    //generate node map
    for orbit in input {
        match memory.get(&orbit.right[..]) {
            Some(_o) => {
                println!("Double orbit: {:?}", &orbit.right);
            },
            None => {
                memory.insert((&orbit.right[..]).to_string(), grid.add_node((&orbit.right[..]).to_string()));
            }
        }
    }
    //add bidirectional edges
    for orbit in input {
        match memory.get(&orbit.right[..]) {
            Some(r) => {
                match memory.get(&orbit.left[..]) {
                    Some(l) => {
                        edges.push((*r, *l, 1));
                        edges.push((*l, *r, 1));
                    },
                    None => {
                        
                    }
                }
            },
            None => {
                
            }
        }
    }
    grid.create_edges(&edges);
    match memory.get("YOU") {
        Some(r) => {
            match memory.get("SAN") {
                Some(l) => {
                    let (path, cost) = grid.find_path(*r,*l).unwrap();
 
                    /*print!("{}", grid.nodes[path[0]].data);
                    for i in path.iter().skip(1) {
                        print!(" -> {}", grid.nodes[*i].data);
                    }
                    println!("\nCost: {}", cost); */
                    return cost-2;
                },
                None => {
                    
                }
            }
        },
        None => {
            
        }
    }

    panic!("No Path found!");
}
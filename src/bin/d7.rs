use aoc2018::load;
use regex::Regex;

fn main() {
    let re = Regex::new(r"Step ([A-Z]+) must be finished before step ([A-Z]+) can begin.").unwrap();
    let s = load(7);

    let mut graph = Graph::new(26);
    for line in s.lines() {
        let caps = re.captures(line).unwrap();
        let src = caps[1].chars().next().unwrap() as u8 - b'A';
        let dst = caps[2].chars().next().unwrap() as u8 - b'A';
        graph.add_edge(src, dst);
    }

    part1(&mut graph);
    part2(&mut graph, 6);
}

fn part1(graph: &mut Graph) {
    graph.topological_sort_with_alphabet();
}

fn part2(graph: &mut Graph, workers: usize) {
    graph.topological_sort_with_workers(workers);
}

#[derive(Debug, Clone)]
struct Graph {
    vertex: usize,
    list: Vec<Vec<u8>>,
    in_degree: Vec<i32>,
}

impl Graph {
    fn new(vertex: usize) -> Self {
        Self {
            vertex,
            list: vec![vec![]; vertex],
            in_degree: vec![-1; vertex],
        }
    }

    fn add_edge(&mut self, src: u8, dst: u8) {
        if src >= 26 || dst >= 26 {
            panic!("vertex out of alphabet bound");
        }
        self.list[src as usize].push(dst);
        if self.in_degree[src as usize] < 0 {
            self.in_degree[src as usize] = 0;
        }
        if self.in_degree[dst as usize] < 0 {
            self.in_degree[dst as usize] = 1;
        } else {
            self.in_degree[dst as usize] += 1;
        }
    }

    fn topological_sort_with_alphabet(&self) {
        let mut stack = Vec::new();
        let mut in_degree = self.in_degree.clone();
        for (vertex, in_degree) in in_degree.iter_mut().enumerate() {
            if in_degree == &0 {
                stack.push(vertex);
                *in_degree = -1;
            }
        }

        stack.sort_unstable_by(|a, b| b.cmp(a));

        let mut res = Vec::with_capacity(self.vertex);
        while let Some(zero_indegree) = stack.pop() {
            res.push(zero_indegree);
            for vertex in self.list[zero_indegree].iter() {
                in_degree[*vertex as usize] -= 1;
            }
            for (vertex, in_degree) in in_degree.iter_mut().enumerate() {
                if in_degree == &0 {
                    stack.push(vertex);
                    *in_degree = -1;
                }
            }

            stack.sort_unstable_by(|a, b| b.cmp(a));
        }

        // format output to alphabet
        for vertex in res {
            print!("{}", (vertex as u8 + b'A') as char);
        }
        println!()
    }

    fn topological_sort_with_workers(&self, workers: usize) {
        let mut stack = Vec::new();
        let mut in_degree = self.in_degree.clone();
        for (vertex, in_degree) in in_degree.iter_mut().enumerate() {
            if in_degree == &0 {
                stack.push(vertex);
                *in_degree = -1;
            }
        }

        stack.sort_unstable_by(|a, b| b.cmp(a));
        let mut workers = Workers::new(workers);

        while let Some(zero_indegree) = stack.pop() {
            if !workers.add_work(zero_indegree) {
                // workers is busy, push back and wait till they finish some works
                stack.push(zero_indegree);
                if let Some(finished) = workers.flush() {
                    for vertex in self.list[finished].iter() {
                        in_degree[*vertex as usize] -= 1;
                    }
                    for (vertex, in_degree) in in_degree.iter_mut().enumerate() {
                        if in_degree == &0 {
                            stack.push(vertex);
                            *in_degree = -1;
                        }
                    }

                    stack.sort_unstable_by(|a, b| b.cmp(a));
                }
            } else {
                in_degree[zero_indegree] = -1;
            }

            while stack.is_empty() && workers.capacity < 6 {
                // all current works are dispatched, wait wrokers to finish some
                // and generate new works pushed into stack
                if let Some(finished) = workers.flush() {
                    for vertex in self.list[finished].iter() {
                        in_degree[*vertex as usize] -= 1;
                    }
                    for (vertex, in_degree) in in_degree.iter_mut().enumerate() {
                        if in_degree == &0 {
                            stack.push(vertex);
                            *in_degree = -1;
                        }
                    }

                    stack.sort_unstable_by(|a, b| b.cmp(a));
                }
            }
        }

        while workers.flush().is_some() {}

        println!("{:?}", workers);
    }
}

#[derive(Debug)]
struct Workers {
    capacity: usize,
    buf: Vec<(usize, usize)>,
    counter: usize,
}

impl Workers {
    fn new(workers: usize) -> Self {
        Self {
            capacity: workers,
            buf: vec![],
            counter: 0,
        }
    }

    fn add_work(&mut self, work: usize) -> bool {
        if self.capacity > 0 {
            self.capacity -= 1;
            self.buf.push((work, 61 + work));
            true
        } else {
            false
        }
    }

    fn flush(&mut self) -> Option<usize> {
        if !self.buf.is_empty() {
            self.buf.sort_by(|a, b| b.1.cmp(&a.1));
            let finished = self.buf.pop().unwrap();
            self.counter += finished.1;
            for pending in self.buf.iter_mut() {
                pending.1 -= finished.1;
            }
            self.capacity += 1;
            Some(finished.0)
        } else {
            None
        }
    }
}

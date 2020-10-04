use std::io;
use std::io::prelude::*;
use std::cmp::max;

fn main() {

    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines().map(|l| l.unwrap());

    let line1: Vec<usize> = lines.next().unwrap()
                                      .split_whitespace().map(|n| n.parse().expect("")).collect();

    let vertices = line1[0];

    let mut graph = vec![vec![];vertices+1];
    for line in lines {
        let nums: Vec<usize> = line.split_whitespace().map(|x| x.parse().expect("")).collect();
        let (vertex, edge): (usize, usize) = (nums[0], nums[1]);
        graph[vertex].push(edge);
    }

    let result = memoize(graph, vertices);
    println!("{}", result);
}

#[allow(dead_code)]
fn backtrack(graph: Vec<Vec<usize>>, num_vertices: usize) -> usize {

    fn go(graph: &Vec<Vec<usize>>, num_vertices: usize, vertex: usize) -> usize {
        if vertex == num_vertices + 1 {
            return 0;
        }

        let mut longest_distance = 0;
        for dest in graph[vertex].clone() {
            longest_distance = max(longest_distance, 1 + go(graph, num_vertices, dest));
        }

        longest_distance
    }

    let mut longest_distance = 0;
    for vertex in 1..=num_vertices {
        longest_distance = max(longest_distance, go(&graph, num_vertices, vertex));
    }

    longest_distance
}


fn memoize(graph: Vec<Vec<usize>>, num_vertices: usize) -> usize {

    fn go(graph: &Vec<Vec<usize>>, num_vertices: usize, vertex: usize, memo: &mut Vec<usize>) -> usize {
        if vertex == num_vertices + 1 {
            return 0;
        }

        if memo[vertex] == 0 {
            let mut longest_distance = 0;
            for dest in graph[vertex].clone() {
                longest_distance = max(longest_distance, 1 + go(graph, num_vertices, dest, memo));
            }
            memo[vertex] = longest_distance;
        }

        memo[vertex]
    }

    let mut memo = vec![0;num_vertices+1];
    let mut longest_distance = 0;
    for vertex in 1..=num_vertices {
        longest_distance = max(longest_distance, go(&graph, num_vertices, vertex, &mut memo));
    }

    longest_distance
}

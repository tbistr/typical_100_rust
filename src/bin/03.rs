use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let res = max_len_dfs(&graph, 0);
    // 最大値のインデックスを取得
    let mut max = 0;
    let mut max_index = 0;
    for (i, v) in res.iter().enumerate() {
        if max < *v {
            max = *v;
            max_index = i;
        }
    }
    let mut res = max_len_dfs(&graph, max_index);
    res.sort();
    res.reverse();
    println!("{}", res[0] + 1);
}

fn max_len_dfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let mut stack = vec![];
    let mut visited = vec![false; graph.len()];
    let mut result = vec![0; graph.len()];
    stack.push((start, 0));
    while let Some((node, depth)) = stack.pop() {
        visited[node] = true;
        for next in &graph[node] {
            if !visited[*next] {
                stack.push((*next, depth + 1));
                result[*next] = depth + 1;
            }
        }
    }

    result
}

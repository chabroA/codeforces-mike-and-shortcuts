use std::collections::VecDeque;
use std::io::{self, BufRead};

fn compute_distances(intersections: usize, shortcuts: Vec<usize>) -> Vec<i32> {
    let mut distances = vec![-1; intersections];
    distances[0] = 0;
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(0);

    while let Some(intersection) = queue.pop_front() {
        let prev = intersection.checked_sub(1).unwrap_or_default();
        let next = intersection + 1;
        let shortcut = shortcuts[intersection] - 1;

        let distance = distances[intersection] + 1;

        if prev > 0 && (distances[prev] < 0 || distance < distances[prev]) {
            distances[prev] = distance;
            queue.push_back(prev);
        }

        if next < intersections && (distances[next] < 0 || distance < distances[next]) {
            distances[next] = distance;
            queue.push_back(next);
        }

        if shortcut < intersections && (distances[shortcut] < 0 || distance < distances[shortcut]) {
            distances[shortcut] = distance;
            queue.push_back(shortcut);
        }
    }

    distances
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let intersections = iterator
        .next()
        .expect("Invalid input, number of intersections not provided")
        .unwrap()
        .parse::<usize>()
        .expect("Invalid input, number of intersections should be an integer");

    let shortcuts = iterator
        .next()
        .expect("Invalid input, list of shortcuts not provided")
        .unwrap()
        .split_whitespace()
        .map(|x| {
            x.parse::<usize>()
                .expect("Invalid input, shortcut should be an integer")
        })
        .collect::<Vec<usize>>();

    assert!(
        intersections == shortcuts.len(),
        "Invalid input, number of intersections should be equal to number of shortcuts"
    );

    let distances = compute_distances(intersections, shortcuts);

    println!(
        "{}",
        distances
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let intersections = 3;
        let shortcuts = vec![2, 2, 3];
        let distances = compute_distances(intersections, shortcuts);
        assert_eq!(distances, vec![0, 1, 2]);
    }

    #[test]
    fn test_2() {
        let intersections = 5;
        let shortcuts = vec![1, 2, 3, 4, 5];
        let distances = compute_distances(intersections, shortcuts);
        assert_eq!(distances, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_3() {
        let intersections = 7;
        let shortcuts = vec![4, 4, 4, 4, 7, 7, 7];
        let distances = compute_distances(intersections, shortcuts);
        assert_eq!(distances, vec![0, 1, 2, 1, 2, 3, 3]);
    }
}

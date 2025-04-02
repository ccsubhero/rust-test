// 请从命令行读取一行字符串（例如 "apple banana pear banana apple banana"）。
// 使用空格进行拆分，统计每个单词出现的次数，并以从高到底的顺序输出。
// 如果出现次数相同，按单词本身的字典序从小到大排序输出。
//在下面实现run行数，并在marin.rs中调用
use std::collections::HashMap;
use std::io::{self, BufRead};   //引入io库
use std::cmp::Ordering;   //引入Ordering库
pub fn run() {
    let mut map = HashMap::new();
    //增加提示，输入一行字符串
    println!("Please input a line of string:");
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    for word in line.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }
    let mut words: Vec<_> = map.iter().collect();
    words.sort_by(|a, b| match b.1.cmp(a.1) {
        Ordering::Equal => a.0.cmp(b.0),
        other => other,
    });
    for (word, count) in words {
        println!("{}: {}", word, count);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_input() {
        let input = "apple banana pear banana apple banana";
        let expected_output = vec![
            ("banana", 3),
            ("apple", 2),
            ("pear", 1),
        ];
        let mut map = HashMap::new();
        for word in input.split_whitespace() {
            *map.entry(word).or_insert(0) += 1;
        }
        let mut words: Vec<_> = map.iter().collect();
        words.sort_by(|a, b| match b.1.cmp(a.1) {
            Ordering::Equal => a.0.cmp(b.0),
            other => other,
        });
        let result: Vec<_> = words.into_iter().map(|(word, count)| (*word, *count)).collect();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let expected_output: Vec<(&str, i32)> = vec![];
        let mut map = HashMap::new();
        for word in input.split_whitespace() {
            *map.entry(word).or_insert(0) += 1;
        }
        let mut words: Vec<_> = map.iter().collect();
        words.sort_by(|a, b| match b.1.cmp(a.1) {
            Ordering::Equal => a.0.cmp(b.0),
            other => other,
        });
        let result: Vec<_> = words.into_iter().map(|(word, count)| (*word, *count)).collect();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_tie_breaker() {
        let input = "apple banana apple banana pear";
        let expected_output = vec![
            ("apple", 2),
            ("banana", 2),
            ("pear", 1),
        ];
        let mut map = HashMap::new();
        for word in input.split_whitespace() {
            *map.entry(word).or_insert(0) += 1;
        }
        let mut words: Vec<_> = map.iter().collect();
        words.sort_by(|a, b| match b.1.cmp(a.1) {
            Ordering::Equal => a.0.cmp(b.0),
            other => other,
        });
        let result: Vec<_> = words.into_iter().map(|(word, count)| (*word, *count)).collect();
        assert_eq!(result, expected_output);
    }
}


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
//增加测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "apple banana pear banana apple banana";
        let output = "banana: 3\napple: 2\npear: 1\n";
        let mut result = Vec::new();
        let old_stdout = std::io::stdout();
        let mut buffer = Vec::new();
        let mut new_stdout = std::io::stdout();
        new_stdout.write_all(&mut buffer).unwrap();
        std::io::stdout = new_stdout;
        run();
        std::io::stdout = old_stdout;
        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, output);
    }
}

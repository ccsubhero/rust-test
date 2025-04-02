use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::io;

// 实现一个命令行工具，对指定目录下的所有文本文件进行搜索，将匹配结果汇总后输出。
// 为增强可玩性和综合性，该工具需要支持：
// - 命令行参数（接收搜索关键词、目录路径、是否忽略大小写等）。
// - 并发搜索。
// - 消息通信。
// - 数据结构。
// - 错误处理。
// - 文件操作。
// - 迭代器与泛型（文本行迭代、搜索函数可考虑使用泛型或 trait 做一定延伸）。
// - 可选扩展：忽略大小写、正则匹配、统计行数或文件数等。

pub fn summarize_results(results: &Arc<Mutex<Vec<String>>>) -> io::Result<()> {
    let results = results.lock().unwrap();
    let summary = results.join("\n");
    fs::write("questions6.txt", summary)?;
    Ok(())
}
pub fn run(keyword: &str, dir_path: &str, ignore_case: bool) -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    let dir_path = Path::new(dir_path);

    if !dir_path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Provided path is not a directory"));
    }

    let files = fs::read_dir(dir_path)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file() && entry.path().extension().map_or(false, |ext| ext == "txt"))
        .collect::<Vec<_>>();

    let keyword = if ignore_case {
        keyword.to_lowercase()
    } else {
        keyword.to_string()
    };

    let keyword = Arc::new(keyword);
    let results = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    for file in files {
        let tx = tx.clone();
        let keyword = Arc::clone(&keyword);
        let results = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let file_path = file.path();
            if let Ok(content) = fs::read_to_string(&file_path) {
                let lines = content.lines();
                for (line_number, line) in lines.enumerate() {
                    let search_line = if ignore_case {
                        line.to_lowercase()
                    } else {
                        line.to_string()
                    };

                    if search_line.contains(&*keyword) {
                        let result = format!(
                            "File: {:?}, Line {}: {}",
                            file_path, line_number + 1, line
                        );
                        results.lock().unwrap().push(result.clone());
                        tx.send(result).unwrap();
                    }
                }
            }
        });

        handles.push(handle);
    }

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    while let Ok(result) = rx.recv() {
        println!("{}", result);
    }

    summarize_results(&results)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_run() {
        let keyword = "apple";
        let dir_path = "test";
        let ignore_case = false;

        fs::create_dir(dir_path).expect("Failed to create directory");

        let file1 = format!("{}/file1.txt", dir_path);
        let file2 = format!("{}/file2.txt", dir_path);

        fs::write(&file1, "apple banana\npear banana\napple banana").expect("Failed to write to file");
        fs::write(&file2, "apple banana\npear banana\napple banana").expect("Failed to write to file");

        let result = run(keyword, dir_path, ignore_case);

        fs::remove_dir_all(dir_path).expect("Failed to remove directory");

        assert!(result.is_ok());
    }
}

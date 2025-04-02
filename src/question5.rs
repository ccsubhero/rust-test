// 使用多线程并行计算某个函数的值或模拟并发任务。
// 需要创建 3 个线程同时进行下载，并在下载完成后将结果（例如“URL + 下载完成”）
// 通过消息通道（std::sync::mpsc）发送回主线程。主线程依次接收并打印结果。
// 在下面实现run函数，并在marin.rs中调用,需要有“URL + 下载完成”这种结果提示
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
pub fn run() {
    let (tx, rx) = mpsc::channel();
    let urls = vec![
        "https://www.rust-lang.org",
        "https://doc.rust-lang.org",
        "https://github.com",
    ];
    for url in urls {
        let tx = tx.clone();
        thread::spawn(move || {
            let content = download(url);
            tx.send(content).unwrap();
        });
    }
    drop(tx);
    for received in rx {
        println!("{} downloaded", received);
    }
}
fn download(url: &str) -> String {
    thread::sleep(Duration::from_secs(2));
    url.to_string()
}
//增加测试
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_run() {
        let start = Instant::now();
        run();
        let duration = start.elapsed();
        assert!(duration.as_secs() < 3);
    }
}

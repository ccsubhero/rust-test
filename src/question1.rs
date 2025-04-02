// 从命令行读取一个整数 n（若读取失败或没有输入则默认 n = 5）。
// 打印从 1 到 n 的所有整数，每行一个。
// 若该整数可以被 3 整除，则在数字后面附加输出 "Fizz"；若可以被 5 整除，则附加输出 "Buzz"；若同时满足可以被 3 和 5 整除的情况，则输出 "FizzBuzz"。

use std::env;

//实现基本函数，将调用的逻辑放在main.rs中
pub fn run() {
	let args: Vec<String> = env::args().collect();
	let n: i32 = args.get(1)
		.and_then(|arg| arg.parse().ok())
		.unwrap_or(5);

	for i in 1..=n {
		if i % 3 == 0 && i % 5 == 0 {
			println!("{} FizzBuzz", i);
		} else if i % 3 == 0 {
			println!("{} Fizz", i);
		} else if i % 5 == 0 {
			println!("{} Buzz", i);
		} else {
			println!("{}", i);
		}
	}
}

//增加测试
#[cfg(test)]
mod tests {

    #[test]
    fn test_main() {
        use std::process::Command;

        let output = Command::new("cargo")
            .args(&["run", "--", "15"])
            .output()
            .expect("Failed to execute process");

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "1\n\
            2\n\
            3 Fizz\n\
            4\n\
            5 Buzz\n\
            6 Fizz\n\
            7\n\
            8\n\
            9 Fizz\n\
            10 Buzz\n\
            11\n\
            12 Fizz\n\
            13\n\
            14\n\
            15 FizzBuzz\n"
        );
    }
}   

// 从命令行参数接收一个文件路径，例如 input.txt。如果没有提供路径或文件无法打开，给出相应的错误提示并退出。
// 读取该文件的所有文本内容，统计文本中一共包含的字符数（不含换行符）与行数，并将结果写入 output.txt。
// 若 output.txt 文件已存在，可以选择直接覆盖或者追加，任选其一，但需要在程序里明确注释或说明处理方式。
// 在下面实现run函数，并在marin.rs中调用，其中文件路径在用户执行了run之后进行提示输入
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::env;
use std::fs;
pub fn run() {

  
  //输出提示信息，输入文件路径
    println!("Please input a file path:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input_file = input.trim();
    let output_file = "output.txt";
    let input = File::open(input_file).expect("Failed to open file");
    let output = File::create(output_file).expect("Failed to create or overwrite file");
    let reader = io::BufReader::new(input);
    let mut char_count = 0;
    let mut line_count = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        char_count += line.chars().count();
        line_count += 1;
    }
    let mut writer = io::BufWriter::new(output);
    writeln!(writer, "Character count: {}", char_count).expect("Failed to write to file");
    writeln!(writer, "Line count: {}", line_count).expect("Failed to write to file");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_with_valid_file() {
        let test_input = "test_input.txt";
        let test_output = "output.txt";

        // Create a test input file
        fs::write(test_input, "Hello\nWorld\nRust").expect("Failed to create test input file");

        // Run the function
        run();

        // Verify the output file
        let output_content = fs::read_to_string(test_output).expect("Failed to read output file");
        assert!(output_content.contains("Character count: 14"));
        assert!(output_content.contains("Line count: 3"));

        // Clean up
        fs::remove_file(test_input).expect("Failed to remove test input file");
        fs::remove_file(test_output).expect("Failed to remove test output file");
    }

    #[test]
    #[should_panic(expected = "Failed to open file")]
    fn test_run_with_invalid_file() {
        let invalid_file = "non_existent.txt";

        // Simulate user input for an invalid file
        let _ = File::open(invalid_file).expect("Failed to open file");
    }
}

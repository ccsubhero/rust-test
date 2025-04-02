// 从命令行参数接收一个文件路径，例如 input.txt。如果没有提供路径或文件无法打开，给出相应的错误提示并退出。
// 读取该文件的所有文本内容，统计文本中一共包含的字符数（不含换行符）与行数，并将结果写入 output.txt。
// 若 output.txt 文件已存在，可以选择直接覆盖或者追加，任选其一，但需要在程序里明确注释或说明处理方式。
// 在下面实现run函数，并在marin.rs中调用，其中文件路径在用户执行了run之后进行提示输入
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::env;
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
//增加测试
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_run() {
        let input = "input.txt";
        let output = "output.txt";
        let content = "apple banana\npear banana\napple banana";
        fs::write(input, content).expect("Failed to write to file");
        run();
        let output = fs::read_to_string(output).expect("Failed to read file");
        assert_eq!(output, "Character count: 38\nLine count: 3\n");
    }


}
// 定义一个 Student 结构体，包含以下字段：name、age、score，并能够在 main.rs 中调用 Student 的方法。
// 实现 Student 的功能：
// new(name: &str, age: u8, score: f32) -> Student：返回一个新的学生实例。
// show(&self)：打印 Student 的信息，格式如 Name: Alice, Age: 18, Score: 95.5。
// is_passed(&self) -> bool：如果 score >= 60.0 则返回 true，否则返回 false。
// 在 main.rs 中传入学生的姓名、年龄、分数，打印学生信息，并判断是否通过。
// 在 question2.rs 中增加测试。
// 提示：在 main.rs 中调用 Student 的方法时，可以使用 Student::new() 的方式创建一个新的学生实例。
// 提示：在 main.rs 中调用 Student 的方法时，可以使用 student.show() 的方式打印学生信息。
// 提示：在 main.rs 中调用 Student 的方法时，可以使用 student.is_passed() 的方式判断是否通过。
// 提示：在 question2.rs 中增加测试时，可以使用 #[cfg(test)] 标记测试模块，然后在模块中定义测试函数。
// 提示：在 question2.rs 中增加测试时，可以使用 assert_eq! 宏来断言测试结果。
struct Student {
	name: String,
	age: u8,
	score: f32,
}

// 实现 Student 的功能，并通过main.rs中的调用进行测试
impl Student {
	// new(name: &str, age: u8, score: f32) -> Student：返回一个新的学生实例。
	pub fn new(name: &str, age: u8, score: f32) -> Student {
		Student {
			name: name.to_string(),
			age,
			score,
		}
	}

	// show(&self)：打印 Student 的信息，格式如 Name: Alice, Age: 18, Score: 95.5。
	pub fn show(&self) {
		println!("Name: {}, Age: {}, Score: {:.1}", self.name, self.age, self.score);
	}

	// is_passed(&self) -> bool：如果 score >= 60.0 则返回 true，否则返回 false。
	pub fn is_passed(&self) -> bool {
		self.score >= 60.0
	}

}
	//增加调用，在main.rs中传入学生的姓名、年龄、分数，打印学生信息，并判断是否通过
	pub fn run(name: &str, age: u8, score: f32) {
		let student = Student::new(name, age, score);
		student.show();
		println!("Is passed: {}", student.is_passed());
	}

//增加测试
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_student() {
		let student = Student::new("Alice", 18, 95.5);
		assert_eq!(student.name, "Alice");
		assert_eq!(student.age, 18);
		assert_eq!(student.score, 95.5);
		assert_eq!(student.is_passed(), true);
	}
}

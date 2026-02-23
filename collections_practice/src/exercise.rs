use std::collections::HashMap;
use std::io;
pub fn run() {
    //1.find median and mode
    let mut numbers = vec![4, 2, 7, 2, 9, 2, 5, 7, 7];
    // 计算 median
    // TODO: 排序 vector
    numbers.sort();
    // TODO: 找到中间值
    let median = if numbers.len() % 2 == 0 {
        let mid = numbers.len() / 2;
        (numbers[mid - 1] + numbers[mid]) / 2
    } else {
        numbers[numbers.len() / 2]
    };
    println!("Median: {}", median);

    // 计算 mode
    // TODO: 使用 HashMap 统计频率
    let mut counts = HashMap::new();
    for &num in &numbers {
        *counts.entry(num).or_insert(0) += 1;
    }
    // TODO: 找出出现次数最多的值
    let mode = counts
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(&num, _)| num)
        .unwrap_or(0);
    println!("Mode: {:?}", mode);

    //2.pig latin transformer
    let words = vec!["apple", "banana", "orange", "grape"];
    for word in words {
        println!("{} - {}", word, to_pig_latin(word));
    }

    fn to_pig_latin(word: &str) -> String {
        let vowels = "aeiou";

        //get first char of word, return empty string if word is empty
        let first_char = match word.chars().next() {
            Some(c) => c,
            None => return String::new(),
        };

        //if a word start with a vowel, add "hay" to the end
        if vowels.contains(first_char) {
            format!("{}-hay", word)
        } else {
            //if a word start with a consonant, move all the letters before the first vowel to the end of the word, and add "ay"
            let rest = &word[1..];
            format!("{}-{}ay", rest, first_char)
        }
    }

    //3.department management
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("===== Department Management =====");
        println!("1. Add employee(format: Add <name> to <department>)");
        println!("2. List employees by department");
        println!("3. Remove employee(format: Remove <name> from <department>)");
        println!("4. List all employees");
        println!("5. Exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim(); // remove \n in the end

        match input {
            "1" => {
                println!("Please enter your cmd(format: Add <name> to <department>)");
                let mut cmd = String::new();
                io::stdin()
                    .read_line(&mut cmd)
                    .expect("faield to read line");
                let cmd = cmd.trim();
                let parts: Vec<&str> = cmd.splitn(4, " ").collect();
                if parts.len() == 4 && parts[0] == "Add" && parts[2] == "to" {
                    let name = parts[1];
                    let department = parts[3];

                    departments
                        .entry(department.to_string())
                        .or_insert_with(Vec::new)
                        .push(name.to_string());

                    println!("{} has been added to {}", name, department);
                } else {
                    println!("Invalid command format. Please use: Add <name> to <department>");
                }
            }

            "2" => {
                println!("Please enter the department name you want to list employees from:");
                let mut dept = String::new();
                io::stdin().read_line(&mut dept).expect("faield to read");
                let dept = dept.trim();
                match departments.get(dept) {
                    Some(employee) => {
                        let mut sorted_employee = employee.clone();
                        sorted_employee.sort();
                        println!("Employees in {}: {:?}", dept, sorted_employee);
                    }
                    None => {
                        println!("Department {} not found", dept);
                    }
                }
            }

            "3" => {
                println!("Please enter your cmd(format: Remove <name> from <department>)");
                let mut cmd = String::new();
                io::stdin()
                    .read_line(&mut cmd)
                    .expect("faield to read line");
                let cmd = cmd.trim();
                let parts: Vec<&str> = cmd.splitn(4, " ").collect();
                if parts.len() == 4 && parts[0] == "Remove" && parts[2] == "from" {
                    let name = parts[1];
                    let department = parts[3];

                    if let Some(employees) = departments.get_mut(department) {
                        employees.retain(|e| e != name);
                        println!("{} has been removed from {}", name, department);
                    } else {
                        println!("Department {} not found", department);
                    }
                } else {
                    println!("Invalid command format. Please use: Remove <name> from <department>");
                }
            }

            "5" => {
                println!("bye!");
                break;
            }

            "4" => {
                for (department, employees) in &departments {
                    println!("Department {}: {:?}", department, employees);
                }
            }

            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}

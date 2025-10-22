fn main() {
    loop {
        let input = get_input();
        if input.contains("exit") {
            std::process::exit(0);
        }
        let mut equation = Equation::from_string(&input);
        //println!("Equation: {:#?}", equation);
        if equation.is_some() {
            let mut equation = equation.unwrap();
            equation.process_left();
        }
    }
}

#[derive(Debug)]
struct Equation<'a> {
    left_string: &'a str,
    right_string: &'a str,
    left_values: Vec<String>,
    right_values: Vec<String>,
    left_operators: Vec<String>,
    right_operators: Vec<String>,
}

impl<'a> Equation<'a> {
    pub fn from_string(input: &'a str) -> Option<Self> {
        let operators = ['+', '-', '*', '/'];
        let mut split: Vec<&str> = input.split("=").collect();
        if split.len() != 2 {
            println!("Equation must have two sides!");
            return None
        }
        Some(Self {
            left_string: split[0],
            right_string: split[1],
            left_values: split[0].split(|c| operators.contains(&c))
                .map(|s| 
                    s.chars()
                    .filter(|c| !c.is_whitespace()) // remove all whitespace
                    .collect::<String>()
                ).filter(|s: &String| !s.is_empty())
                .collect(),
            right_values: split[1].split(|c| operators.contains(&c))
                .map(|s| 
                    s.chars()
                    .filter(|c| !c.is_whitespace()) // remove all whitespace
                    .collect::<String>()
                ).filter(|s: &String| !s.is_empty())
                .collect(),
            left_operators: split[0].split(|c| !operators.contains(&c))
                .map(|s| 
                    s.chars()
                    .filter(|c| !c.is_whitespace()) // remove all whitespace
                    .collect::<String>()
                ).filter(|s: &String| !s.is_empty())
                .collect(),
            right_operators: split[1].split(|c| !operators.contains(&c))
                .map(|s| 
                    s.chars()
                    .filter(|c| !c.is_whitespace()) // remove all whitespace
                    .collect::<String>()
                ).filter(|s: &String| !s.is_empty())
                .collect()
        })
    }

    pub fn process_left(&mut self) -> Option<String> {
        for i in 0..self.left_values.len() - 1 {
            let mut to_remove1 = 0.0;
            let mut to_remove2 = 0.0;
            let result = match self.left_operators[i].chars().next().unwrap() {
                '+' => {
                    let val1: f32 = self.left_values[i].parse::<f32>().unwrap();
                    let val2: f32 = self.left_values[i+1].parse::<f32>().unwrap();
                    println!("{}", val1 + val2);
                    to_remove1 = val1;
                    to_remove2 = val2;
                    Some(format!("{}", val1+val2))
                },
                '-' => {
                },
                _ => {
                    println!("Multiplication and division are not supported yet!");
                    return None;
                }
            };
            self.left_values.clone().into_iter().filter(|c| *c != to_remove1.to_string()).collect::<Vec<_>>();
            self.left_values.push(format!("{}", result));
        }
        None
    }
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to get input\n");
    return input;
}

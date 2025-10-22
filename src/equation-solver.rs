fn main() {
    loop {
        let input = get_input();
        let equation = Equation::from_string(&input);
        println!("Equation: {:#?}", equation)
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
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to get input\n");
    return input;
}

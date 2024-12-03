pub fn main() {
    let input = advent::input_from_args()
        .map(|line| line.expect("unable to get line"))
        .collect::<Vec<_>>()
        .join("");

    let multipliers = find_multipliers(input.as_str());

    println!("multipliers: {:?}", multipliers);
    println!("sum: {:?}", sum_multipliers(multipliers.clone(), false));
    println!("dodont sum: {:?}", sum_multipliers(multipliers, true));
}

#[derive(Clone, Debug)]
enum Value {
    Mul(usize, usize),
    Do,
    Dont,
}

fn find_multipliers(input: &str) -> Vec<Value> {
    let re = regex::Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do\(\))|(don't\(\))")
        .expect("invalid regex");

    re.captures_iter(input)
        .filter_map(|cap| {
            if let (Some(a), Some(b)) = (cap.get(1), cap.get(2)) {
                return Some(Value::Mul(
                    a.as_str().parse().expect("unable to parse mul a"),
                    b.as_str().parse().expect("unable to parse mul b"),
                ));
            } else if let (Some(_), None) = (cap.get(3), cap.get(4)) {
                return Some(Value::Do);
            } else if let (None, Some(_)) = (cap.get(3), cap.get(4)) {
                Some(Value::Dont)
            } else {
                None
            }
        })
        .collect()
}

fn sum_multipliers(input: Vec<Value>, dodont: bool) -> usize {
    let mut enabled = true;
    let mut sum = 0;
    for val in input.iter() {
        match val {
            Value::Dont => {
                enabled = !dodont;
            }
            Value::Do => enabled = true,
            Value::Mul(a, b) => {
                if enabled {
                    println!("a: {}, b: {}, a+b={}, sum: {}", a, b, a + b, sum);
                    sum += a * b;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod test {

    #[test]
    fn test_mul() {
        const INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(
            super::sum_multipliers(super::find_multipliers(INPUT), false),
            161
        );
    }

    #[test]
    fn test_do_mul() {
        const INPUT: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(
            super::sum_multipliers(super::find_multipliers(INPUT), true),
            48
        );
    }
}

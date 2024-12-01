use std::collections::HashMap;

fn main() {
    let input = advent::input_from_args();

    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = input
        .map(|line| line.expect("error reading lines"))
        .map(|line| {
            let split = line.split_ascii_whitespace();
            let mut nums = split.map(|n| n.parse::<i32>().expect("error parsing numbers"));
            (
                nums.next().expect("no first number"),
                nums.next().expect("no second number"),
            )
        })
        .unzip();

    list1.sort();
    list2.sort();

    let mut distance = 0;
    for (a, b) in list1.iter().zip(list2.iter()) {
        let max = a.max(b);
        let min = a.min(b);
        distance += max - min;
    }

    println!("total distance: {}", distance);

    let freqs = freq(&list2);
    let similarity = list1.into_iter().fold(0, |mut acc, n| {
        let count = match freqs.get(&n) {
            Some(count) => *count,
            None => 0,
        };
        acc += count * n;

        acc
    });

    println!("similarity: {}", similarity);
}

fn freq(nums: &Vec<i32>) -> HashMap<i32, i32> {
    let mut freqs = HashMap::new();
    for n in nums {
        let count = freqs.entry(*n).or_insert(0);
        *count += 1;
    }

    freqs
}

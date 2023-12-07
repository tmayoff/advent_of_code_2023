pub trait Runner {
    fn run(input: &str) -> u64;
}

pub fn get_number(n_str: &str) -> u64 {
    assert!(!n_str.is_empty());

    let mut num = 0;
    for c in n_str.chars() {
        num = (num * 10) + c.to_digit(10).unwrap() as u64;
    }

    num
}

pub fn get_number_from_list(list: &str) -> Vec<u64> {
    let num_str = list.split(' ').filter(|n| !n.is_empty());

    let mut nums = Vec::new();
    for s in num_str {
        nums.push(get_number(s));
    }

    nums
}

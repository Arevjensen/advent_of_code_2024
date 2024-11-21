pub fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

pub fn gcd_vec(nums: Vec<usize>) -> usize {
    nums.iter().fold(nums[0], |acc, &x| gcd(acc, x))
}

pub fn lcm_slice(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm_slice(&nums[1..]);
    a * b / gcd(a, b)
}

fn lcm(numbers: Vec<u64>) -> u64 {
    let mut temp = numbers.clone();

    // check all the same
    loop {
        let mut same = true;

        for idx in 1..temp.len() {
            if temp[0] != temp[idx] {
                same = false;
                break;
            }
        }

        if same {
            return temp[0];
        }

        // Find lowest index
        match temp
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
        {
            Some(idx) => {
                temp[idx] += numbers[idx];
            }
            None => panic!("Not possible"),
        }
    }
}

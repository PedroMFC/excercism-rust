use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors.iter().map(|factor| {
        let mut set = HashSet::new();
        if *factor == 0 {
            return set;
        }
        let mut num = *factor;
        while num < limit {
            set.insert(num);
            num += factor;
        }
        set
    }).fold(HashSet::new(), |acc, set| acc.union(&set).cloned().collect()).iter().sum()

}

// More efficient solution
// pub fn sum_of_multiples(limit: u32, divs: &[u32]) -> u32 {
//     (1..limit)
//         .filter(|x| divs.iter().any(|d| *d != 0 && x % d == 0))
//         .sum()
// }

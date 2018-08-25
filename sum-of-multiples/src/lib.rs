//extern crate time;
//use time::PreciseTime;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    //let start = PreciseTime::now();

    let res_one = sum_one(limit, factors);
    //let end_one = PreciseTime::now();

    let res_two = sum_two(limit, factors);
    //let end_two = PreciseTime::now();

    let res_three = sum_three(limit, factors);
    //let end_three = PreciseTime::now();

    //println!("{} seconds for one, {} seconds for two, {} seconds for three",
    //         start.to(end_one), end_one.to(end_two), end_two.to(end_three));

    assert_eq!(res_one, res_two);
    assert_eq!(res_two, res_three);

    res_one
}

// should be faster for many small factors
pub fn sum_one(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = vec![];

    for res in 1u32..limit {
        for fac in factors {
            if res % fac == 0 {
                multiples.push(res);
                break;
            }
        }
    }

    multiples.iter().sum()
}

// should be faster for less, bigger factors
pub fn sum_two(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = vec![];

    for fac in factors {
        // because we know we will be multiplying by fac we can stop when the coefficient * fac is limit-1
        for coefficient in 1u32..=((limit-1)/fac) {
            multiples.push(coefficient * fac);
        }
    }
    multiples.sort_unstable();
    multiples.dedup();

    multiples.iter().sum()
}

// using the nifty iter function it seems like it could be more optimized
// as there is nothing mutable here it could be executed on multiple threads at the same time
pub fn sum_three(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).filter(|i| factors.iter().any(|fac| i % fac == 0)).sum()
}
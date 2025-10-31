// Tiny Data Lab — student template
// Implement the functions below. You may add small private helpers if you like.

pub fn sum_i32(arr: &[i32; 8]) -> i32 {
    // TODO: sum all values using a loop

    let mut sum = 0;

    for &x in arr {
        sum += x;
    }

    sum
}

pub fn min_max_i32(arr: &[i32; 8]) -> (i32, i32) {
    // TODO: return (min, max)
    let mut min = arr [0];
    let mut max = arr [0];

    for &x in arr.iter() {
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
    }

    (min, max)
}


pub fn mean_f64(arr: &[f64; 6]) -> f64 {
    // TODO: compute average    
    let mut sum = 0.0;
    for &x in arr.iter() {
        sum +=x;
    }
    sum / arr.len() as f64 
}

pub fn normalize_minmax_f64(arr: &[f64; 6]) -> [f64; 6] {
    // TODO: normalize to [0,1] using (x - min) / (max - min); handle flat case
    let mut result = [0.0; 6];
    let mut min = arr[0];
    let mut max = arr[0];

    // find min and max
    for &x in arr.iter() {
        if x < min { min = x; }
        if x > max { max = x; }
    }

    // if all values are equal, return zeros
    if min == max {
        return result;
    }

    // normalize each element
    for (i, &x) in arr.iter().enumerate() {
        result[i] = (x - min) / (max - min);
    }

    result  // return after loop
}

pub fn clamp_u8(arr: &mut [u8; 10], low: u8, high: u8) {
    // TODO: mutate arr so each value is within [low, high]
    for x in arr.iter_mut() {
        if *x < low {
            *x = low; 
        } else if *x > high {
            *x = high;
        }
    }
}

pub fn histogram3(arr: &[i32; 12], thresholds: (i32, i32)) -> [usize; 3] {
    // TODO: return three counts: < t1, between [t1,t2], > t2
    let (t1, t2) = thresholds;
    let mut counts = [0usize; 3];

    for &x in arr.iter() {
        if x < t1 {
            counts [0] += 1;
        } else if x <= t2 {
            counts [1] += 1;
        } else {
            counts [2] += 1;
        }
    }
    counts 
}

pub fn label_parity_sign(n: i32) -> &'static str {
    // TODO: "neg-even" | "neg-odd" | "zero" | "pos-even" | "pos-odd"
    if n == 0 {
        "zero"
    } else if n > 0 {
        if n % 2 == 0 { "pos-even" } else { "pos-odd" }
    } else {
        if n % 2 == 0 { "neg-even" } else { "neg-odd" }
    }
}


pub fn parse_grade(letter: char) -> u8 {
    // TODO: map letters to 4/3/2/1/0 as described
    match letter {
        'A' | 'a' => 4,
        'B' | 'b' => 3,
        'C' | 'c' => 2,
        'D' | 'd' => 1,
        'F' | 'f' => 0,
        _ => 0,

    }
}

// adding tests

mod tests {
    use super::*;

    #[test]
    fn test_sum_i32() {
        assert_eq!(sum_i32(&[1,2,3,4,5,6,7,8]), 36);
    }

    #[test]
    fn test_min_max_i32() {
        assert_eq!(min_max_i32(&[-3, 2, 10, -5, 4, 6, 0, 1]), (-5, 10));
    }

    #[test]
    fn test_mean_f64() {
        let arr = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        assert!((mean_f64(&arr) - 3.5).abs() < 1e-12);
    }

    #[test]
    fn test_normalize_minmax_f64() {
        let arr = [2.0, 4.0, 6.0, 8.0, 10.0, 12.0];
        let expected = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
        let res = normalize_minmax_f64(&arr);
        for i in 0..6 { assert!((res[i] - expected[i]).abs() < 1e-12); }

        let flat = [5.0; 6];
        assert_eq!(normalize_minmax_f64(&flat), [0.0; 6]);
    }

    #[test]
    fn test_clamp_u8() {
        let mut arr = [0u8, 10, 20, 30, 40, 50, 200, 255, 128, 5];
        clamp_u8(&mut arr, 10, 100);
        assert!(arr.iter().all(|&x| x >= 10 && x <= 100));
    }

    #[test]
    fn test_histogram3() {
        let arr = [-10, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let counts = histogram3(&arr, (0, 4));
        // <0 : -10, -1  => 2
        // 0..=4 : 0,1,2,3,4 => 5
        // >4 : 5,6,7,8,9 => 5
        assert_eq!(counts, [2, 5, 5]);
    }

    #[test]
    fn test_label_parity_sign() {
        assert_eq!(label_parity_sign(0), "zero");
        assert_eq!(label_parity_sign(7), "pos-odd");
        assert_eq!(label_parity_sign(8), "pos-even");
        assert_eq!(label_parity_sign(-3), "neg-odd");
        assert_eq!(label_parity_sign(-4), "neg-even");
    }

    #[test]
    fn test_parse_grade() {
        assert_eq!(parse_grade('A'), 4);
        assert_eq!(parse_grade('b'), 3);
        assert_eq!(parse_grade('C'), 2);
        assert_eq!(parse_grade('d'), 1);
        assert_eq!(parse_grade('f'), 0);
        assert_eq!(parse_grade('Z'), 0);
    }
}

// You may add tests here (they won't be run by autograder). See tests/tests.rs instead.

///This function returns the double of the i32 parameter by multiplying it by 2.
pub fn double_v1(n: i32) -> i32 {
    n * 2
}

///This function returns the double of the i32 parameter by creating a new i32 to hold the
///value of the doubled parameter, and returning this as an i64.
pub fn double_v2(n: &i32) -> i64 {
    let newint = n * 2;
    newint as i64
}

///This function doubles the i32 pointer parameter by updating the value that the pointer references.
pub fn double_v3(n: &mut i32) {
    *n *= 2
}

//This function finds the largest integer that is a square root within the range of the parameter number.
pub fn sqrt(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let mut i = 1;
    let mut sqrt = i;
    while i * i <= n {
        sqrt = i;
        i += 1;
    }
    sqrt
}

///This function returns an array of size OUTSIZE with i32 values which are the fibbonacci numbers after the two parameter
///fibbonacci numbers that have been passed.
const OUTSIZE: usize = 5;
pub fn fibonacci(ns: (i32, i32)) -> [i32; OUTSIZE] {
    let start = ns.0 + ns.1;
    let mut arr: [i32; OUTSIZE] = [0; OUTSIZE];
    let mut count = 2;

    arr[0] = start;
    arr[1] = start + ns.1;

    while count < OUTSIZE {
        arr[count] = arr[count - 1] + arr[count - 2];
        count += 1;
    }

    arr
}

///This function takes an integer array and a range of usizes and returns either an error if the ranges are out of bounds,
/// or returns the slice that is within those ranges if they are in bounds
pub fn slice(arr: &[i32], range: (usize, usize)) -> Result<&[i32], &'static str> {
    let start = range.0;
    let end = range.1;

    if end > arr.len() {
        Err("OOB!")
    } else {
        Ok(&arr[start..end])
    }
}

///This function uses a binary search algorithm to find a given element in the array paramter and return
/// its location within the array as an option, or returns None if the element is not in the array. This
/// functio uses Ordering to compare the middle element and the query element in order to pass the cargo clippy command
use std::cmp::Ordering;
pub fn binary_search(arr: &[i32], query: i32) -> Option<usize> {
    let mut start = 0;
    let mut end = arr.len();
    while start <= end && (start + end) / 2 < arr.len() {
        let middle = (start + end) / 2;
        match arr[middle].cmp(&query) {
            Ordering::Less => start = middle + 1,
            Ordering::Greater => {
                if middle == 0 {
                    return None;
                } else {
                    end = middle - 1
                }
            }
            Ordering::Equal => return std::option::Option::Some(middle),
        }
    }
    None
}

///These are tests for each function
#[cfg(test)]
#[test]
fn test_double1() {
    assert_eq!(double_v1(100000), 200000);
    assert_eq!(double_v1(9), 18);
    assert_eq!(double_v1(0), 0);
}

#[test]
fn test_double2() {
    assert_eq!(double_v2(&100000), 200000);
    assert_eq!(double_v2(&9), 18);
    assert_eq!(double_v2(&0), 0);
}

#[test]
fn test_double3() {
    let mut db = 100000;
    double_v3(&mut db);
    assert_eq!(db, 200000);
    db = 9;
    double_v3(&mut db);
    assert_eq!(db, 18);
    db = 0;
    double_v3(&mut db);
    assert_eq!(0, 0);
}

#[test]
fn test_sqrt() {
    assert_eq!(sqrt(36), 6);
    assert_eq!(sqrt(0), 0);
    assert_eq!(sqrt(20), 4);
    assert_eq!(sqrt(2), 1);
}

#[test]
fn test_fib() {
    assert_eq!([1, 2, 3, 5, 8], fibonacci((0, 1)));
    assert_eq!([2584, 4181, 6765, 10946, 17711], fibonacci((987, 1597)));
}

#[test]
fn test_slice() {
    let arr: &[i32] = &[0, 1, 2, 3, 4, 5, 6, 7];
    assert_eq!(Ok(&arr[1..5]), slice(&[0, 1, 2, 3, 4, 5, 6, 7], (1, 5)));
    assert_eq!(Err("OOB!"), slice(&arr, (1, 100)));
}

#[test]
fn test_bs() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(std::option::Option::Some(0), binary_search(&arr, 1));
    assert_eq!(std::option::Option::Some(1), binary_search(&arr, 2));
    assert_eq!(std::option::Option::Some(2), binary_search(&arr, 3));
    assert_eq!(std::option::Option::Some(3), binary_search(&arr, 4));
    assert_eq!(std::option::Option::Some(4), binary_search(&arr, 5));
    assert_eq!(std::option::Option::Some(5), binary_search(&arr, 6));
    assert_eq!(std::option::Option::Some(6), binary_search(&arr, 7));
    assert_eq!(std::option::Option::Some(7), binary_search(&arr, 8));

    assert_eq!(std::option::Option::None, binary_search(&arr, 1000));
    assert_eq!(std::option::Option::None, binary_search(&arr, 0));
}

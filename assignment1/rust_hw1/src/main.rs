pub mod lib;

fn main() {
    println!("Double of {} is {}", 3, lib::double_v1(3));
    println!("fib after 0, 1 is {:?}", lib::fibonacci((0, 1)));
    println!(
        "found 4 in {:?} at {:?}",
        &[2, 4, 6, 8],
        lib::binary_search(&[2, 4, 6, 8], 4)
    );
    println!(
        "slice of {:?} from 1 to 3 is {:?}",
        &[1, 2, 3, 4, 5, 6],
        lib::slice(&[1, 2, 3, 4, 5, 6], (1, 3))
    );
}

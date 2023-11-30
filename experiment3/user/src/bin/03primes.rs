#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;


fn prime(x: i32) -> bool {
    let flag:bool=false;
    for i in 2..= x {
        if i*i>x {break;}
        if x%i == 0 {
            return false;
        }
    }
    return true;
}

#[no_mangle]
fn main() -> i32 {
    let n = 1000;
    println!("primes <= {} : ",n);
    for i in 2..=n {
        if prime(i) {
            print!("{i} ")
        }
    }
    0
}

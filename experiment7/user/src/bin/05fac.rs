
   
#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const LEN: usize = 100;

static mut S: [u64; LEN] = [0u64; LEN];

#[no_mangle]
unsafe fn main() -> i32 {
    let m = 1000000007u64;
    let iter: usize = 300000;
    let mut cur = 0usize;
    S[cur] = 1;
    for i in 1..=iter {
        let next = if cur + 1 == LEN { 0 } else { cur + 1 };
        S[next] = S[cur] * i as u64 % m;
        cur = next;
        if i % 10000 == 0 {
            println!("fac [{}/{}]", i, iter);
        }
    }
    println!("{}! = {}(MOD {})", iter, S[cur], m);
    println!("Test fac OK!");
    0
}

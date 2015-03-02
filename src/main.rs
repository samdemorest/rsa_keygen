extern crate num;
extern crate "rustc-serialize" as rustc_serialize;
extern crate rand;
use num::bigint::BigUint;
use num::bigint;
use num::bigint::{ToBigUint, RandBigInt};
use std::fmt;
use std::old_io as io;
use std::old_io::File;
use rustc_serialize::base64::*;
use std::num::FromPrimitive;

use num::bigint::BigInt::*;
use std::{env};

fn main() {
    let mut bit_size: usize;
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        bit_size = args[1].parse().unwrap();
    } else {
        panic!("Needs an argument for key bit size!");
    }
    let mut optional: num::bigint::BigUint;
    let mut bignum: num::bigint::BigUint;
    let mut rnjesus = rand::thread_rng();
    bignum = rnjesus.gen_biguint(bit_size);
    let mut big_vec: Vec<num::bigint::BigUint> = Vec::new();
    println!("Big number incoming: {}", bignum);
    for _ in 0..100 {
        big_vec.push(rnjesus.gen_biguint(bit_size));
    }
    write_bnum(bignum);
    println!("Option unwrap: {}", BigUint::from_u64(420).unwrap());

}

/**
 * This function writes the bytes of a BigUint to a file.
 */
fn write_bnum(bignum: num::bigint::BigUint){
    let mut file = File::create(&Path::new("outfile"));
    let testnum = [65, 65, 65, 65];
    let bignum_bytes: Vec<u8> = bignum.to_bytes_le();
    println!("{}", testnum.to_base64(STANDARD));
    file.write_all(b"ssh-rsa ");
    let bigslice = bignum_bytes.as_slice();
    println!("{}",bigslice.to_base64(STANDARD));
    let formatted = format!("{}", bigslice.to_base64(STANDARD));
    file.write_all(formatted.as_bytes());
    
}

fn check_primality(bignum: num::bigint::BigUint) -> bool {
    let mut q: num::bigint::BigUint;   
    let mut r: num::bigint::BigUint;   
    let mut m: num::bigint::BigUint;   
    let mut isprime: bool;
    

    return false;
}

/*
 * Was going to write GCD function, but it is already implemented for BigUint type.
 */
/*fn gcd(bignum: num::bigint::BigUint, div: num::bigint::BigUint) 
    -> num::bigint::BigUint {
}*/

#![feature(core)]
extern crate num;
extern crate "rustc-serialize" as rustc_serialize;
extern crate rand;
extern crate core;
use num::bigint::BigUint;
use num::bigint;
use num::traits::*;
use num::integer::Integer;
use num::bigint::{ToBigUint, RandBigInt};
use std::fmt;
use std::old_io as io;
use std::old_io::File;
use rustc_serialize::base64::*;
use std::num::FromPrimitive;
use std::num::ToPrimitive;
use std::num::Float;
use std::usize;
use core::ops::*;
use rand::Rng;

use num::bigint::BigInt::*;
use std::{env};

/*
 * Main driver for the program. Lots of experimentation going on here on how to do various tasks
 * using the BigUint type, and Rust-learning things, too.
 */
fn main() {
    let mut bit_size: usize;
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        bit_size = args[1].parse().unwrap();
    } else {
        panic!("Needs an argument for key bit size!");
    }
    let max_exp: u64 = 
        bit_size.add(1.to_usize().unwrap()).to_f64().unwrap().log2().ceil().to_u64().unwrap();
    let mut optional: BigUint;
    let mut bignum: BigUint;
    let mut rnjesus = rand::thread_rng();
    bignum = rnjesus.gen_biguint(bit_size);
    let mut big_vec: Vec<BigUint> = Vec::new();
    println!("Big number incoming: {}", bignum);

    let copied: BigUint = bignum.clone();
    let another: BigUint = BigUint::from_u32(69).unwrap();
    //write_bnum(bignum.clone());
    println!("Even? {}", bignum.is_even());
    println!("Prime? {}", check_primality(bignum.clone(), max_exp));

}

/**
 * This function writes the bytes of a BigUint to a file.
 */
fn write_bnum(bignum: BigUint){
    let mut file = File::create(&Path::new("outfile"));
    let testnum = [65, 65, 65, 65];
    let bignum_bytes: Vec<u8> = bignum.to_bytes_le();
    //println!("{}", testnum.to_base64(STANDARD));
    file.write_all(b"ssh-rsa ");
    let bigslice = bignum_bytes.as_slice();
    //println!("{}",bigslice.to_base64(STANDARD));
    let formatted = format!("{}", bigslice.to_base64(STANDARD));
    file.write_all(formatted.as_bytes());
    
}

fn check_primality(bignum: BigUint, max_exp: u64) -> bool {
    let mut q: BigUint;   
    let mut r: BigUint;   
    let mut m: BigUint;   
    let mut isprime: bool;

    // for k, j <= lg(sizeof(bignum))
    // if k^j == bignum return false
    let mut iterator: BigUint = BigUint::from_usize(2).unwrap();
    let mut r = iterator.clone();
    if bignum.is_even(){
        return false;
    }
    while r < bignum{
       if r.gcd(&bignum.clone()) != BigUint::one(){
           return false;
       }
       r = r.clone().add(BigUint::one());
    }
    println!("{}", max_exp);
    while iterator < bignum{
        if num::pow(iterator.clone(), 2.to_usize().unwrap()).gt(&bignum.clone()){
            return false;
        }
        for j in 2..max_exp {
            if num::pow(iterator.clone(), j.to_usize().unwrap()).gt(&bignum.clone()){
                break;
            } else if num::pow(iterator.clone(), j.to_usize().unwrap()).eq(&bignum.clone()) {
                return false;
            }
        }
        /*
         * Have to do this temp swap here because iterator is moved when we enter the loop,
         * and the BigUint type is not copyable.
         */
        let temp: BigUint = iterator.clone().add(BigUint::one());
        iterator = temp;
    }
    return false;
}

/*
 * This function is used in the AKS Primality Test, in which the first part
 * of the sieve determines whether or not our number being tested is a power
 * of another number, in which case our number will not be prime.
 *
 * ACTUALLY, this is unnecessary, as we can do what this functoin does and more in the prime
 * testing function.
 */
fn is_perfect_power(n: BigUint, k: BigUint, j: usize) -> bool {
    if n == num::pow(k, j){
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    extern crate num;
    use super::is_perfect_power;
    use std::num::FromPrimitive;
    use std::num::ToPrimitive;
    use num::bigint::BigUint;
    use super::check_primality;

    #[test]
    fn test_perfect_power(){
        assert_eq!(is_perfect_power(BigUint::from_i32(1024).unwrap(),
                    BigUint::from_i32(2).unwrap(), 10.to_usize().unwrap()), true);
    }

    #[test]
    fn test_prime_check(){
        assert_eq!(check_primality(BigUint::from_u64(1024).unwrap(), 10), false);
    }

}

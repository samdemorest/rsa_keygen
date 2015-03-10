#![feature(core)]
#![feature(old_io)]
#![feature(old_path)]
extern crate num;
extern crate "rustc-serialize" as rustc_serialize;
extern crate rand;
extern crate core;
use num::traits::*;
use num::integer::Integer;
use num::bigint::{BigUint, ToBigUint, RandBigInt};
use std::num::{FromPrimitive, ToPrimitive, Float};
use std::old_io as io;
use std::old_io::File;
use rustc_serialize::base64::*;
use rand::Rng;
use core::ops::*;
use std::{env, fmt, usize};

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
    //gen_large_prime(bit_size);
    bigint_exp(BigUint::from_usize(2).unwrap(), BigUint::from_usize(8).unwrap());

}

fn gen_large_prime(bit_size: usize){
    let mut bignum: BigUint;
    let mut rnjesus = rand::OsRng::new().unwrap();
    let mut is_prime: bool = false;
    let max_exp: u64 = 
        bit_size.add(1.to_usize().unwrap()).to_f64().unwrap().log2().ceil().to_u64().unwrap();
    
    while is_prime  == false{
       bignum = rnjesus.gen_biguint(bit_size);
       if bignum.clone().is_even(){
           continue;
       } 
       /*
        * 5 should be a sufficient number of runs for the Miller-Rabin test. The greatest
        *   possibility that the test gives an inaccurate result is 25%. 0.25^5 = 0.0009765, which
        *   is an acceptable threshold of accuracy.
        */
       is_prime = miller_rabin(bignum.clone(), 5);
       if is_prime{
           println!("Prime discovered: {}", bignum.clone());
       }
    }

}


fn miller_rabin(bignum: BigUint, num_runs: usize) -> bool {
    let mut rnjesus = rand::OsRng::new().unwrap();
    let mut d: BigUint = bignum.clone().sub(BigUint::one());
    let mut a: BigUint;
    let mut x: BigUint;
    let mut s: usize = 0;
    while d.is_even() {
        s += 1;
        d = d.clone().div(BigUint::from_usize(2).unwrap());
    }
    for i in 0..num_runs {
        a = rnjesus.gen_biguint_range(&BigUint::from_usize(2).unwrap(), 
                                                   &bignum.clone().sub(BigUint::one()));
        x = bigint_exp(a.clone(), d.clone());


    }
    return false;
}

fn mod_exp(d: BigUint, n: BigUint) -> BigUint{
    let mut retval: BigUint;

    return n;
}

fn bigint_exp(base: BigUint, pow: BigUint) -> BigUint{
    let b: BigUint = base.clone();
    let mut retval: BigUint = base.clone();
    let mut counter: BigUint = BigUint::one();
    while counter.clone().lt(&pow.clone()){
        retval = retval.clone().mul(b.clone());
        counter = counter.clone().add(BigUint::one());
    }
    return retval;
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
    // Start iterator at 3, since if bignum is even, it won't be prime.
    let mut iterator: BigUint = BigUint::from_usize(3).unwrap();
    let mut r = iterator.clone();

    while r < bignum{
       if r.gcd(&bignum.clone()) != BigUint::one(){
           return false;
       }
       r = r.clone().add(BigUint::from_usize(2).unwrap());
    }
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
        iterator = iterator.clone().add(BigUint::one());
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
    use super::{bigint_exp, check_primality, is_perfect_power};
    use std::num::{FromPrimitive, ToPrimitive};
    use num::bigint::BigUint;

    #[test]
    fn test_perfect_power(){
        assert_eq!(is_perfect_power(BigUint::from_i32(1024).unwrap(),
                    BigUint::from_i32(2).unwrap(), 10.to_usize().unwrap()), true);
    }

    #[test]
    fn test_prime_check(){
        assert_eq!(check_primality(BigUint::from_u64(1024).unwrap(), 10), false);
    }

    #[test]
    fn test_bigint_ext(){
        assert_eq!(bigint_exp(BigUint::from_usize(2).unwrap(), 
                              BigUint::from_usize(8).unwrap()),
                              BigUint::from_usize(256).unwrap());
    }
}

#![feature(core)]
#![feature(convert)]
#![feature(collections)]
extern crate num;
extern crate rustc_serialize;
extern crate rand;
extern crate core;
use num::traits::*;
use num::integer::Integer;
use num::bigint::{BigUint, ToBigUint, RandBigInt};
use std::io::prelude::*;
use std::collections::BitVec;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use rustc_serialize::base64::*;
use rand::Rng;
use core::ops::*;
use std::{env, fmt, usize};
use num::bigint::BigInt::*;

/**
 * Global variable definitions
 */


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
    gen_large_prime(bit_size);
    //bigint_exp(BigUint::from_usize(2).unwrap(), BigUint::from_usize(8).unwrap());

}

fn gen_large_prime(bit_size: usize){
    let mut bignum: BigUint;
    let mut rnjesus = rand::OsRng::new().unwrap();
    let mut is_prime: bool = false;
    let ONE: BigUint = BigUint::one();
    //let max_exp: u64 = 
    //    bit_size.add(1.to_usize().unwrap()).to_f64().unwrap().log2().ceil().to_u64().unwrap();
    
    while !is_prime{
       bignum = rnjesus.gen_biguint(bit_size).bitor(&ONE.clone().shl(bit_size - 1));
       if bignum.clone().is_even(){
           println!("Even number discarded...");
           continue;
       } 
       /*
        * 5 should be a sufficient number of runs for the Miller-Rabin test. The greatest
        *   possibility that the test gives an inaccurate result is 25%. 0.25^5 = 0.0009765, which
        *   is an acceptable threshold of accuracy.
        */
       println!("Getting value for is_prime via Miller-Rabin");
       is_prime = miller_rabin(bignum.clone(), 5);
       if is_prime{
           println!("Prime discovered: {}", bignum.clone());
       }
    }

}

/**
 * The Miller-Rabin Primality Test implemented. Seems to return prime numbers.
 */
fn miller_rabin(bignum: BigUint, num_runs: usize) -> bool {
    let ONE = BigUint::one();
    let ZERO = BigUint::zero();
    let TWO = BigUint::from_usize(2).unwrap();
    println!("Entering miller-rabin");
    let mut rnjesus = rand::OsRng::new().unwrap();
    let k: usize = num_runs;
    let mut a: BigUint = ZERO.clone(); 
    let mut x: BigUint;
    let mut rval: usize = 0;
    println!("Getting d and s.");
    let (d,s) = get_ds(bignum.clone());
    for i in 1..k{
        println!("i = {} in 1..k", i);
        /* TODO: Got a panic coming from the line below. bignum must be > 4. This shouldn't be a
         * problem, but to keep the program from barfing all over everything, we really should
         * check the value of bignum before we send it down here.
         */
        if bignum.lt(&BigUint::from_usize(5).unwrap()){
            // Just to solve the above panic for now, for testing very small values.
            return false;
        }
        a = rnjesus.gen_biguint_range(&BigUint::from_usize(2).unwrap(), 
                                      &bignum.clone().sub(&TWO));
        println!("found a: {}", a);
        x = mod_exp(a.clone(), d.clone(), bignum.clone());
        println!("\n\nFinished mod_exp");
        if x.ne(&ONE) && x.ne(&bignum.clone().sub(&ONE)){
            for r in 1..s{
                println!("r = {}, s = {}", r, s);
                x = mod_exp(x, TWO.clone(), bignum.clone());
                if x.eq(&BigUint::one()){
                    println!("Rejected in for r = 1..s loop");
                    return false
                } else if x.eq(&bignum.clone().sub(&ONE)){
                    a = ZERO.clone();
                    println!("x = n-1. Loop break. a = 0.");
                    break;
                }
                rval = r;
            } // end for r in 1..s
            if a.ne(&BigUint::zero()){
                return false
            }
        }
    }
    return true
}

/**
 * Fast modulo exponentiation as described in algorithm 11.4 of Neapolitan
 * modulo cannot be 1 or we will have problems.
 * TODO: Make this not have problems.
 */
fn mod_exp(base: BigUint, power: BigUint, modulo: BigUint) -> BigUint{
    let b: BitVec = BitVec::from_bytes(&power.to_bytes_le());
    let mut a: BigUint = BigUint::one();
    for i in 0..b.len(){
        a = a.clone().mul(&a).mod_floor(&modulo);
        if b[i]{
            a = a.mul(&base).mod_floor(&modulo);
        }
    }
    return a;
}

/**
 * Gets the result of [base_modulo]^power
 * TODO: This needs to be optimized if this is going to be a practical key generator.
 */
fn old_mod_exp(base: BigUint, power: BigUint, modulo: BigUint) -> BigUint{
    let ONE = BigUint::one();
    let mut retval: BigUint = power.clone();
    let mut c: BigUint = ONE.clone(); 
    let mut i: BigUint = ONE.clone();
    while i.le(&power){
        c = (base.clone().mul(c)).mod_floor(&modulo);
        i = i.add(&ONE);
    }

    return c;
}

/**
 * Raises a BigUint to the power of another BigUint
 */
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
 * Get d and s for use in calculation in Miller-Rabin test
 */
fn get_ds(bignum: BigUint) -> (BigUint, usize){
    let ONE = BigUint::one();
    let ZERO = BigUint::zero();
    println!("In get_ds");
    let mut test = bignum.clone().sub(&ONE);
    let mut s: usize = 0;
    /*
     * TODO: Sometimes this will hit zero when n-1 is even. Gotta figure this one out.
     */
    /*while test.is_even(){
        println!("In loop determining if 'test' is even: {}", test);
        test = test.div(BigUint::from_usize(2).unwrap());
        s += 1;
    }*/
    while test.clone().bitand(&ONE) == ZERO{
        println!("Test: {}", &test);
        test = test.shr(1);
        s += 1;
    }
    return (test.clone(), s)
}

/**
 * This function writes the bytes of a BigUint to a file.
 */
fn write_bnum(bignum: BigUint){
    let path = Path::new("outfile");
    let mut f = match File::create(&path){
        Err(why) => panic!("Couldn't create {}: {}",
                           path.display(),
                           Error::description(&why)),
        Ok(f) => f,
    };
    let testnum = [65, 65, 65, 65];
    let bignum_bytes: Vec<u8> = bignum.to_bytes_le();
    //println!("{}", testnum.to_base64(STANDARD));
    match f.write_all(b"ssh-rsa "){
        Err(why) => panic!("Couldn't write: {}",
                           Error::description(&why)),
        Ok(res) => res,
    };
    let bigslice = bignum_bytes.as_slice();
    //println!("{}",bigslice.to_base64(STANDARD));
    let formatted = format!("{}\n", bigslice.to_base64(STANDARD));
    match f.write_all(formatted.as_bytes()){
        Err(why) => panic!("Couldn't write: {}",
                           Error::description(&why)),
        Ok(res) => res,
    };
    
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


#[cfg(test)]
mod tests {
    extern crate num;
    use super::{bigint_exp, check_primality, is_perfect_power, mod_exp};
    use num::bigint::BigUint;
    use num::traits::*;

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

    #[test]
    fn test_mod_exp(){
        assert_eq!(mod_exp(BigUint::from_usize(5).unwrap(), 
                           BigUint::from_usize(45).unwrap(),
                           BigUint::from_usize(257).unwrap()),
                           BigUint::from_usize(147).unwrap());
    }
}

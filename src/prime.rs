#![feature(core)]
extern crate num;
extern crate rustc_serialize;
extern crate rand;
extern crate core;
use num::traits::*;
use num::integer::Integer;
use num::bigint::{BigUint, RandBigInt};
use core::ops::*;



pub fn gen_large_prime(bit_size: usize) -> num::bigint::BigUint{
    let mut bignum = BigUint::parse_bytes("1337".as_bytes(),10).unwrap();
    let mut rnjesus = rand::OsRng::new().unwrap();
    let mut is_prime: bool = false;
    let one: BigUint = BigUint::one();
    
    while !is_prime{
       bignum = rnjesus.gen_biguint(bit_size).bitor(&one.clone().shl(bit_size - 1));
       if bignum.clone().is_even(){
           continue;
       } 
       /*
        * 5 should be a sufficient number of runs for the Miller-Rabin test. The greatest
        *   possibility that the test gives an inaccurate result is 25%. 0.25^5 = 0.0009765, which
        *   is an acceptable threshold of accuracy.
        */
       is_prime = miller_rabin(bignum.clone(), 5);
    }

    return bignum;

}

/**
 * The Miller-Rabin Primality Test implemented. Seems to return prime numbers.
 */
fn miller_rabin(bignum: BigUint, num_runs: usize) -> bool {
    let one = BigUint::one();
    let zero = BigUint::zero();
    let two = BigUint::from_usize(2).unwrap();
    let mut rnjesus = rand::OsRng::new().unwrap();
    let k: usize = num_runs;
    let mut a: BigUint;
    let mut x: BigUint;
    let (d,s) = get_ds(bignum.clone());
    for _ in 1..k{
        /* TODO: Got a panic coming from the line below. bignum must be > 4. This shouldn't be a
         * problem, but to keep the program from barfing all over everything, we really should
         * check the value of bignum before we send it down here.
         */
        if bignum.lt(&BigUint::from_usize(5).unwrap()){
            // Just to solve the above panic for now, for testing very small values.
            return false;
        }
        a = rnjesus.gen_biguint_range(&BigUint::from_usize(2).unwrap(), 
                                      &bignum.clone().sub(&two));
        x = schneier_mod_exp(a.clone(), d.clone(), bignum.clone());
        if x.ne(&one) && x.ne(&bignum.clone().sub(&one)){
            for _ in 1..s{
                x = schneier_mod_exp(x, two.clone(), bignum.clone());
                if x.eq(&BigUint::one()){
                    return false
                } else if x.eq(&bignum.clone().sub(&one)){
                    a = zero.clone();
                    break;
                }
            } // end for r in 1..s
            if a.ne(&BigUint::zero()){
                return false
            }
        }
    }
    return true
}

/**
 * Modulo Exponentiation based on Bruce Schneier's 
 * algorithm discussed in his book Applied Cryptography
 */
pub fn schneier_mod_exp(base: BigUint, power: BigUint, modulo: BigUint) -> BigUint{
    let mut result = BigUint::one();
    let mut pow = power.clone();
    let mut basic = base.clone();
    let two = BigUint::from_usize(2).unwrap();
    let zero = BigUint::zero();
    while pow.gt(&zero){
        if pow.clone().rem(&two).eq(&BigUint::one()){
            result = (result.mul(&basic)).rem(&modulo);
        }
        pow = pow.clone().shr(1);
        basic = basic.clone().mul(&basic).rem(&modulo);
    }
    return result;
}

/**
 * Gets the result of [base_modulo]^power
 * TODO: This needs to be optimized if this is going to be a practical key generator.
 */
pub fn naive_mod_exp(base: BigUint, power: BigUint, modulo: BigUint) -> BigUint{
    let one = BigUint::one();
    let mut c: BigUint = one.clone(); 
    let mut i: BigUint = one.clone();
    while i.le(&power){
        c = (base.clone().mul(c)).rem(&modulo);
        i = i.add(&one);
    }

    return c;
}

/**
 * Raises a BigUint to the power of another BigUint
 */
pub fn bigint_exp(base: BigUint, pow: BigUint) -> BigUint{
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
    let one = BigUint::one();
    let zero = BigUint::zero();
    let mut test = bignum.clone().sub(&one);
    let mut s: usize = 0;
    while test.clone().bitand(&one) == zero{
        test = test.shr(1);
        s += 1;
    }
    return (test.clone(), s)
}


#[cfg(test)]
mod tests {
    extern crate num;
    use super::{bigint_exp, check_primality,  mod_exp};
    use num::bigint::BigUint;
    use num::traits::*;

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
        assert_eq!(schneier_mod_exp(BigUint::from_usize(5).unwrap(), 
                           BigUint::from_usize(45).unwrap(),
                           BigUint::from_usize(257).unwrap()),
                           BigUint::from_usize(147).unwrap());
    }
}

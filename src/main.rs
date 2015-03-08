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



fn generate_large_number(bit_size: usize) -> num::bigint::BigUint{
    let mut bignum: num::bigint::BigUint;
    let mut randyjackson = rand::thread_rng();
    bignum = randyjackson.gen_biguint(bit_size);
	
	if check_primality(bignum) {
		return bignum;
	} else {
		return generate_large_number(bit_size);
	}
}


/**
 * This function writes the bytes of a BigUint to a file.
 */
fn write_bnum(bignum: num::bigint::BigUint){
    let mut file = File::create(&Path::new("outfile.txt"));
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

	//if its a small prime dont bother
	if bignum <= 3 {
		if bignum > 1 {
			return true;
		}
	} else { //divisable by two or three...come on now
		if ((bignum % 2 == 0) || (bignum % 3 == 0)) {
			return false;
		}
	}
	
	//setting up looping to check more possibilities
	let mut done = false;
	let mut i: int;
	
	i = 5;
	
	while !done{
		if (i * i) > bignum{
			done = true;
		} else {
			if bignum % i == 0 || bignum % (i + 2) == 0 {
				return false;
			} else {
				i += 6;
			}
		}
	}
	
	return true;
}


/**
 *  This function returns the euler's totient value based on given numbers p, q and n
 */
fn euler_totient(p: num::bigint::BigUint, q: num::bigint::BigUint, n: num::bigint::BigUint) -> num::bigint::BigUint {
	let mut euler: num::bigint::BigUint;
	euler = n - (p + q - 1);
	return euler;
}
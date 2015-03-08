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
    bignum = generate_large_prime_number(bit_size);
    let mut big_vec: Vec<num::bigint::BigUint> = Vec::new();
    println!("Big number incoming: {}", bignum);
    for _ in 0..100 {
        big_vec.push(rnjesus.gen_biguint(bit_size));
    }
    write_bnum(bignum);
    println!("Option unwrap: {}", BigUint::from_u64(420).unwrap());

}



fn generate_large_prime_number(bit_size: usize) -> num::bigint::BigUint{
    let mut bignum: num::bigint::BigUint;
    let mut randyjackson = rand::thread_rng();
    bignum = randyjackson.gen_biguint(bit_size);
	
	if check_primality(&bignum) {
		return bignum;
	} else {
		return generate_large_prime_number(bit_size);
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


/*
 * This function will test for 2 and 3 divisability before sending it to 
 * 	the aks function 
 */
fn check_primality(bignum: &num::bigint::BigUint) -> bool {

	let mut scalar_one: num::bigint::BigUint;
	let mut scalar_two: num::bigint::BigUint;
	
	scalar_one = BigUint::parse_bytes("3".as_bytes(),10).unwrap();
	scalar_two = BigUint::parse_bytes("2".as_bytes(),10).unwrap();
	
	//println!("{}",scalar_one);
	//println!("{}",scalar_two);
	
	println!("big num {}", bignum);
	
	//if its a small prime dont bother
	if * bignum <= scalar_one {
		scalar_one = BigUint::parse_bytes("1".as_bytes(),10).unwrap();
		if * bignum > scalar_one {
			return true;
		}
		scalar_one = BigUint::parse_bytes("3".as_bytes(),10).unwrap();
	} else { 	
	
		//divisable by two or three...come on now
		
		let mut temp = bignum;
		
		let mut result_one = temp % scalar_two;
		let mut result_two = temp % scalar_one;
		
		//println!("{}",result_one);
		//println!("{}",result_two);
		
		if (result_one == BigUint::parse_bytes("0".as_bytes(),10).unwrap() || ( result_two == BigUint::parse_bytes("0".as_bytes(),10).unwrap())) {
			return false;
		}
	}
	
	//This is where we would start using the aks system
	
	return aks_primality_test(bignum);
}

/**
 *   This is the aks primality testing area
 */
fn aks_primality_test(bignum: &num::bigint::BigUint) -> bool{
	return true;
} 

/**
 *  This function returns the euler's totient value based on given numbers p, q and n
 */
fn euler_totient(p: num::bigint::BigUint, q: num::bigint::BigUint, n: num::bigint::BigUint) -> num::bigint::BigUint {
  let mut euler: num::bigint::BigUint;
 	euler = n - (p + q - BigUint::parse_bytes("1".as_bytes(),10).unwrap());
 	return euler;
 }

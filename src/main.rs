#![feature(convert)]
#![feature(core)]
#![feature(collections)]
extern crate num;
extern crate rustc_serialize;
extern crate core;
extern crate rand;
use num::bigint::BigUint;
use num::bigint::BigInt;
use num::bigint;
use num::bigint::{ToBigUint, RandBigInt};
use std::fmt;
//use std::old_io as io;
use std::io;
use std::io::Stdin;
use std::io::stdin;
use std::fs::File;
use rustc_serialize::base64::*;
//use std::num::FromPrimitive;
use num::bigint::ToBigInt;
use num::traits::FromPrimitive;
use num::traits::{One, Zero};
use num::integer::Integer;
use core::ops::*;
use std::error::Error;
mod prime;

use std::{env};

fn main() {
    
    let mut bit_size: usize;
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        bit_size = args[1].parse().unwrap();
    } else {
        panic!("Needs an argument for key bit size!");
    }
    
    //Values used for testing
	//let mut p = BigUint::parse_bytes("61".as_bytes(),10).unwrap();
	//let mut q = BigUint::parse_bytes("53".as_bytes(),10).unwrap();
    let five = BigUint::from_usize(5).unwrap();
    let twofiveseven = BigUint::from_usize(257).unwrap();
    let fortyfive = BigUint::from_usize(45).unwrap();

    prime::schneier_mod_exp(five, fortyfive, twofiveseven);
	let mut p: BigUint = BigUint::zero();
	let mut q: BigUint = BigUint::zero();
	let mut d: BigUint = BigUint::zero();
    let mut n: BigUint= BigUint::zero();
    let mut totient: BigUint = BigUint::zero();
    let mut e: BigUint = BigUint::zero();
    while d.is_zero(){
        p = prime::gen_large_prime(bit_size);
        q = prime::gen_large_prime(bit_size);
        n = (&p) * (&q);
        totient = euler_totient(&p,&q,&n);
        e = generate_e((&totient), bit_size);
        d = inverse((&(e.to_bigint().unwrap())), ((&totient.to_bigint().unwrap())));
    }
	let mut reader = io::stdin();
    let mut stdin_buf: String = "".to_string();

    println!("Value of p = {}", p);
    println!("Value of q = {}", q);
    println!("Value of n = {}", n);
    println!("Value of totient = {}", totient);
    println!("Value of e = {}", e);
    println!("Value of d = {}", d);
	println!("Please Insert Letter To Encrypt");
	reader.read_line(&mut stdin_buf).ok().expect("Failed to Read Line");

    let send_str = String::from_str(stdin_buf.clone().trim());

	println!("You sent in {}", stdin_buf);

	let encrypted = encrypt(send_str, (&e), (&n));
	println!("Encryption Time = {}", encrypted);


	let decrypted = decrypt(encrypted.clone(), (&d), (&n));
	println!("Decryption Time = {}", decrypted);

}

fn generate_e(totient: &num::bigint::BigUint, bit_size: usize) -> num::bigint::BigUint {
	
	//This would get a number from sam's class to get the e value that is random
	// Testing Statement
	let mut e = prime::gen_large_prime(bit_size);

	while e > (*totient)
	{
		//Get another number from sam until we get one that is smaller than the totient
		
		//Testing Value
		e = prime::gen_large_prime(bit_size);
	}

	if e.clone().gcd(&totient.clone()) == BigUint::one()
	{
		return e;
	}
	
	return generate_e(totient, bit_size);
}

fn encrypt(encrypt: String, e: &num::bigint::BigUint, n: &num::bigint::BigUint) -> BigUint {

    let mut tester: Vec<u8> = Vec::new();


	for c in encrypt.chars() {
        let num: u8 = c as u8;
        println!("Character: {}", c as u8);
        tester.push(num);
	}

	let input_num = BigUint::from_bytes_le(tester.as_slice());

	let number = input_num.clone();

	if (*n) > input_num
	{

		println!("Number before mod {}", (number.clone()));
		return prime::schneier_mod_exp(number, (*e).clone(), (*n).clone());

	}

	return (*n).clone();
}

fn decrypt(decrypt: BigUint, d: &num::bigint::BigUint, n: &num::bigint::BigUint) -> String {
    println!("Into decrypt: {}", decrypt);
	let input_num = decrypt.clone();

	
	let number = prime::schneier_mod_exp(input_num.clone(), (*d).clone(), (*n).clone());
	//let numberb = prime::mod_exp(input_num.clone(), (*d).clone(), (*n).clone());
	//let numberc = prime::schneier_mod_exp(input_num.clone(), (*d).clone(), (*n).clone());
    //println!("modexp: {}\told: {}\tschneier: {}", number.clone(), numberb, numberc);
    println!("After mod in decrypt: {}", number);

    let bites: Vec<u8> = number.clone().to_bytes_le();
    for b in bites.iter(){
        println!("{}", (*b));
    }
    println!("Filling tester");

    //let tester = match String::from_utf8(bites).ok(){
    let tester = match String::from_utf8(bites){
        Err(why) => panic!("{}", Error::description(&why)),
        Ok(bits) => bits
        /*{
            match bits{
            Some(val) => {
                println!("match val: {}", val);
                return val
            },
                None => panic!("nothing to fill tester!")
            }
        }*/
    };

	return tester;
}

/** 
* 	used for finding d in the RSA algorithm
*/
fn inverse(a: &num::bigint::BigInt, n: &num::bigint::BigInt) -> num::bigint::BigUint {
	let mut t = BigInt::zero();
	let mut r = (*n).clone();
	let mut newt = BigInt::one();
	let mut newr = (*a).clone();

	while newr != BigInt::parse_bytes("0".as_bytes(), 10).unwrap()
	{
		let quotient = r.clone() / newr.clone();

		let temp_one = t.sub(&quotient.clone().mul(&newt));
		let temp_two = r.sub(&quotient.clone().mul(&newr));
		t = newt;
		newt = temp_one;
		r = newr;
		newr = temp_two;
		if t < BigInt::zero()
		{
			let temp_three = t + (*n).clone();
			t = temp_three;
        } 
	}
    match t.to_biguint(){
        Some(val) => {
            return val
        },
        None => {
            return BigUint::zero();
        }
    };
}


/**
 *  This function returns the euler's totient value based on given numbers p, q and n
 */
fn euler_totient(p: &num::bigint::BigUint, q: &num::bigint::BigUint, n: &num::bigint::BigUint) -> num::bigint::BigUint {
  let mut euler: num::bigint::BigUint;
 	euler = n - (p + q - BigUint::one());
 	return euler;
 }

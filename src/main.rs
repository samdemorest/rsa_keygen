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
	let a = reader.read_line(&mut stdin_buf).ok().expect("Failed to Read Line");

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
	//let mut e = BigUint::parse_bytes("17".as_bytes(),10).unwrap();
	let e = prime::gen_large_prime(bit_size);

	while e > (*totient)
	{
		//Get another number from sam until we get one that is smaller than the totient
		
		//Testing Value
		//e = BigUint::parse_bytes("17".as_bytes(),10).unwrap();
		let e = prime::gen_large_prime(bit_size);
	}

	//if(find_gcd(&e,totient) == BigUint::parse_bytes("1".as_bytes(),10).unwrap())
	if e.clone().gcd(&totient.clone()) == BigUint::one()
	{
		return e;
	}
	
	return generate_e(totient, bit_size);
}

fn encrypt(encrypt: String, e: &num::bigint::BigUint, n: &num::bigint::BigUint) -> BigUint {

	//let mut tester = String::new();
    let mut tester: Vec<u8> = Vec::new();


	for c in encrypt.chars() {
        let num: u8 = c as u8;
        println!("Character: {}", c as u8);
        tester.push(num);
	}

	let input_num = BigUint::from_bytes_le(tester.as_slice());

	let mut number = input_num.clone();

	if((*n) > input_num)
	{

		println!("Number before mod {}", (number.clone()));
		return (prime::mod_exp(number, (*e).clone(), (*n).clone()));

	}

	return ((*n).clone());
}

fn decrypt(decrypt: BigUint, d: &num::bigint::BigUint, n: &num::bigint::BigUint) -> String {
    println!("Into decrypt: {}", decrypt);
	let mut input_num = decrypt.clone();

	let mut number = input_num.clone();
	
	let mut i = BigUint::one();

	number = prime::old_mod_exp(input_num, (*d).clone(), (*n).clone());
    println!("After mod in decrypt: {}", number);
	let mut tester = String::new();

    let bites = number.clone().to_bytes_le();
    tester = String::from_utf8(bites).ok().unwrap();

	return tester;
}

fn get_ascii_val(character: char) -> u8 {
    return character as u8;
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
		let mut quotient = r.clone() / newr.clone();

		//let mut temp_one = t.clone() - (quotient.clone() * newt.clone());
		//let mut temp_two = r.clone() - (quotient.clone() * newr.clone());
		let mut temp_one = t.sub(&quotient.clone().mul(&newt));
		let mut temp_two = r.sub(&quotient.clone().mul(&newr));
		t = newt;
		newt = temp_one;
		r = newr;
		newr = temp_two;
		if t < BigInt::zero()
		{
			let mut temp_three = t + (*n).clone();
			t = temp_three;
        } 
	}
    let retval = match t.to_biguint(){
        Some(val) => val,
        None => {
            return BigUint::zero();
        }
    };
	return t.to_biguint().unwrap();
}


/**
 *  This function returns the euler's totient value based on given numbers p, q and n
 */
fn euler_totient(p: &num::bigint::BigUint, q: &num::bigint::BigUint, n: &num::bigint::BigUint) -> num::bigint::BigUint {
  let mut euler: num::bigint::BigUint;
 	euler = n - (p + q - BigUint::one());
 	return euler;
 }

extern crate num;
extern crate "rustc-serialize" as rustc_serialize;
extern crate rand;
use num::bigint::BigUint;
use num::bigint::BigInt;
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
    bignum = BigUint::parse_bytes("61".as_bytes(),10).unwrap(); //generate_large_prime_number(bit_size);
    let mut big_vec: Vec<num::bigint::BigUint> = Vec::new();
    println!("Big number incoming: {}", bignum);
    for _ in 0..100 {
        big_vec.push(rnjesus.gen_biguint(bit_size));
    }
    write_bnum(bignum);
    println!("Option unwrap: {}", BigUint::from_u64(420).unwrap());
	let mut p = BigUint::parse_bytes("61".as_bytes(),10).unwrap();
	let mut q = BigUint::parse_bytes("53".as_bytes(),10).unwrap();
	

	println!("Value of p = {}", p);
	println!("Value of q = {}", q);

	let mut n = (&p) * (&q);

	println!("Value of n = {}", n);
	
	let mut totient = euler_totient(&p,&q,&n);

	println!("Value of totient = {}", totient);

	let mut e = generate_e((&totient));

	println!("Value of e = {}", e);

	let mut d = find_modular_inverse((&e),(&totient));

	let mut tempOne = BigInt::parse_bytes("17".as_bytes(),10).unwrap();
	let mut tempTwo = BigInt::parse_bytes("3120".as_bytes(),10).unwrap();

	let mut d = inverse(&tempOne, &tempTwo);

	println!("Value of d = {}", d);
}

fn find_modular_inverse(e: &num::bigint::BigUint, totient: &num::bigint::BigUint) -> num::bigint::BigUint {
	return ((BigUint::parse_bytes("1".as_bytes(),10).unwrap() / e) % totient);
}

fn generate_e(totient: &num::bigint::BigUint) -> num::bigint::BigUint {
	
	//This would get a numbr from sam's class to get the e value that is random
	let mut e = BigUint::parse_bytes("17".as_bytes(),10).unwrap();
	
	while e > (*totient)
	{
		//Get another number from sam until we get one that is smaller than the totient
		e = BigUint::parse_bytes("17".as_bytes(),10).unwrap();
	}

	if(find_gcd(&e,totient) == BigUint::parse_bytes("1".as_bytes(),10).unwrap())
	{
		return e;
	}
	
	return generate_e(totient);
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

/**
 *
 */
fn find_gcd(n: &num::bigint::BigUint, m: &num::bigint::BigUint) -> num::bigint::BigUint {
	
	if *m == BigUint::parse_bytes("0".as_bytes(),10).unwrap(){
		return (*n).clone();
	} else {
		let a = (*m).clone();
		let b = (*n).clone() % (*m).clone();
		return find_gcd(&a, &b)
	}
}

// fn extended_euclids_function(n: &mut &num::bigint::BigUint, m: &mut &num::bigint::BigUint, gcd: &mut &num::bigint::BigUint, i: &mut &num::bigint::BigUint, j: &mut &num::bigint::BigUint)
// {
// 	if((*m) == &mut BigUint::parse_bytes("0".as_bytes(),10).unwrap())
// 	{
// 		let mut gcd = &mut (*n).clone();
// 		let mut i = &mut BigUint::parse_bytes("1".as_bytes(),10).unwrap();
// 		let mut j = &mut BigUint::parse_bytes("0".as_bytes(),10).unwrap();
	
// 		//debugging area
// 		println!("About to pop out of recursion");
// 		println!("Value of gcd = {}", gcd);
// 		println!("Value of i = {}", i);
// 		println!("Value of j = {}", j);

// 	} else {
// 		let mut iprime = BigUint::parse_bytes("0".as_bytes(),10).unwrap();
// 		let mut jprime = BigUint::parse_bytes("0".as_bytes(),10).unwrap();
// 		let mut gcdprime = BigUint::parse_bytes("0".as_bytes(),10).unwrap();
// 		let mut modval = ((*n).clone() % (*m).clone());
// 		extended_euclids_function(m, &mut &modval , &mut &gcdprime, &mut &iprime, &mut &jprime);
// 		let mut gcd = &mut gcdprime;
// 		let mut i = &mut jprime;
// 		let mut j = &mut (iprime - ((*i).clone() * ((*n).clone() / (*m).clone()))); 
// 	}
// }

fn inverse(a: &num::bigint::BigInt, n: &num::bigint::BigInt) -> num::bigint::BigInt {
	let mut t = BigInt::parse_bytes("0".as_bytes(),10).unwrap();
	let mut r = (*n).clone();
	let mut newt = BigInt::parse_bytes("1".as_bytes(),10).unwrap();
	let mut newr = (*a).clone();

	while newr != BigInt::parse_bytes("0".as_bytes(), 10).unwrap()
	{
		let mut quotient = r.clone() / newr.clone();

		let mut tempOne = t.clone() - (quotient.clone() * newt.clone());
		let mut tempTwo = r.clone() - (quotient.clone() * newr.clone());
		t = newt;
		newt = tempOne;
		r = newr;
		newr = tempTwo;

		//if r > BigInt::parse_bytes("1".as_bytes(),10).unwrap() 
		//{
		//	return BigInt::parse_bytes("1337".as_bytes(),10).unwrap();
		//}
		if t < BigInt::parse_bytes("0".as_bytes(),10).unwrap() 
		{
			let mut tempThree = t + (*n).clone();
			t = tempThree;
		}	
	}
	return t;
}


/**
 *  This function returns the euler's totient value based on given numbers p, q and n
 */
fn euler_totient(p: &num::bigint::BigUint, q: &num::bigint::BigUint, n: &num::bigint::BigUint) -> num::bigint::BigUint {
  let mut euler: num::bigint::BigUint;
 	euler = n - (p + q - BigUint::parse_bytes("1".as_bytes(),10).unwrap());
 	return euler;
 }

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
use num::bigint::ToBigInt;

mod prime;

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

    //Values used for testing
	//let mut p = BigUint::parse_bytes("61".as_bytes(),10).unwrap();
	//let mut q = BigUint::parse_bytes("53".as_bytes(),10).unwrap();


	let mut p = prime::gen_large_prime(bit_size);
	let mut q = prime::gen_large_prime(bit_size);
	

	println!("Value of p = {}", p);
	println!("Value of q = {}", q);

	let mut n = (&p) * (&q);

	println!("Value of n = {}", n);
	
	let mut totient = euler_totient(&p,&q,&n);

	println!("Value of totient = {}", totient);

	let mut e = generate_e((&totient), bit_size);

	println!("Value of e = {}", e);

	//Testing Values
	//let mut temp_one = BigInt::parse_bytes("17".as_bytes(),10).unwrap();
	//let mut temp_two = BigInt::parse_bytes("3120".as_bytes(),10).unwrap();
	//let mut d = inverse(&temp_one, &temp_two);

	let mut d = inverse((&(e.to_bigint().unwrap())), ((&totient.to_bigint().unwrap())));

	println!("Value of d = {}", d);

	let mut reader = io::stdin();

	println!("Please Insert Word To Encrypt");
	let a = reader.read_line().ok().expect("Failed to Read Line");


	println!("value of a = {}", a);

	let mut encrypted = encrypt(a, (&totient), (&e));
	let mut decrypted = decrypt(encrypted.clone(), (&d), (&e));
	println!("Encryption Time = {}", encrypted);
	println!("Decryption Time = {}", decrypted);

}

fn generate_e(totient: &num::bigint::BigUint, bit_size: usize) -> num::bigint::BigUint {
	
	//This would get a number from sam's class to get the e value that is random
	// Testing Statement
	//let mut e = BigUint::parse_bytes("17".as_bytes(),10).unwrap();
	let mut e = prime::gen_large_prime(bit_size);

	while e > (*totient)
	{
		//Get another number from sam until we get one that is smaller than the totient
		
		//Testing Value
		//e = BigUint::parse_bytes("17".as_bytes(),10).unwrap();
		let mut e = prime::gen_large_prime(bit_size);
	}

	if(find_gcd(&e,totient) == BigUint::parse_bytes("1".as_bytes(),10).unwrap())
	{
		return e;
	}
	
	return generate_e(totient, bit_size);
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

fn encrypt(encrypt: String, totient: &num::bigint::BigUint, e: &num::bigint::BigUint) -> String {


	return String::from_str("this should be the encrypted part");
}

fn decrypt(decrypt: String, d: &num::bigint::BigUint, e: &num::bigint::BigUint) -> String {


	return String::from_str("this should be the decrypted part");
}

/**
 *  Euclidian method for finding the greatest common denominator
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

/** 
* 	used for finding d in the RSA algorithm
*/
fn inverse(a: &num::bigint::BigInt, n: &num::bigint::BigInt) -> num::bigint::BigUint {
	let mut t = BigInt::parse_bytes("0".as_bytes(),10).unwrap();
	let mut r = (*n).clone();
	let mut newt = BigInt::parse_bytes("1".as_bytes(),10).unwrap();
	let mut newr = (*a).clone();

	while newr != BigInt::parse_bytes("0".as_bytes(), 10).unwrap()
	{
		let mut quotient = r.clone() / newr.clone();

		let mut temp_one = t.clone() - (quotient.clone() * newt.clone());
		let mut temp_two = r.clone() - (quotient.clone() * newr.clone());
		t = newt;
		newt = temp_one;
		r = newr;
		newr = temp_two;

		//if r > BigInt::parse_bytes("1".as_bytes(),10).unwrap() 
		//{
		//	return BigInt::parse_bytes("1337".as_bytes(),10).unwrap();
		//}
		if t < BigInt::parse_bytes("0".as_bytes(),10).unwrap() 
		{
			let mut temp_three = t + (*n).clone();
			t = temp_three;
		}	
	}
	return t.to_biguint().unwrap();
}


/**
 *  This function returns the euler's totient value based on given numbers p, q and n
 */
fn euler_totient(p: &num::bigint::BigUint, q: &num::bigint::BigUint, n: &num::bigint::BigUint) -> num::bigint::BigUint {
  let mut euler: num::bigint::BigUint;
 	euler = n - (p + q - BigUint::parse_bytes("1".as_bytes(),10).unwrap());
 	return euler;
 }

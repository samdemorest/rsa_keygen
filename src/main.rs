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

	println!("Please Insert Letter To Encrypt");
	let a = reader.read_line().ok().expect("Failed to Read Line");


	println!("You sent in {}", a);

	let mut encrypted = encrypt(a, (&totient), (&e), (&n));
	println!("Encryption Time = {}", encrypted);


	let mut decrypted = decrypt(encrypted.clone(), (&d), (&n));
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

fn encrypt(encrypt: String, totient: &num::bigint::BigUint, e: &num::bigint::BigUint, n: &num::bigint::BigUint) -> String {

	let mut tester = String::new();


	for c in encrypt.chars() {
		//println!("character {}", get_ascii_val(c));

		if(get_ascii_val(c) != -1)
		{
			tester = tester + (get_ascii_val(c).to_string().as_slice());
		}
	}

	//println!("Test {}", tester);

	let mut input_num = BigUint::parse_bytes(tester.as_bytes(),10).unwrap();

	//println!("Now its a number {}", input_num);

	let mut number = input_num.clone();

	//input_num = input_num.parse().unwrap();

	if((*n) > input_num)
	{
		//println!("It is big enough {}", input_num);

		
		let mut i = BigUint::parse_bytes("1".as_bytes(),10).unwrap();

		while  i < (*e) 
		{
			//println!("Counter i {} out of {}", i, (*e));
			//println!("Number before mod {}", number);
			number = (input_num.clone()) * number;
			i = i + BigUint::parse_bytes("1".as_bytes(),10).unwrap();
		}

		//println!("Number before mod {}", (number.clone()));
		//println!("Number After mod {}", (number.clone() % (*n).clone()));
	}


	return ((number.clone() % (*n).clone())).to_string();
}

fn decrypt(decrypt: String, d: &num::bigint::BigUint, n: &num::bigint::BigUint) -> String {
	//println!("Test {}", tester);

	let mut input_num = BigUint::parse_bytes(decrypt.as_bytes(),10).unwrap();

	//println!("Now its a number {}", input_num);

	let mut number = input_num.clone();

	//input_num = input_num.parse().unwrap();
	
	let mut i = BigUint::parse_bytes("1".as_bytes(),10).unwrap();

	while  i < (*d) 
	{
		//println!("Counter i {} out of {}", i, (*e));
		//println!("Number before mod {}", number);
		number = (input_num.clone()) * number;
		i = i + BigUint::parse_bytes("1".as_bytes(),10).unwrap();
	}

	//println!("Number before mod {}", (number.clone()));
	//println!("Number After mod {}", (number.clone() % (*totient).clone()));

	number = (number.clone() % (*n).clone());


	let mut tester = String::new();

	let mut temp = 0;
	let mut bite = String::new();

	for c in number.to_string().chars() {

		//chew
		if(temp == 0)
		{
			//println!("First Bite Char = {}", c);
			bite = c.to_string();
			temp = 1;

		} else {
			bite = bite + c.to_string().as_slice();
			temp = 0;

			//println!("Second Bite Char = {}", c);
			//println!("Bite String = {}", bite);

			tester = tester + val_to_ascii(BigUint::parse_bytes(bite.as_bytes(), 10).unwrap()).to_string().as_slice();

			//println!("Tester = {}", tester);
		}
	}

	return tester;
}

fn get_ascii_val(character: char) -> int {
	if (character == 'a' || character == 'A')
	{
		return 65;
	}
	if (character == 'b' || character == 'B')
	{
		return 66;	
	}
	if (character == 'c' || character == 'C')
	{
		return 67;
	}
	if (character == 'd' || character == 'D')
	{
		return 68;
	}
	if (character == 'e' || character == 'E')
	{
		return 69;
	}
	if (character == 'f' || character == 'F')
	{
		return 70;
	}
	if (character == 'g' || character == 'G')
	{
		return 71;
	}
	if (character == 'h' || character == 'H')
	{
		return 72;
	}
	if (character == 'i' || character == 'I')
	{
		return 73;
	}
	if (character == 'j' || character == 'J')
	{
		return 74;
	}
	if (character == 'k' || character == 'K')
	{
		return 75;
	}
	if (character == 'l' || character == 'L')
	{
		return 76;
	}
	if (character == 'm' || character == 'M')
	{
		return 77;
	}
	if (character == 'n' || character == 'N')
	{
		return 78;
	}
	if (character == 'o' || character == 'O')
	{
		return 79;
	}
	if (character == 'p' || character == 'P')
	{
		return 80;
	}
	if (character == 'q' || character == 'Q')
	{
		return 81;
	}
	if (character == 'r' || character == 'R')
	{
		return 82;
	}
	if (character == 's' || character == 'S')
	{
		return 83;
	}
	if (character == 't' || character == 'T')
	{
		return 84;
	}
	if (character == 'u' || character == 'U')
	{
		return 85;
	}
	if (character == 'v' || character == 'V')
	{
		return 86;
	}
	if (character == 'w' || character == 'W')
	{
		return 87;
	}
	if (character == 'x' || character == 'X')
	{
		return 88;
	}
	if (character == 'y' || character == 'Y')
	{
		return 89;
	}
	if (character == 'z' || character == 'Z')
	{
		return 90;
	}

	return -1;

}

fn val_to_ascii(integer: num::bigint::BigUint) -> char {
	
	if (integer == BigUint::parse_bytes("65".as_bytes(),10).unwrap())
	{
		return 'A';
	}
	if (integer == BigUint::parse_bytes("66".as_bytes(),10).unwrap())
	{
		return 'B';	
	}
	if (integer == BigUint::parse_bytes("67".as_bytes(),10).unwrap())
	{
		return 'C';
	}
	if (integer == BigUint::parse_bytes("68".as_bytes(),10).unwrap())
	{
		return 'D';
	}
	if (integer == BigUint::parse_bytes("69".as_bytes(),10).unwrap())
	{
		return 'E';
	}
	if (integer == BigUint::parse_bytes("70".as_bytes(),10).unwrap())
	{
		return 'F';
	}
	if (integer == BigUint::parse_bytes("71".as_bytes(),10).unwrap())
	{
		return 'G';
	}
	if (integer == BigUint::parse_bytes("72".as_bytes(),10).unwrap())
	{
		return 'H';
	}
	if (integer == BigUint::parse_bytes("73".as_bytes(),10).unwrap())
	{
		return 'I';
	}
	if (integer == BigUint::parse_bytes("74".as_bytes(),10).unwrap())
	{
		return 'J';
	}
	if (integer == BigUint::parse_bytes("75".as_bytes(),10).unwrap())
	{
		return 'K';
	}
	if (integer == BigUint::parse_bytes("76".as_bytes(),10).unwrap())
	{
		return 'L';
	}
	if (integer == BigUint::parse_bytes("77".as_bytes(),10).unwrap())
	{
		return 'M';
	}
	if (integer == BigUint::parse_bytes("78".as_bytes(),10).unwrap())
	{
		return 'N';
	}
	if (integer == BigUint::parse_bytes("79".as_bytes(),10).unwrap())
	{
		return 'O';
	}
	if (integer == BigUint::parse_bytes("80".as_bytes(),10).unwrap())
	{
		return 'P';
	}
	if (integer == BigUint::parse_bytes("81".as_bytes(),10).unwrap())
	{
		return 'Q';
	}
	if (integer == BigUint::parse_bytes("82".as_bytes(),10).unwrap())
	{
		return 'R';
	}
	if (integer == BigUint::parse_bytes("83".as_bytes(),10).unwrap())
	{
		return 'S';
	}
	if (integer == BigUint::parse_bytes("84".as_bytes(),10).unwrap())
	{
		return 'T';
	}
	if (integer == BigUint::parse_bytes("85".as_bytes(),10).unwrap())
	{
		return 'U';
	}
	if (integer == BigUint::parse_bytes("86".as_bytes(),10).unwrap())
	{
		return 'V';
	}
	if (integer == BigUint::parse_bytes("87".as_bytes(),10).unwrap())
	{
		return 'W';
	}
	if (integer == BigUint::parse_bytes("88".as_bytes(),10).unwrap())
	{
		return 'X';
	}
	if (integer == BigUint::parse_bytes("89".as_bytes(),10).unwrap())
	{
		return 'Y';
	}
	if (integer == BigUint::parse_bytes("90".as_bytes(),10).unwrap())
	{
		return 'Z';
	}

	return ' ';

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

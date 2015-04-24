#![feature(convert)]
#![feature(core)]
#![feature(collections)]
extern crate num;
extern crate rustc_serialize;
extern crate core;
extern crate rand;
extern crate time;
extern crate prime;
use num::bigint::{BigUint, BigInt, ToBigUint, ToBigInt};
use std::io;
use num::traits::{FromPrimitive, One, Zero};
use num::integer::Integer;
use core::ops::*;
use std::error::Error;
use std::env;


fn main() {
    println!("{}", time::precise_time_ns());
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

	println!("You sent in {}", send_str);
    
    let e_start_time = time::precise_time_ns();
	let encrypted = encrypt(send_str, (&e), (&n));
    let e_finish_time = time::precise_time_ns();
    let enc_tot_time = e_finish_time - e_start_time;
    println!("Encryption took {} ns", enc_tot_time);
	//println!("Encryption Time = {}", encrypted);

    let d_start_time = time::precise_time_ns();
	let decrypted = decrypt(encrypted.clone(), (&d), (&n));
    let d_finish_time = time::precise_time_ns();
    let dec_tot_time = d_finish_time - d_start_time;
    println!("Decryption took {} ns", dec_tot_time);
	println!("Decryption Time = {}", decrypted);

}

fn generate_e(totient: &BigUint, bit_size: usize) -> BigUint {
	
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

fn encrypt(encrypt: String, e: &BigUint, n: &BigUint) -> BigUint {

    let mut tester: Vec<u8> = Vec::new();


	for c in encrypt.chars() {
        let num: u8 = c as u8; // was u8
        tester.push(num);
	}

	let input_num = BigUint::from_bytes_le(tester.as_slice());

	let number = input_num.clone();

	if (*n) > input_num
	{

		return prime::schneier_mod_exp(number, (*e).clone(), (*n).clone());

	}

	return (*n).clone();
}

fn decrypt(decrypt: BigUint, d: &BigUint, n: &BigUint) -> String {
	let input_num = decrypt.clone();

	
	let number = prime::schneier_mod_exp(input_num.clone(), (*d).clone(), (*n).clone());

    let bites: Vec<u8> = number.clone().to_bytes_le();


    let tester = match String::from_utf8(bites){
        Err(why) => panic!("{}", Error::description(&why)),
        Ok(bits) => bits
    };

	return tester;
}

/** 
* 	used for finding d in the RSA algorithm
*/
fn inverse(a: &BigInt, n: &BigInt) -> BigUint {
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
fn euler_totient(p: &BigUint, q: &BigUint, n: &BigUint) -> BigUint {
  let mut euler: BigUint;
 	euler = n - (p + q - BigUint::one());
 	return euler;
 }

#[cfg(test)]
mod tests{
    extern crate time;
    extern crate num;
    extern crate prime;
    extern crate rand;
    extern crate core;
    use num::bigint::*;
    use prime::*;
    use super::{encrypt, decrypt, inverse, generate_e, euler_totient};
    use std::fs::{OpenOptions, File};
    use std::path::Path;
    use num::traits::{One, Zero};
    use std::io::prelude::*;
    use core::str::Str;

    #[ignore]
    pub fn crypt_times(){
        let bit_vec: Vec<usize> = vec![16, 32, 64, 96, 128];
        for size in bit_vec.iter(){
            let dec_file = match *size{
                16 => "decryption_times16",
                32 => "decryption_times32",
                64 => "decryption_times64",
                96 => "decryption_times96",
                128 => "decryption_times128",
                _ => panic!("out of range")
            };
            let enc_file = match *size{
                16 => "encryption_times16",
                32 => "encryption_times32",
                64 => "encryption_times64",
                96 => "encryption_times96",
                128 => "encryption_times128",
                _ => panic!("out of range")
            };

            let mut e_path = Path::new(enc_file);
            let mut d_path = Path::new(dec_file);
            let mut e_results = 
                match OpenOptions::new().create(true).append(true).write(true).open(e_path){
                    Err(why) => panic!(why),
                    Ok(yeah) => yeah
                };
            let mut d_results = 
                match OpenOptions::new().create(true).append(true).write(true).open(d_path){
                    Err(why) => panic!(why),
                    Ok(yeah) => yeah
                };

            /*
             * Here's where we run the tests 100 times to get data to average.
             */
            for _ in 1..100{
                let mut p: BigUint = BigUint::zero();
                let mut q: BigUint = BigUint::zero();
                let mut d: BigUint = BigUint::zero();
                let mut n: BigUint= BigUint::zero();
                let mut totient: BigUint = BigUint::zero();
                let mut e: BigUint = BigUint::zero();
                while d.is_zero(){
                    p = prime::gen_large_prime(*size);
                    q = prime::gen_large_prime(*size);
                    n = (&p) * (&q);
                    totient = euler_totient(&p,&q,&n);
                    e = generate_e((&totient), *size);
                    d = inverse((&(e.to_bigint().unwrap())), ((&totient.to_bigint().unwrap())));
                }
                let e_start_time = time::precise_time_ns();
                let encrypted = encrypt("hi".to_string(), (&e), (&n));
                let e_finish_time = time::precise_time_ns();
                let enc_tot_time = e_finish_time - e_start_time;
                let e_tot_form = format!("{}\n", enc_tot_time);
                e_results.write_all(e_tot_form.as_bytes());

                let d_start_time = time::precise_time_ns();
                let decrypted = decrypt(encrypted.clone(), (&d), (&n));
                let d_finish_time = time::precise_time_ns();
                let d_tot_time = d_finish_time - d_start_time;
                let d_tot_form = format!("{}\n", d_tot_time);
                d_results.write_all(d_tot_form.as_bytes());
                }
        }
    assert_eq!(true, true);
    }

    #[test]
    pub fn modexp_test(){
        let bit_vec: Vec<usize> = vec![16,18,20,22];
        let mut rng = rand::thread_rng();
        for size in bit_vec.iter(){
            let sch_file = match *size{
                16 => "schneier16",
                18 => "schneier18",
                20 => "schneier20",
                22 => "schneier22",
                _ => panic!("out of range")
            };
            let nai_file = match *size{
                16 => "naive16",
                18 => "naive18",
                20 => "naive20",
                22 => "naive22",
                _ => panic!("out of range")
            };

            let mut s_path = Path::new(sch_file);
            let mut n_path = Path::new(nai_file);
            let mut s_results = 
                match OpenOptions::new().create(true).append(true).write(true).open(s_path){
                    Err(why) => panic!(why),
                    Ok(yeah) => yeah
                };
            let mut n_results = 
                match OpenOptions::new().create(true).append(true).write(true).open(n_path){
                    Err(why) => panic!(why),
                    Ok(yeah) => yeah
                };

            for n in 1..100{
                let a: BigUint = rng.gen_biguint(*size);
                let b: BigUint = rng.gen_biguint(*size);
                let c: BigUint = rng.gen_biguint(*size);

                let schneier_start = time::precise_time_ns();
                let sme = prime::schneier_mod_exp(a.clone(), b.clone(), c.clone());
                let schneier_finish = time::precise_time_ns();
                let schneier_total = schneier_finish - schneier_start;
                let schneier_ttime = format!("{}\n", schneier_total);
                s_results.write_all(schneier_ttime.as_bytes());


                let naive_start = time::precise_time_ns();
                let nme = prime::naive_mod_exp(a.clone(), b.clone(), c.clone());
                let naive_finish = time::precise_time_ns();
                let naive_total = naive_finish - naive_start;
                let naive_ttime = format!("{}\n", naive_total);
                n_results.write_all(naive_ttime.as_bytes());
            }
        }

        assert_eq!(true, true);
    }
}

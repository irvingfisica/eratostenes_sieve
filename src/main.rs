fn main() {

	let limite = 1000;
	let sqlim = (limite as f64).sqrt().ceil() as u64;
    
	let mut a = vec![true;limite];
	let mut primes: Vec<i32> = Vec::new(); 

	for i in 2..sqlim {
		let index = (i-2) as usize;
		if a[index] {
			let mut j = i.pow(2) as usize;
			let mut c = 0;
			while j < limite {
				a[j-2] = false;
				c = c + 1;
				j = (i.pow(2) + c*i) as usize;
			}
		}
	}

	for i in 2..limite {
		let index = (i-2) as usize;
		if a[index] {
			primes.push(i as i32);
		}
	}

	println!("{:?}",primes);

}

fn main() {
	let t:f64 = 450_000.0;
	let m:f64 = 1_500_000.0;
	let h:f64 = 750_000.0;
	let d:f64 = 2_850_000.0;
	let a:f64 = 250_000.0;
	let q:f64 = 2.0;
	let u:f64 = 1.0;
	let y:f64 = 3.0;

	//Sum and Average
	let sum = (t * q) + (m * u) + (h * y) + (d * y) + (a * u);
	println!("Sum is {}", sum);
	let average = sum / 5.0;
	println!("Average is {}", average);

}

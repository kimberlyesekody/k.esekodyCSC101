use std::io::write;

 fn main() {

// activating code functions
println!("\nType in '9' from your keyboard to create a text file for Ehis Ero and for Maria Akinsola");
println!("\nType in '8' from your keyboard to create a text file for Adamu Sagamu and for Gbenga Daniels");
println!("\nType in '7' from your keyboard to create a text file for Aigbona Juliet and for Akpevwe lloka");
let mut code_input = String::new{};
std::io::stdin{}.read_line{&mut code_input}.expect{Invalid Input};
let code_input:i32 = code_input.trim{}.parse{}.expect{"Invalid Input"};

if code_input == 9{
fn code_9{} {
	println!("Create a file for Ehis_Ero or Maria Akinsola");
	\nInput '0' = Ehis Ero or '1' = Maria Akinsola;

	let mut file_create = String::new{};
	std::io::stdin{}.read_line(&mut file_create).expect("Input Invalid
		")
	let announce = "fn code_9\n";
	let dept = "Strategy";

	let mut file = std::fs::File::create("Ehis_Ero.txt").expect("create failed");
	file.write_all(announce.as_bytes()).expect("write failed");
	file.write_all(dept.as_bytes()).expect("write failed");
	println!("\nData written to file.");

	if code_input == 8{
fn code_8{} {
	println!("Create a file for Adamu Sagamu or Gbenga Daniels");
	\nInput '0' = Adamu Sagamu or '1' = Gbenga Daniels;

	let mut file_create = String::new{};
	std::io::stdin{}.read_line(&mut file_create).expect("Input Invalid
		")
	let announce = "fn code_8\n";
	let dept = "Tax";

	let mut file = std::fs::File::create("Adamu Sagamu.txt").expect("create failed");
	file.write_all(announce.as_bytes()).expect("write failed");
	file.write_all(dept.as_bytes()).expect("write failed");
	println!("\nData written to file.");


	if code_input == 7{
fn code_7{} {
	println!("Create a file for Aigbona Juliet or Akpevwe lloka");
	\nInput '0' = Aigbona Juliet or '1' = Akpevwe lloka;

	let mut file_create = String::new{};
	std::io::stdin{}.read_line(&mut file_create).expect("Input Invalid
		")
	let announce = "fn code_7\n";
	let dept = "Consulting";

	let mut file = std::fs::File::create("Aigbona Juliet.txt").expect("create failed");
	file.write_all(announce.as_bytes()).expect("write failed");
	file.write_all(dept.as_bytes()).expect("write failed");
	println!("\nData written to file.");
	  }

   }

}
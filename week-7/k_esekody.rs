use std::io;

fn main() {

	println!("\n g = GeoPo_Merger function
		p = Pub_Service function");

	let mut input1 = String::new();
	println!("\nChoose your division ");
	io::stdin().read_line(&mut input1).expect("Invalid");

    if input1 == "p"{
    	pub_service_function();
    }
    else if input1 == "g"{
    	geopo_merger_function();
    }
    else {
    	println!("Please try again");
    }
}

    fn GeoPo_Merger_Function(){
    	println!("\n Ai = Aigbogun Alamba Daudu  -Internal Affairs  -South West
    		M = Murtala Afeez Bendu             -Justice           -North East
    		Ok = Okorocha Calistus Ogbona        -Defence           -South South
    		Ad = Adewale Jimoh Akanbi            -Power & Steel     -South West
    		Os = Osazuwa Faith Etieye            -Petroleum         -South East");

    	let mut input1 = String::new();
    	let mut input2 = String::new();
    	let mut input3 = String::new();

    	println!("Enter name ");
    	io::stdin().read_line(&mut input1).expect(Invalid);
    	let name = input1.trim();

    	println!("Enter Ministry ");
    	io::stdin().read_line(&mut input2).expect(Invalid);
    	let ministry = input2.trim();

    	println!("Enter Geopolitical Zone ");
    	io::stdin().read_line(&mut input3).expect(Invalid);
    	let geopolitical_zone = input3.trim();

    	if name == "Ai"{
    		println!("This commisioner details include: ",name,ministry,geopolitical_zone);
    	}
    	else if name == "M"{
    		println!("This commisioner details include: ",name,ministry,geopolitical_zone);
    	}
    	else if name == "Ok"{
    		println!("This commisioner details include: ",name,ministry,geopolitical_zone);
    	}
        else if name == "Ad"{
    		println!("This commisioner details include: ",name,ministry,geopolitical_zone);
    	}
    	else if name == "Os"{
    		println!("This commisioner details include: ",name,ministry,geopolitical_zone);
    	}
    }

    fn Pub_Service_function(){
    	println!("\n Oa = Office administrator
    		A = Academic
    		L = Lawyer
    		T = Teacher");
    
        let mut input1 = String::new();
    	let mut input2 = String::new();
    	let mut input3 = String::new();
    	
    	println!("Enter name");
        io::stdin().read_line(&mut input1).expect("Not a valid string");

    	println!("Enter Job");
    	io::stdin().read_line(&mut input2).expect("Invalid input");
    	let Job = input1.trim();

    	println!("How many years of experience do you have? ");
    	io::stdin().read_line(&mut input3).expect("Invalid input");
	    let years:i32 = input2.trim().parse().expect("Invalid input");

	    if years <= 2 && Job == "Oa"{
	    println!("You are an Intern {} ", input1);
        }
        else if years <= 2 && Job == "A"{
        	println!("You are not hired {} ", input1);
        }
        else if years <= 2 && Job == "L"{
        	println!("You are a Paralegal {} ", input1);
        }
        else if <= 2 && Job == "T"{
        	println!("You are a Placement {} ", input1);
        }
        else if years >= 3 && years <= 5 && Job == "Oa"{
        	println!("You are an Administrator {} ", input1);
        }
        else if years >= 3 && years <= 5 && Job == "A"{
        	println!("You are a Research Assistant {} ", input1);
        }
        else if years >= 3 && years <= 5 && Job == "L"{
        	println!("You are a Junior Associate {} ", input1);
        }
        else if years >= 3 && years <= 5 && Job == "T"{
        	println!("You are a Classroom Taecher {} ", input1);
        }
        else if years >= 5 && years <= 8 && Job == "Oa"{
        	println!("You are Senior Administrator {} ", input1);
        }
        else if years >= 5 && years <= 8 && Job == "A"{
        	println!("You are PhD Candidate {} ", input1);
        }
        else if years >= 5 && years <= 8 && Job == "L"{
        	println!("You are Associate {} ", input1);
        }
        else if years >= 5 && years <= 8 && Job == "T"{
        	println!("You are Snr Teacher {} ", input1);
        }
        else if years >= 8 && years <= 10 && Job == "Oa"{
        	println!("You are an Office Manager {} ", input1);
        }
        else if years >= 8 && years <= 10 && Job == "A"{
        	println!("You are a Post-Doc Researcher {} ", input1);
        }
        else if years >= 8 && years <= 10 && Job == "L"{
        	println!("You are a Senior Associate 1-2 {} ", input1);
        }
        else if years >= 8 && years <= 10 && Job == "T"{
        	println!("You are a Leading Teacher {} ", input1);
        }
        else if years >= 10 && years <= 13 && Job == "Oa"{
        	println!("You are a Director {} ", input1);
        }
        else if years >= 10 && years <= 13 && Job == "A"{
        	println!("You are an Senior Lecturer {} ", input1);
        }
        else if years >= 10 && years <= 13 && Job == "L"{
        	println!("You are a Senior Assistant 3-4 {} ", input1);
        }
        else if years >= 10 && years <= 13 && Job == "T"{
        	println!("You are a Deputy Principle {} ", input1);
        }
        else if years = 14 && Job == "Oa"{
        	println!("You are a CEO {} ", input1);
        }
        else if years = 14 && Job == "A"{
        	println!("You are a Dean {} ", input1);
        }
        else if years = 14 && Job == "L"{
        	println!("You are a Partner {} ", input1);
        }
        else if years = 14 && Job == "Oa"{
        	println!("You are a Principle {} ", input1);
        }
        else {
        	println!("Error");
        }

    }

 
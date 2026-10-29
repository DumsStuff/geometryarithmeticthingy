use ez_input::inputln;

fn main() {
	loop {

	println!(">---------------- ||  [Welcome to the CLINIB Tranformation Calculator v1]  || ----------------<");
	println!("                         (command line interface & number input based) ");
	println!("1 for translation,");
	println!("2 for rotation,");
	println!("3 for mirroring.");

	let input_string = inputln!("");

		let r: i32 = input_string.trim().parse().unwrap_or(0);
		// trim removes space parse makes it an int.
		match r {
			1 => {
				println!("Translation selected.");
				let mut x = inputln!("Enter X coordinate: ").trim().parse().unwrap_or(0);
				let mut y = inputln!("Enter Y coordinate: ").trim().parse().unwrap_or(0);
				let xvec = inputln!("Enter X Vector: ").trim().parse().unwrap_or(0);
				let yvec = inputln!("Enter Y Vector: ").trim().parse().unwrap_or(0);

				x = x + xvec;
				y = y + yvec;
				println!("the new coordinates are: {}, {}", x, y)
			}
			2 => {
				println!("Rotation selected.");
				println!("Please select: Clockwise or Counterclockwise (1 or 2)");
				let dir: i32 = inputln!("").trim().parse().unwrap_or(0);
				if dir == 1 {
					println!("clockwise rotation Selected");
					println!("please select one of the above rotations. 90, 180, 270. (1, 2, 3)");
					let o: i32 = inputln!("").trim().parse().unwrap_or(0);
					match o {
						1 => {
							println!("90 Degree rotation Selected");
							let mut x = inputln!("Enter X coordinate: ").trim().parse().unwrap_or(0);
							let mut y = inputln!("Enter Y coordinate: ").trim().parse().unwrap_or(0);
							let xsave = x;  //save both vars
							x = y;
							y = -xsave;
							println!("your new coordinates are: {}, {}", x, y);
						}
						2 => {
							println!("180 Degree rotation Selected");
							let mut x = inputln!("Enter X coordinate: ").trim().parse().unwrap_or(0);
							let mut y = inputln!("Enter Y coordinate: ").trim().parse().unwrap_or(0);
							y = -y;
							x = -x;
							println!("your new coordinates are: {}, {}", x, y);
						}
						3 => {
							println!("270 Degree rotation Selected");
							let mut x = inputln!("Enter X coordinate: ").trim().parse().unwrap_or(0);
							let mut y = inputln!("Enter Y coordinate: ").trim().parse().unwrap_or(0);
							let xsave = x;  //save both vars
							x = -y;
							y = xsave;
							println!("your new coordinates are: {}, {}", x, y);
						}
						_ => {
							println!("Please enter a positive integer (1-3)");
						}
					}
				} else {
					println!("Counter-clockwise rotation Selected");
					println!("please select one of the above rotations. 90, 180, 270. (1, 2, 3)");
					let o: i32 = inputln!("").trim().parse().unwrap_or(0);
					match o {
						1 => {
							println!("90 Degree rotation Selected");
							let mut x = inputln!("Enter X coordinate: ").trim().parse().unwrap_or(0);
							let mut y = inputln!("Enter Y coordinate: ").trim().parse().unwrap_or(0);
							let xsave = x;  //save both vars
							x = -y;
							y = xsave;
							println!("your new coordinates are: {}, {}", x, y);
						}
						2 => {
							println!("180 Degree rotation Selected");
							let mut x = inputln!("Enter X coordinate: ").trim().parse().unwrap_or(0);
							let mut y = inputln!("Enter Y coordinate: ").trim().parse().unwrap_or(0);
							y = -y;
							x = -x;
							println!("your new coordinates are: {}, {}", x, y);
						}
						3 => {
							println!("270 Degree rotation Selected");
							let mut x = inputln!("Enter X coordinate: ").trim().parse().unwrap_or(0);
							let mut y = inputln!("Enter Y coordinate: ").trim().parse().unwrap_or(0);
							let xsave = x;  //save both vars
							x = y;
							y = -xsave;
							println!("your new coordinates are: {}, {}", x, y);
						}
						_ => {
							println!("Please enter a positive integer (1-3)");
						}
					}
				}
			}
			3 => {
				println!("Mirroring selected.");
				println!("Please select:");
				println!("Reflect over X axis (1)");
				println!("Reflect over Y axis (2)");
				println!("Reflect over X = ?? (3)");
				println!("Reflect over Y = ?? (4)");
				println!("Reflect over Y = X  (5)");
				println!("Reflect over Y = -X (6)");
				let o: i32 = inputln!("").trim().parse().unwrap_or(0);
				match o {
					1 => {
						println!("You chose: Reflect over X axis");
						let x = inputln!("Enter X coordinate: ").trim().parse().unwrap_or(0);
						let mut y = inputln!("Enter Y coordinate: ").trim().parse().unwrap_or(0);
						y = -y;
						println!("the flipped coordinates are: {}, {}", x, y);
					}
					2 => {
						println!("You chose: Reflect over Y axis");
						let mut x = inputln!("Enter X coordinate: ").trim().parse().unwrap_or(0);
						let y = inputln!("Enter Y coordinate: ").trim().parse().unwrap_or(0);
						x = -x;
						println!("the flipped coordinates are: {}, {}", x, y);
					}
					3 => {
						println!("You chose: Reflect over X = ??");
						let mut x = inputln!("Enter X coordinate: ").trim().parse().unwrap_or(0);
						let y = inputln!("Enter Y coordinate: ").trim().parse().unwrap_or(0);
						let xflip = inputln!("Enter X flip: ").trim().parse().unwrap_or(0);
						x = (2 * xflip) - x;
						println!("the flipped coordinates are: {}, {}", x, y);
					}
					4 => {
						println!("You chose: Reflect over Y = ??");
						let x = inputln!("Enter X coordinate: ").trim().parse().unwrap_or(0);
						let mut y = inputln!("Enter Y coordinate: ").trim().parse().unwrap_or(0);
						let yflip = inputln!("Enter Y flip: ").trim().parse().unwrap_or(0);
						y = (2 * yflip) - y;
						println!("the flipped coordinates are: {}, {}", x, y);
					}
					5 => {
						println!("You chose: Reflect over Y = X");
						let mut x = inputln!("Enter X coordinate: ").trim().parse().unwrap_or(0);
						let mut y = inputln!("Enter Y coordinate: ").trim().parse().unwrap_or(0);
						let xsave = x;
						x = y;
						y = xsave;
						println!("the flipped coordinates are: {}, {}", x, y);
					}
					6 => {
						println!("You chose: Reflect over Y = -X");
						let mut x = inputln!("Enter X coordinate: ").trim().parse().unwrap_or(0);
						let mut y = inputln!("Enter Y coordinate: ").trim().parse().unwrap_or(0);
						let xsave = x;
						x = -y;
						y = -xsave;
						println!("the flipped coordinates are: {}, {}", x, y);
					}
					_ => {
						println!("Please enter a positive integer (1-6)");
					}
				}
			}
			_ => {
				println!("Please enter a positive integer (1-3)");
			}
		}

		println!("type 1 to go to menu, type literally anything else to close it.");
		let menu: i32 = inputln!("").trim().parse().unwrap_or(0);
		if menu != 1 {
			break
		}
	}
}

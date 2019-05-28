fn main() {
	println!("Hello, world!");
	let mut x = 5;
	println!("The value of x is: {}",x);
	x = 6;
	println!("The value of x is: {}", x);
	const MAX_POINTS: u32 = 100_000;
	println!("The value of MAX_POINTS is: {}", MAX_POINTS);
	let spaces = "   ";
	let spaces = spaces.len();
	println!("The value of spaces is: {}", spaces);

	let tup = (500, 6.4, 1);

	let (_a, b, _c) = tup;

	println!("The value of b is: {}, which is the same as {}", b,tup.1);
    
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let second = a[1];
    println!("The second element of the array is: {}", second);

    /*
    another_function(5,6);
    */
    another_function(five(),7);

    another_function(five(),plus_one(7));
}

fn another_function(x: i32, y:i32) {
    println!("Another function.");
	println!("The value of x is: {}", x);
	println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

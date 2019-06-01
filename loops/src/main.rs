use f64;

fn main() {
    println!("Hello, world!");
    breakreturns();
    iter_func();
    liftoff(10);
    println!("50.0F is {}C", far_to_cel(50.0));
    println!("fibonacci(5) = {}", fibonacci(5));
    println!("sqrt(10000.0) = {}", sqrt(10000.0));
}

fn sqrt(x:f64) -> f64 {
    let ans : f64 = if x <= 0.0 {
        0.0
        }
        else {
            let mut _guess : f64 = x/2.0;
            for _k in 1..100 {
                _guess = _guess - (_guess * _guess - x) / (2.0 * _guess);
            }
            _guess
        };
    ans
}

fn breakreturns(){
    let mut counter : u8 = 0;
    let result : u8 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn iter_func(){
    let a:[u8;5] = [10,20,30,40,50];

    for element in a.iter() {
        println!("the value of is: {}", element);
    }
}

fn liftoff(start:u8) {
    if start > 1 {
        for number in (1..start).rev() {
            println!("{}!", number)
        }
    }
    println!("LIFTOFF!!!");
}

fn far_to_cel(far:f32) -> f32 {
    (far - 32.0) * (5.0/9.0)
}


fn fibonacci(n:u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2)
    }
}




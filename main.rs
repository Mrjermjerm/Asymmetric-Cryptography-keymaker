use rand::Rng;

fn generate_random_number(max: u32) -> u32 {
    let mut rng = rand::rng();
    rng.random_range(1..=max)
}

const MODULES: f64 = 17.0;

fn point_doubling(x1: f64, y1: f64) -> (f64, f64) {
    // Compute the slope s
    let mut s = (3.0 * x1 * x1 + 2.0).rem_euclid(MODULES);
    let t = mod_inverse(y1*2.0, MODULES);
    if let Some(t_value) = t{
        s = s * t_value;
        println!("t: {}, ", t_value);
    }else {
        println!("No modular inverse");
    }
    
    s = s.rem_euclid(MODULES);
    println!("s: {}, ", s);
    
    // Compute the new x-coordinate x3
    let mut x3 = s * s - x1 - x1;
    x3 = x3.rem_euclid(MODULES);
    
    // Compute the new y-coordinate y3
    let mut y3 = s * (x1 - x3) - y1;
    y3 = y3.rem_euclid(MODULES);
    
    (x3, y3)
}

fn point_addition(x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64) {
    // Compute the slope
    let mut s = (y1 - y2)/ (x1 - x2);

    // let t = mod_inverse(x1 - x2, MODULES);
    // if let Some(t_value) = t{
    //     s = s * t_value;
    //     println!("t: {}, ", t_value);
    // }else {
    //     println!("No modular inverse");
    // }

    // println!("x1,x2,y1,y2: {}, {}, {}, {} ", x1,x2,y1,y2);
    // Compute the new x-coordinate x_r
    let x_r = negative_modular_inverse(s * s - (x1 + x2), MODULES);
    // Compute the new y-coordinate y_r
    let y_r = negative_modular_inverse(s * (x1 - x_r) - y1, MODULES);
    println!("x_r, y_r: {}, {} ", x_r, y_r);

    // s * (x1 - x_r) - y1;
    (x_r, y_r)
}



fn mod_inverse(a: f64, m: f64) -> Option<f64> {
    let (gcd, x, _) = extended_gcd(a, m);
    if gcd != 1.0 {
        None
    } else {
        Some((x % m + m) % m)
    }
}

fn extended_gcd(a: f64, b: f64) -> (f64, f64, f64) {
    if a == 0.0 {
        return (b, 0.0, 1.0);
    }
    let (gcd, x1, y1) = extended_gcd(b % a, a);
    (gcd, y1 - (b / a).floor() * x1, x1)
}

fn negative_modular_inverse(a: f64, m: f64) -> f64 {
    if a == 1.0
    {
        return 1.0;
    }
    let x = a+m;
    return x;

}

fn cyclic_group(mut x1: f64, mut y1: f64) -> (f64, f64) {
    // Call point doubling
    let o = 19;
    let (mut x_new, mut y_new) = point_doubling(x1, y1);
    print!("({}, {}), ", x_new, y_new);
    
    println!("X1 and Y1({}, {}), ", x1, y1);
    for _ in 1..=o { // goes from 1 to 19
        let (x_newest, y_newest) = point_addition(x1, y1, x_new, y_new);
        println!("Newest ({}, {}), ", x_newest, y_newest);
        x1 = x_new;
        y1 = y_new;
        x_new = x_newest;
        y_new = y_newest;
    }
    (x1, y1)
}
fn main() {
    let x1 = 5.0;
    let y1 = 1.0;
    let result = cyclic_group(x1, y1);
    println!("Result: {:?}", result);

    // get the random number
    let random_number = generate_random_number(10);
    println!("Random number: {}", random_number);
}

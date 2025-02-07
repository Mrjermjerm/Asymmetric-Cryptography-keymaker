use rand::Rng;

const MODULUS: f64 = 9173.0;

fn generate_random_number(max: u32) -> u32 {
    let mut rng = rand::rng();
    rng.random_range(1..=max)
}


fn point_doubling(x1: f64, y1: f64) -> (f64, f64) {
    // Compute the slope s
    let mut s = (3.0 * x1 * x1 + 2.0).rem_euclid(MODULUS);
    let t = mod_inverse(y1*2.0, MODULUS);
    s = s * t;
    
    s = s.rem_euclid(MODULUS);
    // println!("s: {}, ", s);
    
    // Compute the new x-coordinate x3
    let mut x3 = s * s - x1 - x1;
    x3 = x3.rem_euclid(MODULUS);
    
    // Compute the new y-coordinate y3
    let mut y3 = s * (x1 - x3) - y1;
    y3 = y3.rem_euclid(MODULUS);
    
    (x3, y3)
}

fn point_addition(x1: f64, y1: f64, x2: f64, y2: f64) -> (f64, f64) {
    // Compute the slope
    let mut s = (y2 - y1).rem_euclid(MODULUS);
    let t = mod_inverse((x2 - x1).rem_euclid(MODULUS), MODULUS);
    s = (s * t).rem_euclid(MODULUS);
            
    // Compute the new x-coordinate x_r
    let mut x_r = (s * s - x1 - x2).rem_euclid(MODULUS);
    if x_r < 0.0 {
        x_r += MODULUS;
    }

    // Compute the new y-coordinate y_r
    let mut y_r = (s * (x1 - x_r) - y1).rem_euclid(MODULUS);
    if y_r < 0.0 {
        y_r += MODULUS;
    }

    (x_r, y_r)
}



fn mod_inverse(a: f64, m: f64) -> f64 {
    let a = a.rem_euclid(m);
    let (gcd, x, _) = extended_gcd(a, m);
    if gcd != 1.0 {
        return 1.0;
    } else {
        return (x % m + m) % m;
    }
}

fn extended_gcd(a: f64, b: f64) -> (f64, f64, f64) {
    if a == 0.0 {
        return (b, 0.0, 1.0);
    }
    let (gcd, x1, y1) = extended_gcd(b % a, a);
    (gcd, y1 - (b / a).floor() * x1, x1)
}

fn cyclic_group(x1: f64, y1: f64) -> Vec<(f64, f64)> {
  
    let mut cyclic_group_nums = Vec::new();
    cyclic_group_nums.push((x1, y1));
    let (mut x_new, mut y_new) = point_doubling(x1, y1);
    

    cyclic_group_nums.push((x_new, y_new));


    while x_new != x1 && y_new != MODULUS -y1 {
        (x_new, y_new) = point_addition(x1, y1, x_new, y_new);
        cyclic_group_nums.push((x_new, y_new));
        println!("x_new: {}, y_new: {}", x_new, y_new);
    }

    cyclic_group_nums
}


fn main() {
    let x1 = 5.0;
    let y1 = 1.0;
    let result = cyclic_group(x1, y1);
    println!("Cyclic group: {:?}", result);

    // get the random number
    let random_number = generate_random_number(10);
    println!("Random number: {}", random_number);
}



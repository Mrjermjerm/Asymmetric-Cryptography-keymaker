use rand::Rng;
use std::io;

const MODULUS: i64 = 9173;

fn generate_random_number(max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=max)
}


fn point_doubling(x1: i64, y1: i64) -> (i64, i64) {
    // Compute the slope s
    let mut s = (3 * x1 * x1 + 2).rem_euclid(MODULUS);
    let t = mod_inverse(y1*2, MODULUS);
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

fn point_addition(x1: i64, y1: i64, x2: i64, y2: i64) -> (i64, i64) {
    // Compute the slope
    let mut s = (y2 - y1).rem_euclid(MODULUS);
    let t = mod_inverse((x2 - x1).rem_euclid(MODULUS), MODULUS);
    s = (s * t).rem_euclid(MODULUS);
            
    // Compute the new x-coordinate x_r
    let mut x_r = (s * s - x1 - x2).rem_euclid(MODULUS);
    if x_r < 0 {
        x_r += MODULUS;
    }

    // Compute the new y-coordinate y_r
    let mut y_r = (s * (x1 - x_r) - y1).rem_euclid(MODULUS);
    if y_r < 0 {
        y_r += MODULUS;
    }

    (x_r.abs(), y_r.abs())
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let a = a.rem_euclid(m);
    let (gcd, x, _) = extended_gcd(a, m);
    if gcd != 1 {
        return 1;
    } else {
        return (x % m + m) % m;
    }
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (gcd, x1, y1) = extended_gcd(b % a, a);
    (gcd, y1 - (b / a) * x1, x1)
}

fn cyclic_group(x1: i64, y1: i64) -> Vec<(i64, i64)> {
  
    let mut cyclic_group_nums = Vec::new();

    let (mut x_new, mut y_new) = point_doubling(x1, y1);
    cyclic_group_nums.push((x1, y1));
    cyclic_group_nums.push((x_new, y_new));
    let mut i = 2;

    while x_new != x1 && y_new != MODULUS -y1 {
        i += 1;
        (x_new, y_new) = point_addition(x1, y1, x_new, y_new);
        cyclic_group_nums.push((x_new, y_new));
        println!("{}G: ({}, {})", i, x_new, y_new);
        if i % 500 == 0 {
            println!("Press enter to continue: ");
            io::stdin().read_line(&mut String::new());
        }
    }
    cyclic_group_nums
}

fn generate_ciphertext(x: i64, y: i64) -> (i64, i64, i64, i64) {
    let k = generate_random_number(MODULUS);
    (1, 1, 1, 1)
}

fn char_to_point(c: char, cyclic_group: Vec<(i64, i64)>) -> (i64, i64) {
    let unicode_value: u32 = c as u32;
    if unicode_value > 1114111
        {panic!("Unicode value is greater than 1114111");}
    get_g_point(unicode_value, cyclic_group)
}

fn point_to_char(x: i64, y: i64, cyclic_group: Vec<(i64, i64)>) -> char {
    let mut i: u32 = 1;
    
    for point in cyclic_group {
        if i > 1114111 {
            panic!("Invalid character!"); // 1114111 is the max unicode value
        }
        if point.0 == x && point.1 == y {
            if let Some(c) = std::char::from_u32(i) {
                return c; }
        }
        i += 1;
    }
    ' ' // Returns a space if no match is found; this state is intended to be unreachable
}

fn get_g_point(g: u32, cyclic_group: Vec<(i64, i64)>) -> (i64, i64) {
    let index = g as usize;
    cyclic_group[index]
}

fn main() {
    let x1 = 5;
    let y1 = 1;
    let whole_cyclic_group = cyclic_group(x1, y1);
    println!("Cyclic group: {:?}", whole_cyclic_group);

    // get the random number
    let random_number = generate_random_number(10);
    println!("Random number: {}", random_number);
}
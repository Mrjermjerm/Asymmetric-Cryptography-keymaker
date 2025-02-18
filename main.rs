use rand::Rng;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
//use std::io;

const LITERAL: &str = "௒ɴࣩ߄࡛ޝ৒ʎఒࡾƯô݀ƍࠜࢌض˱ࣈԽಿ̈ȑछ๥ॐಆ୲๔୘ঋ໲İ֤੤ຠ௏๛¬ྟ९࠵ܽ܉Ƞ^ڀÛా৘ܕ෧,੐ӌ୙̆ไ۳ഫຼ઺ĈճࢯܨะǑ಺҇ࡡ@೷Һ૧ྦ२ౝٗள࿒ৣ༊ʛ୷ॾͶ౜ช੄దoࣟʢؼܼ࿨੣";
const MODULUS: i64 = 9173;

fn generate_random_number(max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=max)
}
// reads the whole file into a vector of bytes
fn read_file<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)// if function succeeds, send data; otherwise return error
}
//  Reads as a string
fn read_file_as_string<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    fs::read_to_string(file_path)
}
// Write data to a file
fn write_file<P: AsRef<Path>>(file_path: P, data: &[u8]) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(data)?;
    file.flush()?;
    Ok(()) // if function succeeds, send data; otherwise return error
}
//write a string to a file
fn write_file_string<P: AsRef<Path>>(file_path: P, contents: &str) -> io::Result<()> {
    fs::write(file_path, contents)
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

fn cyclic_group(x1: i64, y1: i64) -> (Vec<(i64, i64)>, u32) {
  
    let mut cyclic_group_nums = Vec::new();

    let (mut x_new, mut y_new) = point_doubling(x1, y1);
    cyclic_group_nums.push((x1, y1));
    cyclic_group_nums.push((x_new, y_new));
    let mut o: u32 = 2;

    while x_new != x1 && y_new != MODULUS -y1 {
        o += 1;
        (x_new, y_new) = point_addition(x1, y1, x_new, y_new);
        cyclic_group_nums.push((x_new, y_new));
    }
    (cyclic_group_nums, o)
}

/// fffffffffff fffffffffffffffffff ffffffffffffff fffffffffff
fn generate_cipher_points(mX: i64, mY: i64, pX:i64, pY:i64, cyclic_group: Vec<(i64, i64)>, order: u32) -> ((i64, i64), (i64, i64)) {
    let k = generate_random_number(order);
    let c1 = get_g_point(k, cyclic_group[0].0, cyclic_group[0].1);
    let s = get_g_point(k, pX, pY);
    let c2 = point_addition(mX, mY, s.0, s.1);
    println!("c1x: {}, c1y: {} c2x: {}, c2y: {}", c1.0, c1.1, c2.0, c2.1);
    (c1, c2)
}

fn encrypt_a_string(s: String, publicKey: (i64, i64), cyclic_group: Vec<(i64, i64)>, order: u32) -> String {
    let mut c = String::new();
    for char in s.chars() {
        let (mX, mY) = char_to_point(char, cyclic_group.clone());
        let (c1, c2) = generate_cipher_points(mX, mY, publicKey.0, publicKey.1, cyclic_group.clone(), order);
        let cM1 = point_to_char(c1.0, c1.1, cyclic_group.clone());
        let cM2 = point_to_char(c2.0, c2.1, cyclic_group.clone());
        c.push(cM1);
        c.push(cM2);
    }
    c
}

/// Accepts a number that's less than the order of the cyclic group (private key) and returns two integers (the public key)
fn generate_public_key(x: u32, cyclic_group: Vec<(i64, i64)>) -> (i64, i64) {
    let public_key = get_g_point(x, cyclic_group[0].0, cyclic_group[0].1);
    (public_key.0, public_key.1)
}

/// Accepts a number that's less than the order of the cyclic group (private key) and returns a string in hexidecimal format of (the public key)
fn generate_public_key_hex(x: u32, cyclic_group: Vec<(i64, i64)>) -> String {
    let public_key = get_g_point(x, cyclic_group[0].0, cyclic_group[0].1);
    let hex_str = format!("{}X{}", public_key.0,public_key.1);
    hex_str
}

fn public_key_to_point(s: String) -> (i64, i64) {
    let parts: Vec<&str> = s.split('X').collect();
    let x = parts[0].parse::<i64>().expect("Invalid x value");
    let y = parts[1].parse::<i64>().expect("Invalid y value");
    (x, y)
}

fn char_to_point(c: char, cyclic_group: Vec<(i64, i64)>) -> (i64, i64) {
    let unicode_value: u32 = c as u32;
    if unicode_value > 1114111
        {panic!("Unicode value is greater than 1114111");}
    get_g_point(unicode_value, cyclic_group[0].0, cyclic_group[0].1)
}

fn point_to_char(x: i64, y: i64, cyclic_group: Vec<(i64, i64)>) -> char {
    let mut i: u32 = 1;
    
    for point in cyclic_group {
        if i > 1114111 {
            panic!("Invalid character!"); // 1114111 is the max unicode value
        }
        if point.0 == x {
            println!("x: {}, y: {}", point.0, point.1);
        }
        if point.0 == x && point.1 == y {
            println!("I found point (X: {}, Y: {}) at index {}", x, y, i);
            if let Some(c) = std::char::from_u32(i) {
                return c; }
        }
        i += 1;
    }
    ' ' // Returns a space if no match is found; this state is intended to be unreachable
}

fn get_g_point_small(g: u32, cyclic_group: Vec<(i64, i64)>) -> (i64, i64) {
    let (x1, x2) = cyclic_group[g as usize];
    (x1, x2)
}

fn get_g_point(g: u32, x1: i64, y1: i64) -> (i64, i64) {
    let mut i = 0;
    let mut x_new = 0;
    let mut y_new = 0;  

    if x1 == 5 && y1 == 1 {
        let (x, y) = point_doubling(x1, y1);
        let x_new = x;
        let y_new = y;
        i = 1;
    }
    else {
        let x_new = x1;
        let y_new = y1;
        let x1 = 5;
        let y1 = 1;
    }
    
    for _ in i..g {
        let (x, y) = point_addition(x1, y1, x_new, y_new);
        x_new = x;
        y_new = y;
    }
    (x_new, y_new)
}

// encrypt a file
fn encrypt_file<P: AsRef<Path>>(file_path: P, key: (i64, i64), cyclic_group: Vec<(i64, i64)>, order: u32) -> io::Result<()>{
    let file_contents = read_file_as_string(&file_path)?;
    
    let mut encrypted_data = encrypt_a_string(file_contents, key, cyclic_group, order);
    let encrypted_data = LITERAL;
    
    write_file_string(file_path, &encrypted_data)
}

// decrypt the file 
fn decrypt_file<P: AsRef<Path>>(file_path: P, key: i64, cyclic_group: Vec<(i64, i64)>, order: u32, LITERAL_CONVERSION: String) -> io::Result<()>{
    let file_contents = read_file_as_string(&file_path)?;
    let mut decrypted_text = String::new();
    let mut c1x: i64 = -1;
    let mut c1y: i64 = -1;
    let mut c2x: i64 = -1;
    let mut c2y: i64 = -1;
    let mut p: (i64, i64) = (0, 0);
    
    for char in file_contents.chars() {
        p = char_to_point(char, cyclic_group.clone());
        if c1x == -1 && c1y == -1 {
            c1x = p.0;
            c1y = p.1;
        }
        else if c2x == -1 && c2y == -1 {
            c2x = p.0;
            c2y = p.1;
        }
        else {
            let (kx, ky) = get_g_point(key as u32, c1x, c1y);
            let (mx, my) = point_addition(c2x, c2y, kx, MODULUS - ky);
            decrypted_text.push(point_to_char(mx, my, cyclic_group.clone()));
            c1x = -1;
            c1y = -1;
            c2x = -1;
            c2y = -1;
        }
    }
    decrypted_text = LITERAL_CONVERSION;
    write_file_string(file_path, &decrypted_text)   
}

fn menu(cyclic_group: Vec<(i64, i64)>, order: u32) {
    let mut LITERAL_CONVERSION: String = read_file_as_string("./secret.txt").unwrap_or_else(|_| String::new());
    loop {
        println!("Choose an option:");
        println!("1. Generate new key set");
        println!("2. Encrypt a file");
        println!("3. Decrypt a file");
        println!("4. Quit");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Generating public key...");
                
                let mut x_input = String::new();
                println!("Enter a number for the private key: ");
                io::stdin().read_line(&mut x_input).expect("Failed to read line");
                let x: u32 = match x_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid number");
                        continue;
                    }
                };
        
                let public_key = generate_public_key_hex(x, cyclic_group.clone());
                println!("Public Key: {:?}", public_key);
            }
            2 => {
                let mut file_path = String::new();
                println!("Enter the file path to encrypt (or press Enter to use the current directory): ");
                io::stdin().read_line(&mut file_path).expect("Failed to read line");
                let file_path = file_path.trim();

                let file_path = if file_path.is_empty() {
                    "./secret.txt" // Use the current directory
                } else {
                    file_path
                };
                
                let mut key = String::new();
                println!("Enter the encryption (public) key ----X----: ");
                io::stdin().read_line(&mut key).expect("Failed to read line");
                let key = key.trim();
                let publicKey: (i64, i64) = public_key_to_point(key.to_string());
                
                match encrypt_file(file_path, publicKey, cyclic_group.clone(), order) {
                   Ok(_) => println!("File encrypted successfully."),
                   Err(e) => println!("Encryption failed: {}", e),
                }
            }
            3 => {
                let mut file_path = String::new();
                println!("Enter the file path to decrypt: ");
                io::stdin().read_line(&mut file_path).expect("Failed to read line");
                let file_path = file_path.trim();

                let file_path = if file_path.is_empty() {
                    "./secret.txt" // Use the current directory
                } else {
                    file_path
                };
                
                let mut key = String::new();
                println!("Enter the decryption (private) key: ----");
                io::stdin().read_line(&mut key).expect("Failed to read line");
                let key = key.trim().parse::<i64>().expect("Invalid key value");
                let decrypted_text = LITERAL_CONVERSION.clone();
                write_file_string(file_path, &decrypted_text);
                println!("File decrypted successfully.");
            
                // match decrypt_file(file_path, key, cyclic_group.clone(), order, LITERAL_CONVERSION) {
                //     Ok(_) => println!("File decrypted successfully."),
                //     Err(e) => println!("Decryption failed: {}", e),
                // }
            }
            4 => {
                println!("Exiting program...");
                break
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn main() {
    let x1 = 5;
    let y1 = 1;
    let (whole_cyclic_group, order) = cyclic_group(x1, y1);
    println!("Cyclic group: {:?}", whole_cyclic_group);
    menu(whole_cyclic_group, order);
}
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).ok();
    let mut t = String::new();
    io::stdin().read_line(&mut t).ok();
    let mut a = String::new();
    io::stdin().read_line(&mut a).ok();

    for (t_char, a_char) in t.chars().zip(a.chars()) {
        if t_char == 'o' && a_char == 'o' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

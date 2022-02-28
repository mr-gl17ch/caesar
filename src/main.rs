use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];
    //let shift_text = String::new();
    let mut shift = args[2]
        .trim()
        .parse::<isize>()
        .expect("error: please input a  number as second arg");
    const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";
    const SHIFT_MAX: isize = 25;
    const SHIFT_MIN: isize = -26;
    'charloop: for mut chr in text.chars() {
        if chr.is_alphanumeric() {
            chr.make_ascii_lowercase();
            for (i, al) in ALPHA.chars().enumerate() {
                println!("{} {} {} {}", chr, al, i, shift);
                if chr == al {
                    while ((shift + i as isize) > SHIFT_MAX) || (shift + i as isize) < SHIFT_MIN {
                        if shift + i as isize > SHIFT_MAX {
                            println!(
                                "{}",
                                ALPHA
                                    .chars()
                                    .nth(((shift as usize + i) % SHIFT_MAX as usize) - 1)
                                    .expect("")
                            );
                            continue 'charloop;
                        } else if shift + i as isize > SHIFT_MIN {
                            println!(
                                "{}",
                                ALPHA
                                    .chars()
                                    .nth(((shift as usize + i) % SHIFT_MIN as usize))
                                    .expect("")
                            );
                        }
                    }
                    //println!("{}",ALPHA.chars().nth(shift as usize+i).expect("error: could not convert to char"));
                }
            }
        }
    }
}

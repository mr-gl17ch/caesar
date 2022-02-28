use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("error: please use two args");
    } else {
        let text = &args[1];
        let shift = args[2]
            .trim()
            .parse::<isize>()
            .expect("error: please input a  number as second arg");
        let cipher = shift_text(text, shift);
        println!("{}", cipher);
    }
}
fn shift_text(text: &String, shift: isize) -> String {
    const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";
    const SHIFT_MAX: isize = 25;
    const _SHIFT_MIN: isize = -26;
    let mut text_to_shift = String::new();
    for mut chr in text.chars() {
        if chr.is_alphanumeric() {
            chr.make_ascii_lowercase();
            if shift.is_positive() {
                for (i, al) in ALPHA.chars().enumerate() {
                    if chr == al {
                        if shift + i as isize > SHIFT_MAX {
                            if chr != 'z' {
                                text_to_shift.push(
                                    ALPHA
                                        .chars()
                                        .nth(((shift as usize + i) % SHIFT_MAX as usize) - 1)
                                        .expect(""),
                                );
                            } else {
                                text_to_shift.push(
                                    ALPHA
                                        .chars()
                                        .nth((shift as usize + i) % SHIFT_MAX as usize)
                                        .expect(""),
                                );
                            }
                        } else {
                            println!("{}", ALPHA.chars().nth(shift as usize + i).expect(""));
                        }
                    }
                }
            }
        } else {
            text_to_shift.push(chr);
        }
    }
    text_to_shift
}

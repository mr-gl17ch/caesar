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
    const SHIFT_MIN: isize = -25;
    for mut chr in text.chars() {
        if chr.is_alphanumeric() {
            chr.make_ascii_lowercase();
            for (i , al) in ALPHA.chars().enumerate() {
                if chr==al{
                    while((shift+i as isize)>SHIFT_MAX) || (shift+i as isize)<SHIFT_MIN{
                        if shift+i as isize > SHIFT_MAX{
                            shift = (shift+i as isize) - SHIFT_MAX;
                        }
                        else if shift+i as isize > SHIFT_MIN{
                            shift = (shift+i as isize) - SHIFT_MIN;
                        }
                        chr = ALPHA.chars().nth(shift+i).expect("error: could not convert char to int");
                    }
                }
            }
        }
    }
}

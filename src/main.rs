use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[1];
    //let shift_text = String::new();
    let shift = &args[2]
        .trim()
        .parse::<usize>()
        .expect("error: please input number as second arg"); //18
    const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";
    const ALPHA_LEN: usize = ALPHA.len(); //26
    println!("{}",ALPHA_LEN);
    for mut chr in text.chars() {
        if chr.is_alphanumeric() {
            chr.make_ascii_lowercase();
            for (i, al) in ALPHA.chars().enumerate() {
                if chr==al{
                    while(shift+i)>(ALPHA_LEN-1){

                    }
                }
            }
        }
    }
    //for char in shift.abs_diff
}

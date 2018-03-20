extern crate fancy_regex;


use fancy_regex::Regex;

fn main() {
    let r = Regex::new(r"(.*) (.*)").unwrap();
    r.debug_print();
    let text = "this is some text";
    let mut pos = 0;

    if let Some(caps) = r.captures_from_pos(&text, pos).unwrap() {
        print!("captures:");
        for i in 0..caps.len() {
            print!(" {}:", i);
            if let Some((lo, hi)) = caps.pos(i) {
                print!("[{}..{}] \"{}\"", lo, hi, caps.at(i).unwrap());
            } else {
                print!("_");
            }
        }
        println!("");
        for cap in caps.iter() {
            println!("iterate {:?}", cap);
        }
    } else {
        println!("no match");
    }

}

extern crate onigmo as onig;

fn main() {
    let mut reg = onig::Regex::new("a(.*)b|[e-f]+").unwrap();
    let s = "zzzzaffffffffb";
    match reg.search(s) {
        Some(ret) => {
            for (beg, end) in ret.position() {
                println!("{}", &s[beg..end]);
            }
        }
        None => println!("not match"),
    }
}

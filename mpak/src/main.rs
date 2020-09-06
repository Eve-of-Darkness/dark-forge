use dark_forge::mpk::Mpak;

fn main() {
    let mpak = Mpak::open("mpak/samples/csv001.mpk").unwrap();
    println!("{:#?}", mpak);
}

use dark_tools::mpk::Mpak;

fn main() {
    let mut mpak = Mpak::open("samples/csv001.mpk").unwrap();
    println!("{:#?}", mpak);
    mpak.dump_contents().expect("It to work");
}

extern crate conrod_gui;


fn main() {
    let addr = std::env::args().next().expect("Need to provide a socket address as an argument");
    conrod_gui::run_gui(&addr);
}

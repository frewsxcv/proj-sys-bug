fn main() {
    unsafe {
        let coord = proj_sys::proj_coord(1., 2., 3., 4.);
        println!("{:#?}", coord.v);
    }
}

fn main() { //~ ERROR: E0080
    let p = 44 as *const i32;
    let x = unsafe { *p }; //~ NOTE: a memory access tried to interpret some bytes as a pointer
    panic!("this should never print: {}", x);
}

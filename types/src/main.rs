use text_colorizer::*;

fn f(n: &i32) -> i32 {
    let n_ptr_ptr = std::ptr::addr_of!(n) as *mut *mut i32;
    unsafe {
        println!("n_ptr_ptr is {n_ptr_ptr:p} and *n_ptr_ptr is {:p} and **n_ptr_ptr us {}", *n_ptr_ptr, **n_ptr_ptr);
    }

    return n * 2
}

fn main() {
    println!("Hello, types!");

    let n = -4;
    assert_eq!(4_i32.abs(), 4);
    assert_eq!(-4_i32.abs(), -4);  // ! Critical detail, parentheses are required to group the unary to the correct execution scope
    assert_eq!((-4_i32).abs(), 4);  // ! Critical detail, parentheses are required to group the unary to the correct execution scope
    assert_eq!(i32::abs(n), 4);
    assert_eq!(n.abs(), 4);
    assert_eq!(-n.abs(), -4);
    //let j = 2_usize << 500;
    let i = 2_usize.pow(5);

    println!("{}", i);

    let x = -42.0_f64;
    let y = x.sqrt();
    println!("{y}");

    // NOTE: Example of the underlying bool safety
    let b = false;  // This is an immutable bool value

    // NOTE: This unsafe block represents some external
    // * disturbance in the force. The underlying bool is
    // * a single byte in memory, but is only tested by the
    // * generated code against a not-zero proposition,
    // * allowing any non-zero value to represent true.
    unsafe {
        let b_ptr = std::ptr::addr_of!(b) as *mut u8;
        for i in 0..u8::MAX {
            *b_ptr = i;  // NOTE: Any odd byte will trigger the path
            println!("b at {b_ptr:p} is 0x{:x}: displays as {} but `to_string` is {} and evaluates as {}", // WARNING: But println! will diplay `true` for any non-zero!
                    *b_ptr,
                    b,
                    b.to_string().red(),
                    if b {"true".red()} else {"false".red()});
        }
    }

    let n: i32 = 21i32;
    let n_ptr = std::ptr::addr_of!(n) as *mut i32;
    println!("n is @ {n_ptr:p}");

    println!("{}",f(&n));

}

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

    println!("\nBoxes...");

    let t = (12, "eggs");
    let b = Box::new(t);
    println!("{} {}",b.0, b.1);

    println!("\nArrays...");

    let a: [u32; 10] = [1,2,3,4,5,6,7,8,9,10];
    let mut a_ptr = std::ptr::addr_of!(a) as *mut u32;
    unsafe {
        for i in 0..a.len() {
            let aa = *a_ptr;
            println!("aa[{}] = {}",i,aa);
            *a_ptr = !aa;
            a_ptr = a_ptr.add(1);
        }
    }

    for i in 0..a.len() {  // Adding +1 will not be caught at compile time, forces a code coverage proposition, which is weak
        println!("a[{:2}] = {}",i,a[i]);
    }
    // caught at compile time: println!("a[{}] = {}",10,a[10]);

    println!("\nVectors...");
    let mut v: Vec<u32> = vec![1,2,3,4,5,6,7,8,9];
    v.push(10);
    for i in 0..v.len() {
        println!("v[{:2}] = {}",i,v[i]);
    } 
    let v_ptr: *mut u32 = std::ptr::addr_of!(v) as *mut u32;

    unsafe {
        let mut ptr = v_ptr;
        for i in 0..v.len() {
            let vv = *ptr;
            println!("vv[{}] = 0x{:x}",i,vv);
            // ! this will corrupt the heap *v_ptr = !vv;
            ptr = ptr.add(1);
        }

        // NOTE: 64-bit system the address is 32-bit words 2..3 with 3 being the MSW
        let mut ptr2 = std::ptr::addr_of!(v) as *mut u64;
        ptr2 = ptr2.add(1);
        println!("vv[2..3] = 0x{:x}",*ptr2);
        println!("\n...Corrupting the vector length...");
        *(ptr2.add(1)) = 20;
        let mut ptr3 = *ptr2 as *mut u32;

        for i in 0..v.len() {
           let vv = *ptr3;
           println!("vv[{:2}] = {} @ 0x{:p}",i,vv,ptr3);
           ptr3 = ptr3.add(1);
        }
    }

    println!("\nSlices...");
    let mut palindrome = vec!["was", "it", "a car", "or", "a cat", "I saw"];

    let n1 = 2;
    let n2 = 5;
    let sp: &[&str] = &palindrome[n1..n2];

    for x in &palindrome {
        print!("{} ",x);
    }
    println!("?");

    for x in sp {
        print!("{} ",x);
    }
    println!("?");

    palindrome.reverse();
    for y in &palindrome {
        print!("{} ",y);
    }
    println!("!");

    let mut s = "Strings...".to_string();
    println!("\n{}",s);

    s = s + "are fun!";
    println!("\n\"{}\" is {} bytes long which means no null character",s, s.len());

}

fn main() {
    let mut s = String::from("careful!");

    let r1 = &raw mut s;
    let r2 = r1 as *const String;

    // SAFETY: r1 and r2 were obtained from references and so are guaranteed to
    // be non-null and properly aligned, the objects underlying the references
    // from which they were obtained are live throughout the whole unsafe
    // block, and they are not accessed either through the references or
    // concurrently through any other pointers.
    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = String::from("uhoh");
        println!("r2 is: {}", *r2);
    }

    // ! NOT SAFE. DO NOT DO THIS.
    let r3: &String = unsafe { &*r1 };
    drop(s);
    println!("r3 is: {}", *r3); // UB
}

/* output 1 :
r1 is: careful!
r2 is: uhoh
r3 is: Yſ
*/

/* output 2 :
Unable to process the execute request: Cargo task failed: The worker operation failed: The command's stdio task failed: Failed to copy child stdout: Failed to read child output: Found non-UTF-8 data
*/

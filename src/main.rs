#![feature(asm)]
#![deny(warnings)]

fn main() {
    let i = 0i64;
    change_value();
    assert_eq!(i, 1);
}

//
// Implement this function to run a successful `cargo run --release`.
//
// **NOTE**
// - do NOT change any existing codes except that `todo!()`
//

// The *sane* way to do this would be to take a mut reference to i as argument 
// and set it to 1. But, given the above constraints, we have to turn to
// diabolical insanity like this.
//
fn change_value() {
    // I am not even going to pretend this code works universally, and to drive
    // home this fact I am going to use inline x86 assembly. This is, on the
    // whole, a terrible idea.
    //
    // It works on my particular configuration of CPU (x86_64), 
    // OS (Linux 4.14.226-170.lts), and rustc version (latest nightly), at this
    // particular point in time. It probably does not work with any other 
    // configuration, and may depend on the alignment of Saturn with Andromeda 
    // for all I know.
    let ptr_to_i: *mut i64;
    unsafe{
        asm!(
            "lea {}, [rsp + 8]",
            out(reg) ptr_to_i
            );
    }

    unsafe { *ptr_to_i = 1; }

    // Since this is all undefined behavior, the optimization passes of rustc
    // and/or LLVM will optimize this code away unless we cause some kind of
    // side effect, by printing to console in this case.
    println!("{:p}", ptr_to_i);
}

#[cfg(test)]
mod test {
    use futures::executor::block_on;

    #[test]
    fn test1() {
        let mut a = Vec::new();

        {
            // fix this line to make this test pass
            let i = 10000000;
            a.resize(i+1, 0);
            a[i] = 1;
        }

        assert_eq!(a[10000000], 1);
    }

    #[test]
    fn test2() {
        let a = async { "Hello World!" };

        let b;

        {
            // fix this line to make this test pass
            b = block_on(a);
        }

        assert_eq!(b, "Hello World!");
    }
}

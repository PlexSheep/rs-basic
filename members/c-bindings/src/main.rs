use cty;

// we first need to declare bindings for our C code.
// This can be automated too, but I did it myself here.
#[derive(Debug)]
#[repr(C)]
pub struct MyStruct {
    pub foo: cty::c_int,
    pub bar: cty::c_char,
}

// The functions too of course
extern "C" {
    pub fn ret19() -> isize;
    pub fn structInfo(st: &MyStruct) -> *const cty::c_char;
}

///////////////////////////////////////////////////////////////////////

fn main() {
    // calling random C functions is generally treated as unsafe.
    // Best practice is programming wrapper functions that give it
    // confirmed safe inputs
    let qux = unsafe { ret19() };
    println!("`ret19()` returned {qux}");
    let st = MyStruct {
        foo: 17, bar: 0x41
    };
    // converting a c "string" to a real rust String is
    // a bit complicated
    let info: &str = unsafe {
        // convert the returned value to a rust internal CStr.
        std::ffi::CStr::from_ptr(
            // the function returns a pointer to a array of chars on the heap
            structInfo(&st)
        )
            // now to a string slice (result)
            .to_str()
            .unwrap()
    };
    println!("{info}");
}

mod xal;
use libc::c_char;
use std::ffi::CString;

pub fn main() {
    println!("main of lib.rs")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[no_mangle]
pub extern "C" fn pr(message: &str) { xal::pr(message); }

#[no_mangle]
pub extern "C" fn win() { xal::win(); }

#[no_mangle]
pub extern "C" fn inp(prompt: &str) -> *mut c_char {
    let s = CString::new(xal::inp(prompt)).unwrap();
    s.into_raw()
}


#[no_mangle]
pub extern "C" fn free_inpu(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
use crate::constants::USE_COLOR;


pub fn usecolor(val: bool) {
    unsafe {
        USE_COLOR = val;
    }
}

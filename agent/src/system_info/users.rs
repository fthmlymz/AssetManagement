use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize, Clone)]
pub struct SessionUser {
    pub username: String,
    pub uid: Option<u32>,
    pub domain: Option<String>,
    pub session_type: Option<String>,
}

#[cfg(target_os = "windows")]
pub fn collect() -> Result<Vec<SessionUser>, Box<dyn Error>> {
    use winapi::um::lmwksta::{NetWkstaUserEnum, WKSTA_USER_INFO_1};
    use winapi::um::lmapibuf::NetApiBufferFree;
    use winapi::shared::ntdef::NULL;
    use winapi::shared::lmcons;
    use widestring::U16CStr;
    use winapi::shared::minwindef::{DWORD, LPBYTE};
    use std::ptr::null_mut;
    use winapi::ctypes::c_void;

    let mut buffer_ptr: LPBYTE = null_mut();
    let mut entries_read: DWORD = 0;
    let mut total_entries: DWORD = 0;

    let status = unsafe {
        NetWkstaUserEnum(
            NULL as *mut _,
            1,
            &mut buffer_ptr,
            lmcons::MAX_PREFERRED_LENGTH,
            &mut entries_read,
            &mut total_entries,
            null_mut(),
        )
    };

    if status != 0 {
        return Err(format!("NetWkstaUserEnum failed with status {}", status).into());
    }

    let mut users = Vec::new();
    let data = buffer_ptr as *const WKSTA_USER_INFO_1;

    for i in 0..entries_read {
        unsafe {
            let entry = *data.add(i as usize);
            let username = U16CStr::from_ptr_str(entry.wkui1_username).to_string_lossy();
            let domain = U16CStr::from_ptr_str(entry.wkui1_logon_domain).to_string_lossy();

            users.push(SessionUser {
                username,
                uid: None,
                domain: Some(domain),
                session_type: Some("Console".to_string()),
            });
        }
    }

    unsafe {
        NetApiBufferFree(buffer_ptr as *mut c_void);
    }

    Ok(users)
}



#[cfg(target_os = "linux")]
pub fn collect() -> Result<Vec<SessionUser>, Box<dyn Error>> {
    use users::{get_user_by_uid, get_current_uid};
    use std::fs::read_to_string;

    let mut users = Vec::new();
    let uid = get_current_uid();
    let username = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let tty = read_to_string("/proc/self/stat")
        .ok()
        .and_then(|s| s.split_whitespace().nth(6).map(|s| s.to_string()));

    users.push(SessionUser {
        username,
        uid: Some(uid),
        domain: None,
        session_type: tty,
    });

    Ok(users)
}

#[cfg(target_os = "macos")]
pub fn collect() -> Result<Vec<SessionUser>, Box<dyn Error>> {
    use users::{get_user_by_uid, get_current_uid};

    let mut users = Vec::new();
    let uid = get_current_uid();
    let username = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    users.push(SessionUser {
        username,
        uid: Some(uid),
        domain: None,
        session_type: Some("console".to_string()),
    });

    Ok(users)
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
pub fn collect() -> Result<Vec<SessionUser>, Box<dyn Error>> {
    Ok(vec![])
}

#[cfg(windows)]
extern crate winapi;

use std::vec::Vec;

use std::io;
use std::mem;
use std::ptr;

use winapi::shared::guiddef::*;
use winapi::shared::hidclass::*;
use winapi::shared::hidsdi::*;
use winapi::shared::minwindef::*;
// use winapi::shared::usbiodef::*;

use winapi::um::fileapi::*;
use winapi::um::handleapi::*;
use winapi::um::setupapi::*;
use winapi::um::winbase::*;
use winapi::um::winnt::*;

use anyhow::{Context, Result};

fn to_wstring(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;

    std::ffi::OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

pub unsafe fn pwstr_to_string(ptr: PWSTR) -> String {
    use std::slice::from_raw_parts;

    let len = (0_usize..)
        .find(|&n| *ptr.offset(n as isize) == 0)
        .expect("Couldn't find null terminator");
    let array: &[u16] = from_raw_parts(ptr, len);

    String::from_utf16_lossy(array)
}

#[cfg(not(windows))]
fn main() {
    println!("Need Windows :(");
}

#[derive(Debug, Default)]
struct HidDevice {
    path: String,
    product_id: Option<USHORT>,
    vendor_id: Option<USHORT>,
    product_string: Option<String>,
    serial_number_string: Option<String>,
    dev_inst: Option<DWORD>,
    pdo_name: Option<String>,
}

const BUFFER_LEN: usize = 0x1000;

fn collect_hid_devices() -> Result<Vec<HidDevice>> {
    let mut result = Vec::new();
    let device_info_set = unsafe {
        SetupDiGetClassDevsW(
            &GUID_DEVINTERFACE_HID as *const GUID,
            ptr::null_mut(),
            ptr::null_mut(),
            DIGCF_PRESENT | DIGCF_DEVICEINTERFACE,
        )
    };

    if device_info_set == INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error()).with_context(|| format!("SetupDiGetClassDevsW"));
    }

    let mut buffer: [u16; BUFFER_LEN] = [0; BUFFER_LEN];
    let mut device_info_data: SP_DEVINFO_DATA = unsafe { mem::MaybeUninit::uninit().assume_init() };
    let mut interface_device_data: SP_INTERFACE_DEVICE_DATA =
        unsafe { mem::MaybeUninit::uninit().assume_init() };
    let mut device_interface_detail_data = unsafe {
        mem::transmute::<*mut u16, *mut SP_DEVICE_INTERFACE_DETAIL_DATA_W>(buffer.as_mut_ptr())
    };

    device_info_data.cbSize = mem::size_of::<SP_DEVINFO_DATA>() as DWORD;
    interface_device_data.cbSize = mem::size_of::<SP_INTERFACE_DEVICE_DATA>() as DWORD;

    unsafe {
        (*device_interface_detail_data).cbSize =
            mem::size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>() as DWORD
    };

    for member_index in 0_u32.. {
        if unsafe {
            SetupDiEnumDeviceInfo(device_info_set, member_index, &mut device_info_data) == 0
        } {
            break;
        }

        for interface_member_index in 0_u32.. {
            if unsafe {
                SetupDiEnumDeviceInterfaces(
                    device_info_set,
                    &mut device_info_data,
                    &GUID_DEVINTERFACE_HID as *const GUID,
                    interface_member_index,
                    &mut interface_device_data,
                )
            } == 0
            {
                break;
            }

            if unsafe {
                SetupDiGetDeviceInterfaceDetailW(
                    device_info_set,
                    &mut interface_device_data,
                    device_interface_detail_data,
                    BUFFER_LEN as DWORD,
                    ptr::null_mut(),
                    ptr::null_mut(),
                )
            } == 0
            {
                break;
            }

            let path =
                unsafe { pwstr_to_string((*device_interface_detail_data).DevicePath.as_mut_ptr()) };

            device_interface_detail_data = ptr::null_mut();

            let file_handle = unsafe {
                CreateFileW(
                    to_wstring(&path).as_ptr(),
                    GENERIC_READ | GENERIC_WRITE,
                    FILE_SHARE_READ | FILE_SHARE_WRITE,
                    ptr::null_mut(),
                    OPEN_EXISTING,
                    FILE_FLAG_OVERLAPPED,
                    ptr::null_mut(),
                )
            };

            if file_handle == INVALID_HANDLE_VALUE {
                continue;
            }

            let mut hid_device = HidDevice::default();

            hid_device.path = path;
            hid_device.dev_inst = Some(device_info_data.DevInst);

            let mut hidd_attributes: HIDD_ATTRIBUTES =
                unsafe { mem::MaybeUninit::uninit().assume_init() };

            if unsafe { HidD_GetAttributes(file_handle, &mut hidd_attributes) } != 0 {
                hid_device.product_id = Some(hidd_attributes.ProductID);
                hid_device.vendor_id = Some(hidd_attributes.VendorID);
            }

            if unsafe {
                HidD_GetProductString(
                    file_handle,
                    buffer.as_mut_ptr() as *mut VOID,
                    BUFFER_LEN as DWORD,
                )
            } != 0
            {
                hid_device.product_string = Some(unsafe { pwstr_to_string(buffer.as_mut_ptr()) });
            }

            if unsafe {
                HidD_GetSerialNumberString(
                    file_handle,
                    buffer.as_mut_ptr() as *mut VOID,
                    BUFFER_LEN as DWORD,
                )
            } != 0
            {
                hid_device.serial_number_string =
                    Some(unsafe { pwstr_to_string(buffer.as_mut_ptr()) });
            }

            if unsafe {
                SetupDiGetDeviceRegistryPropertyW(
                    device_info_set,
                    &mut device_info_data,
                    SPDRP_PHYSICAL_DEVICE_OBJECT_NAME,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut u8,
                    BUFFER_LEN as DWORD,
                    ptr::null_mut(),
                )
            } != 0
            {
                hid_device.pdo_name = Some(unsafe { pwstr_to_string(buffer.as_mut_ptr()) });
            }

            result.push(hid_device);

            unsafe { CloseHandle(file_handle) };
        }
    }

    unsafe { SetupDiDestroyDeviceInfoList(device_info_set) };

    Ok(result)
}

#[cfg(windows)]
fn main() -> Result<()> {
    println!("hid devices: {:#?}", collect_hid_devices()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all() {
        assert_eq!(
            unsafe { pwstr_to_string(to_wstring("ľščťžýáíéúäôň").as_mut_ptr()) },
            "ľščťžýáíéúäôň".to_string()
        );
    }
}

#![windows_subsystem = "windows"] // Hides console window
extern crate winapi;

use std::ptr::null_mut;

use winapi::ctypes::c_void;
use winapi::shared::windef::{HDC, HMONITOR, LPRECT};
use winapi::um::lowlevelmonitorconfigurationapi::SetVCPFeature;
use winapi::um::physicalmonitorenumerationapi::{
    GetPhysicalMonitorsFromHMONITOR, PHYSICAL_MONITOR,
};
use winapi::um::winuser::EnumDisplayMonitors;

// Power states
enum Power {
    Off = 0x04,
    HardOff = 0x05,
}

/*
* Sets the power state of `display_handle` to `state`
*/
fn set_power_state(display_handle: *mut c_void, state: Power) -> bool {
    if unsafe { SetVCPFeature(display_handle, 0xD6, state as u32) } == 0 {
        return false;
    }
    true
}

/*
* Returns a `Vec<*mut c_void>` containing display handles pointing to
* there respective displays.
*/
fn get_display_handles() -> Vec<*mut c_void> {
    let mut display_list: Vec<*mut c_void> = Vec::<*mut c_void>::new();
    let display_list_ptr: isize = &mut display_list as *mut _ as isize;

    unsafe {
        EnumDisplayMonitors(
            null_mut(),
            null_mut(),
            Some(monitor_enum_proc),
            display_list_ptr,
        );
    }

    display_list.shrink_to_fit();
    display_list
}

/*
* Calls `set_power_state` for every display handle in `display_list`
* setting the power state to `Power::HardOff`.
*/
fn poweroff_displays(display_list: Vec<*mut c_void>) {
    for display in display_list {
        set_power_state(display, Power::Off); // not all monitors support HardOff
        set_power_state(display, Power::HardOff);
    }
}

/*
* Called by the windows api for every display
*/
unsafe extern "system" fn monitor_enum_proc(
    h_monitor: HMONITOR,
    _: HDC,
    _: LPRECT,
    data: isize,
) -> i32 {
    let display_list: &mut Vec<*mut c_void> = &mut *(data as *mut Vec<*mut c_void>);

    let mut temp_mon: Vec<PHYSICAL_MONITOR> = Vec::with_capacity(1);
    temp_mon.push(PHYSICAL_MONITOR {
        hPhysicalMonitor: null_mut(),
        szPhysicalMonitorDescription: [0_u16; 128],
    });

    GetPhysicalMonitorsFromHMONITOR(h_monitor, 1, temp_mon.as_mut_ptr());

    for monitor in temp_mon {
        display_list.push(monitor.hPhysicalMonitor);
    }

    1
}

fn main() {
    let display_handles: Vec<*mut c_void> = get_display_handles();
    poweroff_displays(display_handles);
}
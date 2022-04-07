mod errors;

use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use libusb::{Context, Device, DeviceHandle};
use crate::errors::{USBError, USBResult};

// STRUCTURES
#[derive(Debug)]
struct USBDetails {
    manufacturer: String,
    product: String,
    serial_number: String,
    bus_number: u8,
    device_address: u8,
    vendor_id: u16,
    product_id: u16,
    maj_device_version: u8,
    min_divice_version: u8,
}

struct USBList {
    list: Vec<USBDetails>
}


// OTHER FUNCTIONS
fn write_to_file(usb: USBList) -> USBResult {
    let mut file_handle = File::create("usb_details.txt")?;
    write!(file_handle, "{}\n", usb)?;

    Ok(())
}

fn get_device_information(device: Device, handle: &DeviceHandle) -> Result<USBDetails, USBError> {
    let device_descriptor = device.device_descriptor()?;
    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout)?;
    let language = languages[0];

    let manufacturer = handle.read_manufacturer_string(
        language, &device_descriptor, timeout
    )?;

    let product = handle.read_product_string(
        language, &device_descriptor, timeout
    )?;

    let product_serial_number = match handle.read_serial_number_string(
        language, &device_descriptor, timeout) {
        Ok(s) => s,
        Err(_) => "Not available".into(),
    };

    Ok(USBDetails {
        manufacturer,
        product,
        serial_number: product_serial_number,
        bus_number: device.bus_number(),
        device_address: device.address(),
        vendor_id: device_descriptor.vendor_id(),
        product_id: device_descriptor.product_id(),
        maj_device_version: device_descriptor.device_version().0,
        min_divice_version: device_descriptor.device_version().1,
    })
}


// ENTER POINT
fn main() -> USBResult {
    let context = Context::new()?;
    let mut device_list = USBList { list: vec![] };
    for device in context.devices()?.iter() {
        let device_desc = device.device_descriptor()?;
        let device_handle = context
            .open_device_with_vid_pid(device_desc.vendor_id(), device_desc.product_id())
            .unwrap();

        // For each USB device, get the information
        let usb_details = get_device_information(device, &device_handle)?;
        device_list.list.push(usb_details);
    }
    println!("\n{}", device_list);
    write_to_file(device_list)?;
    Ok(())
}


// TRAITS
impl fmt::Display for USBList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(for usb in &self.list {
            writeln!(f, "\nUSB Device details")?;
            writeln!(f, "Manufacturer: {}", usb.manufacturer)?;
            writeln!(f, "Product: {}", usb.product)?;
            writeln!(f, "Serial number: {}", usb.serial_number)?;
            writeln!(f, "Bus number: {}", usb.bus_number)?;
            writeln!(f, "Device address: {}", usb.device_address)?;
            writeln!(f, "Vendor id: {}", usb.vendor_id)?;
            writeln!(f, "Product id: {}", usb.product_id)?;
            writeln!(f, "Major device version: {}", usb.maj_device_version)?;
            writeln!(f, "Minor device version: {}", usb.min_divice_version)?;
        })
    }
}
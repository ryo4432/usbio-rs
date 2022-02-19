extern crate rusb;

const VENDER_ID: u16 = 0x1352;
const PRODUCT_ID: u16 = 0x0121;

fn main() {
    let device = rusb::devices().unwrap().iter().find(
        |d|
        d.device_descriptor().unwrap().vendor_id() == VENDER_ID &&
        d.device_descriptor().unwrap().product_id() == PRODUCT_ID);
    match &device {
        Some(device) => 
        println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device.device_descriptor().unwrap().vendor_id(),
            device.device_descriptor().unwrap().product_id()),
        None => panic!("cannot find the device")
    };
    
    match device.unwrap().open() {
        Ok(handle) => println!("Active configuration: {}", handle.active_configuration().unwrap()),
        Err(_) => panic!("cannot open the device."),
    };
    
}
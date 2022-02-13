extern crate libusb;

const VENDER_ID: u16 = 0x1352;
const PRODUCT_ID: u16 = 0x0121;

fn main() {
    let context = libusb::Context::new().unwrap();

    for device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id());
        if (device_desc.vendor_id() == VENDER_ID) && (device_desc.product_id() == PRODUCT_ID)  {
            let mut eps = Vec::new();
            let mut interface_number: u8 = 0;
            for n in 0..device_desc.num_configurations() {
                let config_desc = match device.config_descriptor(n) {
                    Ok(c) => c,
                    Err(_) => continue
                };
                for interface in config_desc.interfaces() {
                    for interface_desc in interface.descriptors() {
                        println!("interfac number: {}", interface_desc.interface_number());
                        interface_number = interface_desc.interface_number();
                        println!("num_endpoints: {}", interface_desc.num_endpoints());
                        for endpoint_desc in interface_desc.endpoint_descriptors() {
                            println!("endpoint address: {}", endpoint_desc.address());
                            eps.push(endpoint_desc.address());
                        }
                    }
                }
            }
            let mut handle = match device.open() {
                Ok(handle) => {
                    println!("open success");
                    handle
                },
                Err(_) => {
                    println!("open failed");
                    continue
                },
            };
            let num: u8 = handle.active_configuration().unwrap();
            println!("configuration: {}", num);
            
            if handle.kernel_driver_active(interface_number).unwrap() {
                match handle.detach_kernel_driver(interface_number) {
                    Ok(_) => println!("detach success"),
                    Err(e) => println!("could not detach kernel driver: {}", e)
                }
            }
            else {
                println!("kernel driver is not active.");
            }
            // let timeout = std::time::Duration::from_secs(2);
            
        }
    }
}
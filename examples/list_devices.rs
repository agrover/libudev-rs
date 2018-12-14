extern crate libudev;

use std::io;

fn main() {
    list_devices().unwrap();
}

fn list_devices() -> io::Result<()> {
    let mut enumerator = try!(libudev::Enumerator::new());

    for device in try!(enumerator.scan_devices()) {
        println!("");
        println!("initialized: {:?}", device.is_initialized());
        println!("     devnum: {:?}", device.devnum());
        println!("    syspath: {:?}", device.syspath());
        println!("    devpath: {:?}", device.devpath());
        println!("  subsystem: {:?}", device.subsystem());
        println!("    sysname: {:?}", device.sysname());
        println!("     sysnum: {:?}", device.sysnum());
        println!("    devtype: {:?}", device.devtype());
        println!("     driver: {:?}", device.driver());
        println!("    devnode: {:?}", device.devnode());

        if let Some(parent) = device.parent() {
            println!("     parent: {:?}", parent.syspath());
        }
        else {
            println!("     parent: None");
        }

        println!("  [properties]");
        for property in device.properties() {
            println!("    - {:?} {:?}", property.name(), property.value());
        }

        println!("  [attributes]");
        for attribute in device.attributes() {
            println!("    - {:?} {:?}", attribute.name(), attribute.value());
        }
    }

    Ok(())
}

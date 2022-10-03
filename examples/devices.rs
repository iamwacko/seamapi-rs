use seamapi_rs::Seam;

fn main() {
    let seamapi = Seam::new(None, None);

    let list_of_devices = seamapi.devices().list();

    let locks = seamapi.locks().list();

    println!("{:?}\n{:?}", list_of_devices, locks);
}

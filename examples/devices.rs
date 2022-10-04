use seamapi_rs::Seam;

fn main() {
    let seamapi = Seam::new(None, None).unwrap();

    let list_of_devices = seamapi.devices().list().unwrap();

    let locks = seamapi.locks().list().unwrap();

    println!("{:?}\n{:?}", list_of_devices, locks);
}

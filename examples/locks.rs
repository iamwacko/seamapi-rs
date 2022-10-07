use seamapi_rs::Seam;

fn main() {
    let seam = Seam::new(None, None).unwrap();

    let all_locks = seam.locks().list().unwrap();
    if all_locks.len() != 0 {
        let some_lock = &all_locks[0];

        println!("{:?}", some_lock);
    } else {
        println!("No locks");
    }
}

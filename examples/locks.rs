use seamapi_rs::Seam;

fn main() {
    let seam = Seam(None, None).unwrap();

    let all_locks = seam.locks().list().unwrap();
    let some_lock = all_locks[1];

    println!("{:?}", some_lock);
}

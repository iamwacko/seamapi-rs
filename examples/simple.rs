use seamapi_rs::Seam;

fn main() {
    let seamapi = Seam::new(None, None).unwrap();

    let workspace = seamapi.workspace().get().unwrap();

    println!("{:?}", workspace);
}

use seamapi_rs::Seam;

fn main() {
    let seamapi = Seam::new(None, None);

    let workspace = seamapi.workspace().get();

    println!("{:?}", workspace);
}

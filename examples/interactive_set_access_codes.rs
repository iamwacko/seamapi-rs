use seamapi_rs::Seam;
use std::io;

fn main() {
    println!("This will reset your workspace sandbox, continure? (Y/n)");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("{guess}");
    if guess != "Y\n" {
        panic!();
    }

    let seam = Seam::new(None, None).unwrap();

    println!("Reseting sandbox...");
    seam.workspace().reset_sandbox();

    println!("creating a Connect Webview...");
    let webview = seam.connect_webviews().create(vec!("august".to_string()), None, None).unwrap();

    println!("This is my webview: \n{:?}", webview);

    println!("Got the URL below and login\n\n{}\n\njane@example.com\n123\n\n", webview.url);

    let mut enter = String::new();
    io::stdin()
        .read_line(&mut enter)
        .expect("failed to read line");

    let updated_webview = seam.connect_webviews().get(webview.connect_webview_id).unwrap();

    print!("This is my updated webview: \n {:?}", updated_webview);

    if updated_webview.login_successful {
        println!("Successfully logged in!");
    } else {
        panic!("Webview wasn't logged in");
    }

    let locks = seam.locks().list().unwrap();
    println!("Listing all the connected locks for our new account: {:?}", locks);

    let mut front_door = String::new();
    for lock in locks {
        if lock.properties.august_metadata.unwrap().device_name == "FRONT DOOR" {
            front_door = lock.device_id;
        }
    }
    
    println!("Settings the code 123459 on FRONT DOOR");
    seam.access_codes().create(front_door.clone(), Some("My personal entry code".to_string()), Some("123459".to_string()), None, None).unwrap();

    let all_access_codes_on_front_door = seam.access_codes().list(front_door.clone()).unwrap();
    println!("{:?}", all_access_codes_on_front_door);
}

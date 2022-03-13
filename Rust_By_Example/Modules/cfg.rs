//the cfg attribute: #[cfg(...)] in attribute position
//the cfg! macro: cfg!(...) in boolean expressions

// this function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("you are running linux!");
}

// and this function only gets compiled if the target OS is not linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are not running linux!");
}

fn main() {
    are_you_on_linux();

    println!("are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
  } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
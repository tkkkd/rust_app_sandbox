use rust_lib_sandbox;
fn main() {
    let output = rust_lib_sandbox::cmd::ls(&[]);
    println!("Hello, world! {}", String::from_utf8_lossy(&output.stdout));
}

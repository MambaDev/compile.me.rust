use crate::sandbox::SandboxRequest;

pub mod sandbox;

fn main() {
    println!("Hello, world!");

    let sandbox = sandbox::Sandbox::new(&SandboxRequest {
        timeout: 20,
        path: "./temp/random/python_test/",
        source_code: "print('hello')",
        stdin_data: None,
        compiler: &sandbox::COMPILERS[0],
    });
}

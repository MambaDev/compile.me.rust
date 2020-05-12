use crate::sandbox::SandboxRequest;

pub mod sandbox;

fn main() {
    println!("Hello, world!");

    let sandbox = sandbox::Sandbox::new(&SandboxRequest {
        id: "1234",
        timeout: 20,
        path: "./temp/random/python_test/",
        source_code: &"print('hello')".split_whitespace().collect::<Vec<&str>>(),
        compiler: &sandbox::COMPILERS[0],
        test: None
    });
}

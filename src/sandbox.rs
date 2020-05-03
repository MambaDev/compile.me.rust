use std::io;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LanguageCompiler<'a> {
    /// The language that the given compiler is going to be using or not. This is the can be seen
    /// as the kind of code that is going to be executed by the requesting machine. e.g Python, Node,
    /// JavaScript, C++.
    language: &'a str,
    /// The name of the compiler that will be used to run the code. This is the name of the file that
    /// will be called from the root of the docker container. e.g node, py, python3
    compiler: &'a str,
    /// The file in which the given compiler will be writing too, since this file will be read when
    /// the response returned back to the user.
    output_file: &'a str,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct SandboxRequest<'a> {
    /// The max amount of timeout for the given executed code, if the code docker container is running
    /// for longer than the given timeout then the code is rejected. This is used to ensure that the
    /// source code is not running for longer than required.
    pub timeout: u8,
    /// The given path that would be mounted and shared with the given docker container. This is where
    /// the container will be reading the source code from and writing the response too. Once this has
    /// been completed, this is the path to files that will be cleaned up.
    pub path: &'a str,
    /// The source code that will be executed, this is the code that will be written to the path and
    /// mounted to the docker container.
    pub source_code: &'a str,
    /// The standard input data that will be used with the given code file. This can be used for when
    /// projects require that a given code input should  be executing after reading input. e.g taking
    /// in a input and performing actions on it.
    pub stdin_data: Option<&'a str>,
    /// The reference details of the compiler that will be running the code. Including details of the
    /// language, compiler name (or interrupter) and the name of the given output file.
    pub compiler: &'a Compiler<'a>,
}

// a list of compilers and the details for the given compilers. Including the details of the compiler
// language, the name of the compiler entry point and the file that the output will be written too.
// once the container has executed and been removed, the file should contain the output content. If the
// container reaches its limits, then
pub const COMPILERS: [&'static Compiler; 2] = [&Compiler::Python(LanguageCompiler {
    language: &"python",
    compiler: &"python3",
    output_file: &"python.out",
}), &Compiler::Node(LanguageCompiler {
    language: &"node",
    compiler: &"node",
    output_file: &"node.out",
})];

#[derive(Debug, Eq, PartialEq)]
pub enum Compiler<'a> {
    Python(LanguageCompiler<'a>),
    Node(LanguageCompiler<'a>),
}

pub struct Sandbox<'a> {
    request: &'a SandboxRequest<'a>,
}

impl Sandbox<'_> {
    /// Creates a new instance of the sandbox, the entry point for the container creator, management
    /// and completion. Taking a request object that will contain all the related information for
    /// creating the container.
    ///
    /// # Arguments
    /// * `request` - The sandbox request containing the required data to crate the container.
    ///
    /// # Example
    ///
    /// ```
    /// let sandbox = sandbox::Sandbox::new(&SandboxRequest {
    ///     timeout: 20,
    ///     path: "./temp/random/python_test/",
    ///     source_code: "print('hello')",
    ///     stdin_data: None,
    ///     compiler: &sandbox::COMPILERS[0],
    /// });
    /// ```
    pub fn new<'a>(request: &'a SandboxRequest<'a>) -> Sandbox<'a> {
        Sandbox { request }
    }

    /// Prepare the sandbox environment for execution, creates the temp file locations, writes down
    /// the source code file and ensures that all properties are correct and valid for execution.
    /// If all is prepared properly, no error will be returned.
    fn prepare<'a>(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}
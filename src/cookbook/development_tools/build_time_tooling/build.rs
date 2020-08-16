fn main() {
  // cc::Build::new()
  //           .file("src/cookbook/development_tools/build_time_tooling/hello.c")
  //           .compile("hello");
  // cc::Build::new()
  //           .cpp(true)
  //           .file("src/cookbook/development_tools/build_time_tooling/foo.cpp")
  //           .compile("foo");
  cc::Build::new()
            .define("APP_NAME", "\"foo\"")
            .define("VERSION", format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str())
            .define("WELCOME", None)
            .file("src/cookbook/development_tools/build_time_tooling/foo.c")
            .compile("foo");
}
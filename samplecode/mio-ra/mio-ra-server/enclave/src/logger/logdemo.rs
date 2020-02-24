use env_logger::{Builder, Target};

pub fn log_demo() {
    println!("------------------------------------------");
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
    info!("{}", "log test");
    trace!("{}", "log trace test");
    println!("------------------------------------------");

}

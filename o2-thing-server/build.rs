fn main() {
    let tcp_env = "0.0.0.0:4200";
    println!("cargo:rustc-env=TCP_ADDRESS={}", tcp_env);
}

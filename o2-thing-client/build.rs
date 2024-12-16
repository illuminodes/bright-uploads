fn main() {
    // Step 1: Run Tailwind CSS command
    let _tailwind_output = std::process::Command::new("tailwindcss")
        .arg("-i")
        .arg("./public/styles/input.css")
        .arg("-o")
        .arg("./public/styles/output.css")
        .arg("-c")
        .arg("./tailwind.config.cjs")
        .output()
        .expect("Failed to run tailwindcss command");
}

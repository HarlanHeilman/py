// A rust based comand line wrapper for the python interpreter
use subprocess::Exec;

fn main() {
    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();
    // Get the local python install infor - use the venv if it exists
    let python = if cfg!(target_os = "windows") {
        if std::path::Path::new("venv/Scripts/python.exe").exists() {
            "venv/Scripts/python.exe"
        } else {
            "python"
        }
    } else {
        if std::path::Path::new("venv/bin/python").exists() {
            "venv/bin/python"
        } else {
            "python"
        }
    };
    // with the rest of the arguments
    let output = Exec::cmd(python).args(&args[1..]).capture().unwrap();
    println!("{}", output.stdout_str());
}

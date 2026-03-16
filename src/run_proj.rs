use std::process::Command;

fn run_python(python_exec_name: &str) {
    Command::new(python_exec_name)
        .arg("run.py")
        .spawn()
        .unwrap_or_else(|e| panic!("running project failed with error {e}"));
}

pub fn run_proj() {
    let exec_name = match std::env::consts::FAMILY
    {
        "windows" => "python",
        "unix" => "python3",
        _ => panic!("unsupported OS family"),
    };

    run_python(exec_name);
}

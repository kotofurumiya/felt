use std::env;
use std::process::{exit, Command};
use std::path::Path;

mod feltlib;

fn main() {
    // Load `.feltrc.toml` files recursively.
    let rc_list = feltlib::toml::load_feltrc();

    // Get felt-cli args
    let cli_args: Vec<String> = env::args().collect();

    // First arg is felt command name to execute.
    let cmd_name = match cli_args.get(1) {
        Some(a) => a,
        None => {
            print_usage();
            exit(0);
        }
    };

    // Rest args are used for command args.
    let cmd_args = cli_args.get(2..).unwrap_or(&[]).to_vec();

    // Should find executables in `node_modules/.bin/` or not.
    let uses_node_modules = feltlib::toml::uses_node_modules(&rc_list);

    // Get actual command from `.feltrc.toml` file.
    // `.feltrc.toml` in current directory is prior to home directory's one.
    let felt_cmd = feltlib::toml::get_command(&rc_list, cmd_name);

    // For node_modules
    let node_modules_bin = format!("./node_modules/.bin/{}", cmd_name);
    let node_bin_exists = Path::new(&node_modules_bin).exists();
    let should_use_node_bin = uses_node_modules && node_bin_exists;

    // Execute!
    match (felt_cmd, should_use_node_bin) {
        // If felt command is defined, execute it.
        (Some(cmd), _) => exec_command(&FeltCommand {
            command: cmd,
            args: &cmd_args,
            use_node_modules: false
        }),
        // If not defined but uses node_modules, execute it.
        (None, true) => exec_command(&FeltCommand {
            command: &node_modules_bin,
            args: &cmd_args,
            use_node_modules: true
        }),
        // If not defined and node_modules is disabled, it might be error.
        (None, false) => {
            eprintln!("[felt][error] command not found: \"{}\"", cmd_name);
            exit(1);
        }
    };
}

#[derive(Debug)]
struct FeltCommand<'a> {
    command: &'a str,
    args: &'a Vec<String>,
    use_node_modules: bool
}

#[cfg(target_family = "unix")]
fn exec_command(cmd: &FeltCommand) {
    let shell = feltlib::shell::unix::detect_login_shell();
    let joined_cmd = format!("{} {}", cmd.command, cmd.args.join(" "));

    match Command::new(shell).args(&["-c", &joined_cmd]).spawn() {
        Ok(mut c) => {
            c.wait()
                .expect(&format!("[felt][error] failed to execute '{}'", cmd.command));
            ()
        }
        Err(e) => println!("[felt][error] failed to execute '{}', {:}", cmd.command, e),
    };
}

#[cfg(not(target_family = "unix"))]
fn exec_command(cmd: &FeltCommand) {
    Command::new("echo")
        .arg("Windows is not supported on felt yet.")
        .spawn()
        .expect(&format!("[felt][error] failed to execute '{}'", cmd));
}

fn print_usage() {
    println!("Usage:");
    println!("   felt <your_command_name>");
}

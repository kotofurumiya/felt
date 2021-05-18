pub mod toml {
    use dirs::home_dir;
    use serde::Deserialize;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::vec::Vec;
    use toml;

    #[derive(Deserialize, Debug, Copy, Clone)]
    pub struct FeltRcFeltSection {
        pub root: Option<bool>,
        pub node_modules: Option<bool>,
    }

    impl FeltRcFeltSection {
        pub fn is_root(&self) -> bool {
            self.root.unwrap_or(false)
        }
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct FeltRcCommand {
        pub name: String,
        pub value: Option<String>,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct FeltRcCommandSection {
        pub commands: Vec<FeltRcCommand>,
        pub toml: Option<toml::value::Table>,
    }

    impl FeltRcCommandSection {
        pub fn get(&self, name: &str) -> Option<&FeltRcCommand> {
            self.commands.iter().find(|c| c.name == name)
        }
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct FeltRc {
        pub felt: FeltRcFeltSection,
        pub command: FeltRcCommandSection,
    }

    impl FeltRc {
        pub fn is_root(&self) -> bool {
            self.felt.is_root()
        }
    }

    #[derive(Deserialize, Debug, Clone)]
    struct FeltRcToml {
        felt: FeltRcFeltSection,
        command: Option<toml::value::Table>,
    }

    fn toml_to_feltrc(toml: &FeltRcToml) -> FeltRc {
        let command_toml = toml.command.clone();

        let commands = match command_toml.as_ref() {
            Some(m) => m
                .iter()
                .map(|(key, value)| FeltRcCommand {
                    name: key.to_string(),
                    value: value.as_str().map(|v| v.to_string()),
                })
                .collect(),
            None => vec![],
        };

        FeltRc {
            felt: toml.felt,
            command: FeltRcCommandSection {
                commands,
                toml: command_toml,
            },
        }
    }

    pub fn load_feltrc() -> Vec<FeltRc> {
        // These dirs could be valid anytime.
        // We don't need to check None.
        // Otherwise, panic.
        let cwd = env::current_dir().unwrap();
        let mut home_file_path = home_dir().unwrap();
        home_file_path.push(".feltrc.toml");

        // List of FeltRc.
        // Former item is loaded prior to latter items.
        let mut feltrc_list: Vec<FeltRc> = vec![];

        // This bool indicates if we had tried to load `~/.felt.rc` or not.
        let mut home_checked = false;

        // Going up to parent directries recursively.
        let mut current_dir = Some(cwd);
        while let Some(mut d) = current_dir.take() {
            let dir = d.to_path_buf();
            let mut file_path = PathBuf::from(dir);
            file_path.push(".feltrc.toml");

            if file_path == home_file_path {
                home_checked = true;
            }

            // Try to load `.feltrc.toml`.
            // If success, push FeltRc to vector.
            let feltrc_toml = load_toml(&file_path);
            if let Some(rc_toml) = feltrc_toml {
                let rc = toml_to_feltrc(&rc_toml);
                let is_root = rc.is_root();
                feltrc_list.push(rc);

                if is_root {
                    break;
                }
            }

            // Go to parent directory.
            if !d.pop() {
                break;
            };
        }

        if home_checked {
            return feltrc_list;
        }

        // Load `~/.feltrc.toml` finally
        // if we had not trying to load home file.
        if let Some(rc_toml) = load_toml(&home_file_path) {
            let rc = toml_to_feltrc(&rc_toml);
            feltrc_list.push(rc)
        }

        feltrc_list
    }

    pub fn get_all_commands(rc_list: &Vec<FeltRc>) -> Vec<FeltRcCommand> {
        let mut commands = vec![];

        for rc in rc_list {
            commands.extend(rc.command.commands.clone())
        }

        commands
    }

    pub fn get_command<'a>(rc_list: &'a Vec<FeltRc>, cmd_name: &str) -> Option<&'a FeltRcCommand> {
        for rc in rc_list {
            match rc.command.get(cmd_name) {
                None => continue,
                c => return c,
            }
        }

        None
    }

    fn load_toml(path: &PathBuf) -> Option<FeltRcToml> {
        let path_str = path.as_path().to_str().unwrap_or("");

        let file_str = match fs::read_to_string(path) {
            Err(_) => None,
            Ok(s) => Some(s),
        };

        if let Some(s) = file_str {
            return match toml::from_str(&s) {
                Ok(cfg) => Some(cfg),
                Err(e) => {
                    eprintln!("[felt][warning] ignoring invalid toml {}", path_str);
                    eprintln!("{}\n", e);
                    None
                }
            };
        }

        None
    }

    pub fn uses_node_modules(rc_list: &Vec<FeltRc>) -> bool {
        for rc in rc_list {
            match rc.felt.node_modules {
                Some(b) => return b,
                None => continue,
            };
        }

        false
    }
}

pub mod shell {
    use std::env;

    pub mod unix {
        use super::env;

        pub fn detect_login_shell() -> String {
            match env::var("SHELL") {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("[felt][warn] cannot detect login shell. Using \"sh\" instead.");
                    "sh".to_string()
                }
            }
        }
    }
}

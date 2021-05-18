pub mod toml {
    use dirs::home_dir;
    use serde::Deserialize;
    use std::env;
    use std::fs;
    use std::path::PathBuf;
    use std::vec::Vec;
    use toml;

    #[derive(Deserialize, Debug, Copy, Clone)]
    pub struct FeltRcFelt {
        pub root: Option<bool>,
        pub node_modules: Option<bool>,
    }

    impl FeltRcFelt {
        pub fn is_root(&self) -> bool {
            self.root.unwrap_or(false)
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct FeltRc {
        pub felt: Option<FeltRcFelt>,
        pub command: Option<toml::value::Table>,
    }

    impl FeltRc {
        pub fn is_root(&self) -> bool {
            match self.felt {
                Some(felt) => felt.is_root(),
                None => false
            }
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

            // Try to `.feltrc.toml`.
            // If it success, push FeltRc to vector.
            let feltrc = load_toml(&file_path);
            if let Some(rc) = feltrc {
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
        if let Some(toml) = load_toml(&home_file_path) {
            feltrc_list.push(toml)
        }

        feltrc_list
    }

    pub fn get_command<'a>(rc_list: &'a Vec<FeltRc>, cmd_name: &str) -> Option<&'a str> {
        for rc in rc_list {
            let cmd_sec = match &rc.command {
                None => continue,
                Some(sec) => sec,
            };

            let cmd = match cmd_sec.get(cmd_name) {
                None => continue,
                Some(cmd_str) => cmd_str.as_str(),
            };

            match cmd {
                None => continue,
                Some(cmd) => return Some(cmd),
            }
        }

        None
    }

    fn load_toml(path: &PathBuf) -> Option<FeltRc> {
        let path_str = path.as_path().to_str().unwrap_or("");

        let file_str = match fs::read_to_string(path) {
            Err(_) => None,
            Ok(s) => Some(s),
        };

        if let Some(s) = file_str {
            return match toml::from_str(&s) {
                Ok(cfg) => Some(cfg),
                Err(e) => {
                    eprintln!("Warning : ignoring invalid toml {}", path_str);
                    eprintln!("{}\n", e);
                    None
                },
            };
        }

        None
    }

    pub fn uses_node_modules(rc_list: &Vec<FeltRc>) -> bool {
        for rc in rc_list {
            let node_flag = match rc.felt {
                Some(felt) => felt.node_modules,
                None => continue,
            };

            match node_flag {
                Some(b) => return b,
                None => continue
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

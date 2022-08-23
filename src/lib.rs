pub mod process {
    use owo_colors::OwoColorize;
    use regex::Regex;
    use sedregex::ReplaceCommand;
    use std::fs::File;
    use std::io::prelude::*;

    #[derive(PartialEq)]
    struct Bind {
        name: String,
    }

    impl Bind {
        fn new(name: String) -> Self {
            Bind { name }
        }
    }

    pub struct Bindings {
        binds: Vec<Bind>,
    }

    impl Bindings {
        pub fn new() -> Self {
            Bindings { binds: Vec::new() }
        }

        pub fn join(&self, separator: &str) -> String {
            let mut result = String::new();
            for bind in &self.binds {
                // return each bind in a separated list, except for the last bind
                if bind != &self.binds[self.binds.len() - 1] {
                    result.push_str(&format!("{}{}", &bind.name, separator));
                } else {
                    result.push_str(&bind.name);
                }
            }
            result
        }
    }

    pub fn check_args(args: &[String]) -> Result<(), String> {
        if args.len() < 2 {
            return Err("Usage: svg2svelte <svg_file>".to_string());
        } else if args.len() > 3 {
            return Err(
                format!("{} Too many arguments", " ERROR ".bold().white().on_red()).to_string(),
            );
        }
        Ok(())
    }

    pub fn check_ext(file: &str) -> bool {
        file.ends_with(".svg")
    }

    pub fn get_filename(file: &str) -> String {
        let mut filename = String::new();
        let parent_dir = file.split('/').last().unwrap();
        let name = parent_dir.split('.').next().unwrap();
        let capitolized = name
            .chars()
            .nth(0)
            .unwrap()
            .to_uppercase()
            .collect::<String>()
            + &name[1..];
        filename.push_str(&capitolized);
        filename
    }

    pub fn read_file(file: &str) -> String {
        std::fs::read_to_string(file).expect(
            &format!(
                "{} Something went wrong reading the file...",
                " ERROR ".bold().white().on_red()
            )
            .to_string(),
        )
    }

    pub fn create_backup(file: &str) -> Result<(), String> {
        let backup_file = format!("{}.bak", file);
        std::fs::copy(file, backup_file).expect(
            &format!(
                "{} Something went wrong creating the backup file...",
                " ERROR ".bold().white().on_red()
            )
            .to_string(),
        );
        Ok(())
    }

    pub fn save_data(name: String, data: String) -> std::io::Result<()> {
        let mut file = File::create(name).unwrap();
        file.write_all(data.as_bytes()).unwrap();
        Ok(())
    }

    pub fn parse_binds(contents: &str) -> Bindings {
        let mut found = Bindings::new();
        for class in contents.lines() {
            if class.contains("class=\"bind:") {
                let mut bind = class.split("class=\"bind:").nth(1).unwrap();
                bind = bind.split("\"").nth(0).unwrap();
                found.binds.push(Bind::new(bind.to_string()));
            }
        }
        Bindings { binds: found.binds }
    }

    pub fn match_script_type(lang: String) -> String {
        let mut script_type = String::new();
        match lang.as_str() {
            "--ts" => {
                script_type.push_str("--ts");
            }
            "" => {
                script_type.push_str("");
            }
            _ => {
                println!(
                    "{} I think you meant to use \"--ts\"",
                    " ERROR ".bold().white().on_red(),
                );
                std::process::exit(1);
            }
        }
        script_type
    }

    pub fn make_script_tag(bindings: Bindings, lang: String) -> String {
        let mut script = String::new();
        match lang.as_ref() {
            "--ts" => {
                script.push_str(&format!(
                    "<script lang=\"ts\">\n\tlet {};\n</script>\n\n",
                    bindings.join(", ")
                ));
            }
            _ => {
                script.push_str(&format!(
                    "<script>\n\tlet {};\n</script>\n\n",
                    bindings.join(", ")
                ));
            }
        }
        script
    }

    pub fn ids_to_classes(contents: &str) -> String {
        let replaced = ReplaceCommand::new("s/id=\"bind:/class=\"bind:/g")
            .unwrap()
            .execute(contents);
        replaced.to_string()
    }

    pub fn run_svgo(file: &str) -> Result<(), String> {
        let mut svgo = std::process::Command::new("svgo");
        svgo.arg(&file.to_string());
        svgo.arg("-o");
        svgo.arg(file);
        svgo.arg("--pretty");
        svgo.output().expect(
            &format!(
                "{} Something went wrong running svgo...",
                " ERROR ".bold().white().on_red()
            )
            .to_string(),
        );
        Ok(())
    }

    pub fn make_svelte_binds(contents: String) -> String {
        let re = Regex::new(r#"class="bind:(.*?)""#).unwrap();
        let replaced = re.replace_all(&contents, |caps: &regex::Captures| {
            let id = caps.get(1).unwrap().as_str();
            format!(r#"class="{}" bind:this={{{}}}"#, id, id)
        });
        replaced.to_string()
    }

    pub fn make_component(name: String, contents: String) -> std::io::Result<()> {
        let mut file = File::create(&format!("{}.svelte", name)).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
        Ok(())
    }
}

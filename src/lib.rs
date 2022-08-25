pub mod svg_file {
    use regex::Regex;
    use sedregex::ReplaceCommand;
    use std::fs::File;
    use std::io::Write;

    fn check_ext(path: &str) -> bool {
        path.ends_with(".svg")
    }

    fn create_backup(file: &str) -> Result<(), String> {
        let backup_file = format!("{}.bak", file);
        std::fs::copy(file, &backup_file)
            .expect(&format!("Something went wrong creating {}", backup_file).to_string());
        Ok(())
    }

    fn read(file: &str) -> String {
        std::fs::read_to_string(file)
            .expect(&format!("Something went wrong reading {}...", file).to_string())
    }

    fn save_data(name: String, data: String) -> std::io::Result<()> {
        let mut file = File::create(&name).unwrap();
        file.write_all(data.as_bytes())
            .expect(&format!("Something went wrong saving {}", name).to_string());
        Ok(())
    }

    fn basename(file: &str) -> String {
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

    fn parse_binds(contents: &str) -> Vec<String> {
        let mut binds = Vec::new();
        for class in contents.lines() {
            if class.contains("class=\"bind:") {
                let mut bind = class.split("class=\"bind:").nth(1).unwrap();
                bind = bind.split("\"").nth(0).unwrap();
                binds.push(bind.to_string());
            }
        }
        binds
    }

    fn match_script_type(lang: bool) -> String {
        let mut script_type = String::new();
        match lang {
            true => {
                script_type.push_str("--ts");
            }
            false => {
                script_type.push_str("");
            }
        }
        script_type
    }

    fn make_script_tag(bindings: &mut Vec<String>, lang: String) -> String {
        let mut script = String::new();
        let typed_binds = add_types(bindings.to_vec());
        match lang.as_ref() {
            "--ts" => {
                script.push_str(&format!(
                    "<script lang=\"ts\">\n\tlet {};\n</script>\n\n",
                    typed_binds
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

    fn add_types(bindings: Vec<String>) -> String {
        let mut types = Vec::new();
        for binding in bindings {
            let mut type_name = String::new();
            type_name.push_str(&binding);
            type_name.push_str(": any");
            types.push(type_name);
        }
        types.join(", ")
    }


    fn ids_to_classes(contents: &str) -> String {
        let replaced = ReplaceCommand::new("s/id=\"bind:/class=\"bind:/g")
            .unwrap()
            .execute(contents);
        replaced.to_string()
    }

    fn run_svgo(file: &str) -> Result<(), String> {
        let mut svgo = std::process::Command::new("svgo");
        svgo.arg(&file.to_string());
        svgo.arg("-o");
        svgo.arg(file);
        svgo.arg("--pretty");
        svgo.output().expect("Something went wrong running svgo...");
        Ok(())
    }

    fn make_svelte_binds(contents: String) -> String {
        let re = Regex::new(r#"class="bind:(.*?)""#).unwrap();
        let replaced = re.replace_all(&contents, |caps: &regex::Captures| {
            let id = caps.get(1).unwrap().as_str();
            format!(r#"class="{}" bind:this={{{}}}"#, id, id)
        });
        replaced.to_string()
    }

    fn make_component(name: String, contents: String) -> std::io::Result<()> {
        let mut file = File::create(&format!("{}.svelte", name)).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
        Ok(())
    }

    pub fn process(file: &str, script_type: bool, verbose: bool) {
        let backup = format!("{}.bak", file);
        let script_type = match_script_type(script_type);
        let filename = basename(file);
        // we check the extension of the file first,
        // so we can determine if the file is an svg or not
        if check_ext(file) == false {
            println!("Please provide an SVG file");
            std::process::exit(1);
        }
        // here we create a backup of the file so we don't destroy the original
        create_backup(file).unwrap();
        let contents = read(&backup);
        // change all instances of id="bind: -> class="bind:
        let replaced_ids = ids_to_classes(&contents);
        // save the data to the backup
        save_data(backup.to_string(), replaced_ids.to_string()).unwrap();
        // optimize the svg file
        run_svgo(&backup.to_string()).unwrap();
        // read the optimized file
        let new_contents = read(&backup);
        // find all instances of class="bind:{}" to get the binds
        let mut parsed = parse_binds(&replaced_ids);
        // create the script tag for the svelte file
        let script_tag = make_script_tag(&mut parsed, script_type);
        // create the svelte bindings we'll use to animate later
        let svelte_binds = make_svelte_binds(new_contents);
        // format the data for the svelte file
        let data = format!("{}{}", script_tag, svelte_binds);
        // now that we have all the data rounded up, we create the svelte component
        make_component(filename, data).unwrap();
        // remove the backup file
        std::fs::remove_file(backup).unwrap();
        // if verbose is true, we print the generated component to stdout
        if verbose {
            println!("{}", script_tag);
            println!("{}", svelte_binds);
        }
    }
}

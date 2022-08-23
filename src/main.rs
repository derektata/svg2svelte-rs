use owo_colors::OwoColorize;
use std::{env, fs, process};
use svg2svelte::process::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    match check_args(&args) {
        Ok(()) => {
            let file = &args[1];
            let backup = format!("{}.bak", file);

            // this grabs the basename of the file and capitolizes the first letter
            let filename = get_filename(file);

            // here we instaniate a new string for the script type,
            // if it's empty, the default is javascript
            // otherwise "--ts" will make it typescript
            let mut script_type = String::new();

            // we check the extension of the file first,
            // so we can determine if the file is an svg or not
            if check_ext(&file) == false {
                println!(
                    "{} Please provide an SVG file",
                    " ERROR ".bold().white().on_red(),
                );
                process::exit(1);
            }

            // this is a hacky way to get the script type from the command line
            // by checking if there's a 2nd arg
            if args.len() == 3 {
                let script_arg = &args[2];
                let matched_type = match_script_type(script_arg.to_string());
                script_type.push_str(&matched_type);
            }

            // here we create a backup of the file so we don't destroy the original
            create_backup(file).unwrap();

            let contents = read_file(&backup);

            // change all instances of id="bind: -> class="bind:
            let replaced_ids = ids_to_classes(&contents);

            // save the data to the backup
            save_data(backup.to_string(), replaced_ids.to_string())
                .expect("Something went wrong saving the backup file");

            // optimize the svg file
            run_svgo(&backup.to_string()).expect("Something went wrong running svgo");

            // read the optimized file
            let new_contents = read_file(&backup);

            // find all instances of class="bind:{}" to get the binds
            let parsed = parse_binds(&replaced_ids);

            // create the script tag for the svelte file
            let script_tag = make_script_tag(parsed, script_type);

            // create the svelte bindings we'll use to animate later
            let svelte_binds = make_svelte_binds(new_contents);

            // format the data for the svelte file
            let data = format!("{}{}", script_tag, svelte_binds);

            // now that we have all the data rounded up, we create the svelte component
            make_component(filename, data)
                .expect("Something went wrong creating the svelte component");

            // remove the backup file
            fs::remove_file(backup).expect("Something went wrong removing the backup file");

            // print the results to the console so we can see what was done
            println!("{}", script_tag);
            println!("{}", svelte_binds);
        }

        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

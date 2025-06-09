use std::{
    fs,
    io::{self},
    path::{Path, PathBuf},
    process::Command,
};

use crate::bindings::{bind_svg_variables, extract_bind_variables, replace_bind_ids};

pub struct Processor {
    svg: PathBuf,
    typescript: bool,
    verbose: bool,
}

impl Processor {
    /// Create a new Processor for `svg`
    pub fn new<P: Into<PathBuf>>(svg: P) -> Self {
        Self {
            svg: svg.into(),
            typescript: false,
            verbose: false,
        }
    }

    /// Emit a `<script lang="ts">`
    pub fn typescript(mut self, ts: bool) -> Self {
        self.typescript = ts;
        self
    }

    /// Print the component to stdout as well
    pub fn verbose(mut self, v: bool) -> Self {
        self.verbose = v;
        self
    }

    /// Perform the conversion
    pub fn run(self) -> io::Result<()> {
        self.validate_extension()?;

        let backup_path = self.create_backup()?;

        self.replace_ids(&backup_path)?;
        Self::optimize(&backup_path)?;

        let optimized = fs::read_to_string(&backup_path)?;
        let binds = extract_bind_variables(&optimized);

        let script = Self::build_script(&binds, self.typescript);
        let body = bind_svg_variables(&optimized);

        self.write_component(script, body)?;
        self.cleanup_backup(backup_path)?;
        Ok(())
    }

    fn validate_extension(&self) -> io::Result<()> {
        if self.svg.extension().and_then(|e| e.to_str()) != Some("svg") {
            eprintln!("Please provide an SVG file");
            std::process::exit(1);
        }
        Ok(())
    }

    fn create_backup(&self) -> io::Result<PathBuf> {
        let backup = self.svg.with_extension("svg.bak");
        fs::copy(&self.svg, &backup)?;
        Ok(backup)
    }

    fn replace_ids(&self, path: &Path) -> io::Result<()> {
        let contents = fs::read_to_string(path)?;
        let replaced = replace_bind_ids(&contents);
        fs::write(path, replaced)
    }

    fn optimize(path: &Path) -> io::Result<()> {
        Command::new("svgo")
            .arg(path)
            .arg("-o")
            .arg(path)
            .arg("--pretty")
            .output()?;
        Ok(())
    }

    fn build_script(binds: &[String], ts: bool) -> String {
        let vars = binds
            .iter()
            .map(|b| if ts { format!("{}: any", b) } else { b.clone() })
            .collect::<Vec<_>>()
            .join(", ");

        let lang = if ts { r#" lang="ts""# } else { "" };
        format!("<script{}>\n    let {};\n</script>\n\n", lang, vars)
    }

    fn write_component(&self, script: String, body: String) -> io::Result<()> {
        let mut output = String::new();
        output.push_str(&script);
        output.push_str(&body);
        let out_path = self.svg.with_extension("svelte");
        fs::write(&out_path, output)?;
        if self.verbose {
            println!("{}", script);
            println!("{}", body);
        }
        Ok(())
    }

    fn cleanup_backup(&self, backup: PathBuf) -> io::Result<()> {
        fs::remove_file(backup)
    }
}

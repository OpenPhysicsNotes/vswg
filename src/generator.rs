use std::path::PathBuf;

use crate::path::PathVec;

/// A rule for the generator
pub trait Rule: 'static {
    /// Run the rule on the given file
    ///
    /// # Arguments
    /// * `root` - The root directory of the content
    /// * `path` - The path to the file
    /// * `rel` - The relative path to the file
    /// * `out_root` - The root directory of the output
    ///
    /// # Returns
    /// `true` if the rule was applied, `false` otherwise
    fn run(&self, root: &PathBuf, path: &PathBuf, rel: &PathVec, out_root: &PathBuf) -> bool;
}

/// A generator for static websites
///
/// # Example
/// ```ignore
/// # use vswg::Generator;
/// Generator::new()
///     .run("./examples/content", "./examples/out");
/// ```
pub struct Generator {
    rules: Vec<Box<dyn Rule>>,
}

impl Generator {
    /// Create a new generator
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }

    /// Add a rule to the generator
    pub fn rule(self, rule: impl Rule) -> Self {
        let mut new = self;
        new.rules.push(Box::new(rule));
        new
    }

    /// Run the generator on the given directory
    pub fn run(&self, dir: &str, out_dir: &str) {
        // TODO a better check
        if out_dir.starts_with(dir) || dir.starts_with(out_dir) {
            panic!("Output directory cannot be a subdirectory of the input directory or vice versa");
        }

        self.handle_dir(
            &PathBuf::from(dir),
            &PathBuf::from(dir),
            &PathVec::new(),
            &PathBuf::from(out_dir)
        );
    }

    fn handle_dir(&self, root: &PathBuf, dir: &PathBuf, rel: &PathVec, out_root: &PathBuf) {
        assert_eq!(dir.is_dir(), true);

        let relative = dir.strip_prefix(root).expect("Error stripping prefix");
        std::fs::create_dir_all(out_root.join(relative)).expect("Error creating directory");

        for entry in dir.read_dir().expect("read_dir call failed") {
            let entry = entry.expect("Error reading entry");
            let path = entry.path();
            let name = path.file_name().unwrap().to_str().unwrap();

            let rel = rel / name;

            if path.is_dir() {
                self.handle_dir(root, &path, &rel, out_root);
            } else {
                self.handle_file(root, &path, &rel, out_root);
            }
        }
    }

    fn handle_file(&self, root: &PathBuf, path: &PathBuf, rel: &PathVec, out_root: &PathBuf) {
        assert_eq!(path.is_file(), true);
        let relative = path.strip_prefix(root).expect("Error stripping prefix");

        for rule in self.rules.iter() {
            if rule.run(root, path, rel, out_root) {
                return;
            }
        }

        let out_path = out_root.join(relative);

        std::fs::copy(path, out_path).expect("Error copying file");
    }
}
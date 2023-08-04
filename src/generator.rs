use std::path::PathBuf;

use crate::path::PathVec;

pub trait Rule: 'static {
    fn run(&self, root: &PathBuf, path: &PathBuf, rel: &PathVec, out_root: &PathBuf) -> bool;
}

pub struct Generator {
    rules: Vec<Box<dyn Rule>>,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }

    pub fn rule(self, rule: impl Rule) -> Self {
        let mut new = self;
        new.rules.push(Box::new(rule));
        new
    }

    pub fn run(&self, dir: &PathBuf, out_dir: &PathBuf) {
        self.handle_dir(dir, dir, &PathVec::new(), out_dir);
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
        //println!("File: {}", path.display());
        let relative = path.strip_prefix(root).expect("Error stripping prefix");
        //println!("Relative: {}", relative.display());

        for rule in self.rules.iter() {
            if rule.run(root, path, rel, out_root) {
                return;
            }
        }

        let out_path = out_root.join(relative);

        // copy file form path to out_path:
        //println!("Copying file from {} to {}", path.display(), out_path.display());
        std::fs::copy(path, out_path).expect("Error copying file");
    }
}
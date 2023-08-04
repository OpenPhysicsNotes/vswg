use std::{path::PathBuf, fs::File, io::Write};

use vswg::{generator::{Generator, Rule}, path::PathVec};



fn main() {

    let content_dir = PathBuf::from("./content");
    let out_dir = PathBuf::from("./out");

    let generator = Generator::new()
        .rule(HtmlPage);

    generator.run(&content_dir, &out_dir);
}


struct HtmlPage;
impl Rule for HtmlPage {
    fn run(&self, root: &PathBuf, path: &PathBuf, _rel: &PathVec, out_root: &PathBuf) -> bool {
        println!("alive");
        if path.extension().unwrap_or_default() != "html" {
            return false;
        }

        let body = std::fs::read_to_string(path).expect("Error reading file");

        use write_html::*;

        let page = html!(
            (Doctype)
            html lang="en" {
                head {
                    (DefaultMeta)
                    title { "My Blog" }
                }
                body {
                    (body.as_html())
                }
            }
        ).to_html_string().unwrap();

        let out_file = out_root.join(path.strip_prefix(root).unwrap());
        let mut out_file = File::create(out_file).expect("Error creating file");
        out_file.write_all(page.as_bytes()).expect("Error writing file");

        true
    }
}

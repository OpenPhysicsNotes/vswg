use std::{path::PathBuf, fs::File, io::Write};

use vswg::{generator::{Generator, Rule}, path::PathVec};

fn main() {
    Generator::new()
        .rule(HtmlPage)
        .run("./content", "./out");
}


struct HtmlPage;
impl Rule for HtmlPage {
    fn run(&self, root: &PathBuf, path: &PathBuf, _rel: &PathVec, out_root: &PathBuf) -> bool {
        // handle HTML files only
        if path.extension().unwrap_or_default() != "html" {
            return false;
        }

        // Read the body from the file
        let body = std::fs::read_to_string(path).expect("Error reading file");

        use write_html::*;

        // wrap the body in a page
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

        // write to the output file
        let out_file = out_root.join(path.strip_prefix(root).unwrap());
        let mut out_file = File::create(out_file).expect("Error creating file");
        out_file.write_all(page.as_bytes()).expect("Error writing file");

        true
    }
}

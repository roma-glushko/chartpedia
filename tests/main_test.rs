include!("../src/main.rs");

#[cfg(test)]
mod tests {
    use std::fs;
    use super::cli::{Cli};
    use clap::{CommandFactory};
    use tempdir::TempDir;

    #[test]
    fn render_middle_param_section() {
        let tmp_dir = TempDir::new("chartpedia").unwrap();
        let markdown_file = tmp_dir.path().join("readme.md");

        fs::copy("./tests/assets/readme.middle.empty.md", markdown_file.clone()).unwrap();

        let matcher = Cli::command()
            .try_get_matches_from(vec![
                "chartpedia",
                "gen",
                "-v=./tests/assets/values.yaml",
                &format!("-m={}", markdown_file.to_string_lossy()),
            ]).unwrap();
    }

    #[test]
    fn render_last_param_section() {

    }

    #[test]
    fn render_last_param_section_with_text_below() {

    }

}

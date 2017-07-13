use std::fs;
use colored::*;
use clap::ArgMatches;
use helpers::template::Template;

pub fn main(matches: &ArgMatches) {
    let arguments = matches.subcommand_matches("delete").unwrap();
    let language = arguments.value_of("language").unwrap().to_string();
    let name = arguments.value_of("name").unwrap().to_string();
    let template = Template { language, name };

    delete_template(&template);
}

fn delete_template(template: &Template) {
    if template.exists() {
        fs::remove_dir_all(template.filepath()).unwrap();
        print_success_message(template);
    } else {
        println!(
            "Template {} for {} not found.",
            template.name.yellow().italic(),
            template.language.yellow().italic()
        );
    }
}

fn print_success_message(template: &Template) {
    println!(
        "Successfully deleted {} for {}.",
        &template.name.italic().green(),
        &template.language.italic().green()
    );
}

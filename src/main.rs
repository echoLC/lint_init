extern crate execute;
use std::process::Command;
use execute::Execute;

use ansi_term::Colour::{Red, Green};
use clap::{Parser};
use std::fs;
use std::env::{current_dir};
use std::path::{PathBuf};
use relative_path::RelativePath;
use lint_init::template::{*};

#[derive(Parser, Debug)]
#[clap(name="lint-init", author="echoLC", version = "0.1.0", about = "Init lint config for a project.", long_about=None)]
struct Cli {
    #[clap(short, long)]
    template: String,

    #[clap(short, long, default_value = ".")]
    dir: String,

    #[clap(short, long)]
    auto_install: bool,
}

const ESLINT_FILE_NAME: &str = "/.eslintrc.json";

struct TemplateInfo {
    template_content: String,
    target_url: String
}

fn main() {
    let args = Cli::parse();
    let dir = args.dir;

    let file_info: TemplateInfo = get_template_content(args.template);

    let template_content = file_info.template_content;
    let target_url = file_info.target_url;
    
    println!("template content is: {}", Green.paint(&template_content));

    let target_dir = PathBuf::from(&dir);
    let target_dir = fs::canonicalize(&target_dir);

    match &target_dir {
        Ok(path) => {
            print!("write file content");

            write_content(&template_content, get_str_from_pathbuf(path.to_path_buf()) + &target_url);

            if args.auto_install {
                execute_install();
            }
        },
        Err(_err) => {
            let target_path =  get_target_dir(&dir);

            fs::create_dir(&target_path).expect("Unable to create dir");

            write_content(&template_content, get_str_from_pathbuf(target_path) + &target_url);

            if args.auto_install {
                execute_install();
            }
        }
    };
}

fn get_template_content (template: String) -> TemplateInfo {
    match template.as_str() {
        "typescript" => TemplateInfo{template_content: String::from(typescript::TEMPLATE_CONTENT), target_url: String::from(ESLINT_FILE_NAME) },
        "reactTs" => TemplateInfo{template_content: String::from(react::TEMPLATE_CONTENT), target_url: String::from(ESLINT_FILE_NAME) },
        "prettier" => TemplateInfo{template_content: String::from(prettier::TEMPLATE_CONTENT), target_url: String::from("/.prettierrc.js") },
        "pureJs" => TemplateInfo{template_content: String::from(pure_js::TEMPLATE_CONTENT), target_url: String::from(ESLINT_FILE_NAME) },
        _ => panic!("unknown template type: {}", Red.paint(template))
    }
}

fn write_content(content: &str, target_path: String) {
    let res = fs::write(&target_path,content);
            
    match res {
        Ok(_) => println!("write {} config successfully", &target_path),
        Err(err) => panic!("write file failed: {:?}", err)
    }
}

fn get_target_dir (dir: &str) -> PathBuf {
    let current_dir_s = get_str_from_pathbuf(current_dir().unwrap());
    let target_dir_path = current_dir_s + "/" + dir;
    let target_dir_path = RelativePath::new(&target_dir_path).normalize();

    println!("target dir is : {:?}", &target_dir_path);
    
    target_dir_path.to_path("/")
}

fn get_str_from_pathbuf (path: PathBuf) -> String {
    match path.to_str() {
        Some(path_str) => path_str.to_string(),
        None => panic!("Unable get string from pathbuf")
    }
}

fn execute_install () {
    let mut install_command = Command::new("yarn");
    
    install_command.arg("add");
    install_command.arg("eslint");
    install_command.arg("@sl/eslint-plugin-fe");
    install_command.arg("--dev");
    install_command.arg("--registry=https://npm-registry.duowan.com");
         
    let res = install_command.execute();

    match res {
        Err(err) => {
            println!("execute install command error: {}", err.to_string());
        },
        _ => {
            println!("install dep successfully");
        }
    }    
}

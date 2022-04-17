extern crate execute;
use std::process::Command;
use execute::Execute;

use ansi_term::Colour::{Red, Green};
use clap::{Parser};
use std::{fs};
use std::env::{current_dir};
use std::path::{PathBuf};
use relative_path::RelativePath;
use dialoguer::{
    MultiSelect,
    theme::ColorfulTheme
};
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
const TEMPLATE_LIST: [&str;5] = ["default", "typescript", "prettier", "pure_js", "react"];

struct TemplateInfo {
    template_content: String,
    target_url: String
}

fn main() {
    
    let template_list = prompt_select_template();
    let template_list = normalize_template_list(template_list);

    println!("current select templates: {:?}", template_list);

    let args = Cli::parse();
    let dir = args.dir;


    for i in template_list {
        let template = TEMPLATE_LIST[i];
        let file_info = get_template_content(template.to_string());

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

fn prompt_select_template () -> Vec<usize> {
    let defaults = vec![true, false, false, false, false];
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(&TEMPLATE_LIST)
        .defaults(&defaults)
        .interact();

    match selection {
        Ok(select_templates) => select_templates,
        Err(err) => {
            println!("select typescript error: {}", err.to_string());
            [0].to_vec()
        }
    }
}

fn normalize_template_list (template_list: Vec<usize>) -> Vec<usize> {
    let result;
    if template_list.len() == 1 && template_list[0] == 0 {
        result = vec![1, 2];
    } else if template_list.contains(&0) {
        result = template_list.into_iter().filter(|index| index != &0).collect();
    } else {
        result = template_list.clone();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_typescript_template () {
        let template_info = get_template_content(String::from("typescript"));
        assert_eq!(template_info.template_content, String::from(typescript::TEMPLATE_CONTENT));
        assert_eq!(template_info.target_url, String::from(ESLINT_FILE_NAME));
    }

    #[test]
    fn get_react_template () {
        let template_info = get_template_content(String::from("reactTs"));
        assert_eq!(template_info.template_content, String::from(react::TEMPLATE_CONTENT));
        assert_eq!(template_info.target_url, String::from(ESLINT_FILE_NAME));
    }

    #[test]
    fn get_pure_js_template () {
        let template_info = get_template_content(String::from("pureJs"));
        assert_eq!(template_info.template_content, String::from(pure_js::TEMPLATE_CONTENT));
        assert_eq!(template_info.target_url, String::from(ESLINT_FILE_NAME));
    }

    #[test]
    fn get_prettier_template () {
        let template_info = get_template_content(String::from("prettier"));
        assert_eq!(template_info.template_content, String::from(prettier::TEMPLATE_CONTENT));
        assert_eq!(template_info.target_url, String::from("/.prettierrc.js"));
    }

    #[test]
    fn get_target_dir_from_current_dir () {
        let current_dir = get_target_dir(".");
        assert_eq!(&current_dir.to_string_lossy()[1..], String::from("/Users/luochao/echoLC-github/lint_init"));
    }

    #[test]
    fn get_target_dir_from_relative_dir () {
        let current_dir = get_target_dir("../react-app");
        assert_eq!(&current_dir.to_string_lossy()[1..], String::from("/Users/luochao/echoLC-github/react-app"));
    }

    #[test]
    fn convert_pathbuf_to_string () {
        assert_eq!(get_str_from_pathbuf(PathBuf::from("/react-app")), String::from("/react-app"));
    }

    #[test]
    fn normalize_default_template_list () {
        assert_eq!(normalize_template_list(Vec::from([0])), Vec::from([1, 2]));
    }

    #[test]
    fn normalize_normal_template_list () {
        assert_eq!(normalize_template_list(Vec::from([1, 2])), Vec::from([1, 2]));
    }

    #[test]
    fn normalize_normal_and_default_template_list () {
        assert_eq!(normalize_template_list(Vec::from([0, 1, 2])), Vec::from([1, 2]));
    }
}

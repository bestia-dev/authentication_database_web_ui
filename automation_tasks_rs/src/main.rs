//! automation_tasks_rs for authentication_database_web_ui workspace
//! The workspace contains 3 members: tier1_browser_wasm, tier2_web_server_actix_postgres, tier3_database_postgres

use cargo_auto_lib::*;

// the server executable binary is called "webpage_hits_admin"
// and it is the main url route
const APP_MAIN_ROUTE: &'static str = "webpage_hits_admin";

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";


fn main() {
    exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => {
            build_cargo_auto_for_members();
            print_help();
        }
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("{YELLOW}Running automation task: {task}{RESET}");
                if &task == "build" {
                    task_build();
                } else if &task == "release" {
                    task_release();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "test" {
                    task_test();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "publish_to_web" {
                    task_publish_to_web();
                } else {
                    println!("{RED}Error: Task {task} is unknown.{RESET}");
                    print_help();
                }
            }
        }
    }
}

/// build all automation_tasks_rs for every member and 
/// delete the unnecessary target files to save disk space
fn build_cargo_auto_for_members(){
    let workspace_dir = std::env::current_dir().unwrap();
         
     println!("Processing member: common_code");    
     std::env::set_current_dir("common_code").unwrap();
     run_shell_command("cargo auto;");      
     std::env::set_current_dir(&workspace_dir).unwrap();  
 
     println!("Processing member tier2_library_for_web_app");    
     std::env::set_current_dir("tier2_library_for_web_app").unwrap();    
     run_shell_command("cargo auto;"); 
     std::env::set_current_dir(&workspace_dir).unwrap();  
     
     println!("Processing member tier2_web_server_actix_postgres");    
     std::env::set_current_dir("tier2_web_server_actix_postgres").unwrap();    
     run_shell_command("cargo auto;"); 
     std::env::set_current_dir(&workspace_dir).unwrap();  
 
     println!("Processing member tier1_browser_wasm");    
     std::env::set_current_dir("tier1_browser_wasm").unwrap();    
     run_shell_command("cargo auto;"); 
     std::env::set_current_dir(&workspace_dir).unwrap();  

     run_shell_command("rm -rf ./common_code/automation_tasks_rs/target/debug/*/");
     run_shell_command("rm -rf ./common_code/automation_tasks_rs/target/debug/.fingerprint/");
     run_shell_command("rm -rf ./tier2_library_for_web_app/automation_tasks_rs/target/debug/*/");
     run_shell_command("rm -rf ./tier2_library_for_web_app/automation_tasks_rs/target/debug/.fingerprint/");
     run_shell_command("rm -rf ./tier2_web_server_actix_postgres/automation_tasks_rs/target/debug/*/");
     run_shell_command("rm -rf ./tier2_web_server_actix_postgres/automation_tasks_rs/target/debug/.fingerprint/");
     run_shell_command("rm -rf ./tier1_browser_wasm/automation_tasks_rs/target/debug/*/");
     run_shell_command("rm -rf ./tier1_browser_wasm/automation_tasks_rs/target/debug/.fingerprint/");
     run_shell_command("rm -rf ./automation_tasks_rs/target/debug/*/");
     run_shell_command("rm -rf ./automation_tasks_rs/target/debug/.fingerprint/");
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to cargo-auto !
    This program automates your custom tasks when developing a Rust project.{RESET}

    User defined tasks in automation_tasks_rs:{RESET}{GREEN}
cargo auto build{RESET}{YELLOW} - builds the crate in debug mode, fmt, increment version{RESET}{GREEN}
cargo auto release{RESET}{YELLOW} - builds the crate in release mode, fmt, increment version{RESET}{GREEN}
cargo auto doc{RESET}{YELLOW} - builds the docs, copy to docs directory{RESET}{GREEN}
cargo auto test{RESET}{YELLOW} - runs all the tests{RESET}{GREEN}
cargo auto commit_and_push "message"{RESET}{YELLOW} - commits with message and push with mandatory message
    (If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.){RESET}{GREEN}
cargo auto publish_to_web{RESET}{YELLOW} - publish to my google VM, git tag
    (You need credentials for publishing. I use ssh-agent and ssh-add to store my credentials for SSH.){RESET}
    © 2022 bestia.dev  MIT License github.com/bestia-dev/cargo-auto
"#
    );
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "release", "doc", "test", "commit_and_push","publish_to_web"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// Build every member of pseudo-workspace. 
/// I am not using the original cargo workspace functionality. The automation task will take care of all members.
/// One is wasm project, so instead of cargo build, I use wam-pack build.
fn task_build() {
    let workspace_dir = std::env::current_dir().unwrap();
            
    // build every member separately. 
    println!("Processing member common_code");    
    std::env::set_current_dir("common_code").unwrap();
    run_shell_command("cargo auto build;"); 
    std::env::set_current_dir(&workspace_dir).unwrap();  

    println!("Processing member tier2_library_for_web_app");    
    std::env::set_current_dir("tier2_library_for_web_app").unwrap();    
    run_shell_command("cargo auto build;"); 
    std::env::set_current_dir(&workspace_dir).unwrap();  

    // when I use --exclude tier1_browser_wasm, cargo rebuilds a bunch of dependencies !?!
    // to debug why cargo rebuilds I used: CARGO_LOG=cargo::core::compiler::fingerprint=info cargo build
    println!("Processing member tier2_web_server_actix_postgres");    
    std::env::set_current_dir("tier2_web_server_actix_postgres").unwrap();    
    run_shell_command("cargo auto build;"); 
    std::env::set_current_dir(&workspace_dir).unwrap();  

    // wasm-pack changes something in the folder target, so the next build unnecessarily rebuilds dependencies
    // I will try to use --release to force wasm-pack to use a different folder
    println!("Processing member tier1_browser_wasm");    
    std::env::set_current_dir("tier1_browser_wasm").unwrap();    
    run_shell_command("cargo auto build;"); 
    std::env::set_current_dir(&workspace_dir).unwrap();  
    
    run_shell_command(&format!("rsync -a --info=progress2 --delete-after tier1_browser_wasm/pkg/ web_server_folder/{APP_MAIN_ROUTE}/pkg/"));
            
    println!(
        r#"{YELLOW}
    After `cargo auto build`, run the compiled binary, examples and/or tests
cd web_server_folder ; ../tier2_web_server_actix_postgres/target/debug/{APP_MAIN_ROUTE} ; cd ..
    In the browser or in curl open 
http://localhost:8080/{APP_MAIN_ROUTE}/c1_webpage_hits_mod/c1_webpage_hits_list
    if ok, then
cargo auto release
{RESET}"#
    );
}
/// build release every member of workspace. One is wasm project, so instead of cargo build, I use wam-pack build
/// this workspace is basically one single application splitted into 3 projects
/// it deserves the same version number for the release build. It means that it will build all members. 
/// A little slower than only build.
fn task_release() {
    // let cargo_toml = CargoToml::read();
    //auto_check_micro_xml(&format!("web_server_folder/{APP_MAIN_ROUTE}"));
    auto_version_increment_semver_or_date_forced();    
    run_shell_command("cargo fmt");

    //run_shell_command("cd tier1_browser_wasm;wasm-pack build --target web --release;cd ..");
    // copy to web_server_folder/pkg
    //run_shell_command(&format!("rsync -a --info=progress2 --delete-after tier1_browser_wasm/pkg/ web_server_folder/{APP_MAIN_ROUTE}/pkg/"));

    auto_cargo_toml_to_md();

    auto_lines_of_code("");

    // when I use --exclude tier1_browser_wasm, cargo rebuild a bunch of dependencies !?!
    run_shell_command("cargo build --release --workspace ");    
    run_shell_command(&format!("strip target/release/{APP_MAIN_ROUTE}"));
    run_shell_command("cd tier1_browser_wasm; wasm-pack build --target web --release ; cd ..");
    run_shell_command(&format!("rsync -a --info=progress2 --delete-after tier1_browser_wasm/pkg/ web_server_folder/{APP_MAIN_ROUTE}/pkg/"));

    println!(
        r#"{YELLOW}
    After `cargo auto release`, run the compiled binary, examples and/or tests
cd web_server_folder ; ../target/release/{APP_MAIN_ROUTE}; cd ..
    In the browser or in curl open 
http://localhost:8080/{APP_MAIN_ROUTE}/c1_webpage_hits_mod/c1_webpage_hits_list
    if ok, then
cargo auto doc
{RESET}"#,
    );
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    let cargo_toml = CargoToml::read();
    auto_md_to_doc_comments();
    auto_plantuml(&cargo_toml.package_repository().unwrap());

    #[rustfmt::skip]
    let shell_commands = [
        "cargo doc --no-deps --document-private-items",
        // copy target/doc into docs/ because it is github standard
        "rsync -a --info=progress2 --delete-after target/doc/ docs/",
        "echo Create simple index.html file in docs directory",
        &format!("echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",cargo_toml.package_name().replace("-","_")) ,
    ];
    run_shell_commands(shell_commands.to_vec());
    run_shell_command("cargo fmt");
    // message to help user with next task
    println!(
        r#"{YELLOW}
    After `cargo auto doc`, check `docs/index.html`. If ok, then test the documentation code examples
cargo auto test
{RESET}"#
    );
}

/// cargo test
fn task_test() {
    run_shell_command("cargo test");
    println!(
        r#"{YELLOW}
    After `cargo auto test`. If ok, then commit with mandatory commit message{RESET}{GREEN}
cargo auto commit_and_push "message"{RESET}{YELLOW}
{RESET}"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    match arg_2 {
        None => println!("{RED}Error: Message for commit is mandatory.{RESET}"),
        Some(message) => {
            // I want separate commit for the /docs/ folder, so the commit of code is not cluttered
            if std::path::Path::new("docs").exists(){
                run_shell_command(&format!(r#"git add docs && git commit --allow-empty -m "docs" "#));    
            }
            run_shell_command(&format!(r#"git add -A && git commit --allow-empty -m "{}""#, message));
            run_shell_command("git push");
            println!(
                r#"{YELLOW}
    After `cargo auto commit_and_push "message"`
cargo auto publish_to_web
{RESET}"#
            );
        }
    }
}

/// publish to web for podman container and git tag
fn task_publish_to_web() {
    println!(r#"{YELLOW}Use ssh-agent and ssh-add to store the credentials.{RESET}"#);
    let cargo_toml = CargoToml::read();
    // git tag
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = cargo_toml.package_version()
    );
    run_shell_command(&shell_command);

    // rsync files
    //run_shell_command("rsync -e ssh -a --info=progress2 ./target/release/{APP_MAIN_ROUTE} luciano_bestia@bestia.dev:/var/www/transfer_folder/{APP_MAIN_ROUTE}/");
    //run_shell_command("rsync -e ssh -a --info=progress2 ./.env luciano_bestia@bestia.dev:/var/www/transfer_folder/{APP_MAIN_ROUTE}/");
    //run_shell_command("rsync -e ssh -a --info=progress2 ./deploy/buildah_image_{APP_MAIN_ROUTE}.sh luciano_bestia@bestia.dev:/var/www/transfer_folder/{APP_MAIN_ROUTE}/");
    //run_shell_command("rsync -e ssh -a --info=progress2 ./deploy/{APP_MAIN_ROUTE}_pod_create.sh luciano_bestia@bestia.dev:/var/www/transfer_folder/{APP_MAIN_ROUTE}/");

    println!(
        r#"{YELLOW}
    After `cargo auto publish_to_web`, 
    connect to the google VM bash using SSH.
ssh -i ~/.ssh/ssh_certificate username@domain -v
    There run the bash scripts to create the image and to create the pod.
cd /var/www/transfer_folder/{APP_MAIN_ROUTE}
sh buildah_image_{APP_MAIN_ROUTE}.sh
sh {APP_MAIN_ROUTE}_pod_create.sh
    Test the postgres server:
psql -h localhost -p 5432 -U admin -W
    Test the web application locally:
curl http://localhost:8011/{APP_MAIN_ROUTE}/get_svg_image/555555.svg
    Test the web application on the internet:
curl https://bestia.dev/{APP_MAIN_ROUTE}/get_svg_image/555555.svg
{RESET}"#
    );
}
// endregion: tasks

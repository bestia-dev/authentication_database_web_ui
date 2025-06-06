// automation_tasks_rs for authentication_database_web_ui workspace

// This is not a standard cargo workspace. It does not have Cargo.toml in the workspace folder.
// Because of one WASM project I cannot use the profile panic="abort" on all the members.
// There is no workaround as of 2022-10-20.
// We can use cargo-auto to automate the tasks for every member individually.
// Then we code the automation_tasks_rs on the workspace level to call all members tasks together.
// The presence of Cargo.toml is used by cargo-auto to recognize project folders.
// The workspace contains 4 members: tier0_common_code, tier1_browser_wasm, tier2_library_for_web_app, tier2_webpage_hits_admin
// Every member has its own automation_tasks_rs. This workspace automation just calls the automation of the members.
// I am not using the original cargo workspace functionality. The automation task will take care of all members.

// region: library with basic automation tasks
use cargo_auto_lib as cl;
// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;

use cargo_auto_lib::GREEN;
use cargo_auto_lib::RED;
use cargo_auto_lib::RESET;
use cargo_auto_lib::YELLOW;

// region: library with basic automation tasks

// the server executable binary is called "webpage_hits_admin"
// and it is also the main url route
const APP_MAIN_ROUTE: &'static str = "webpage_hits_admin";

const MEMBERS:&'static [&'static str] =&[
    "tier0_common_code", 
    "tier1_browser_wasm",
    "tier2_library_for_web_app",
    "tier2_webpage_hits_admin", 
];

fn main() {
    cl::exit_if_not_run_in_rust_project_root_directory();

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
                    /*
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
                     */
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
    fn internal_fn_cd_member_cargo_auto(member:&str) {
        let workspace_dir = std::env::current_dir().unwrap();
        println!("Processing member: {member}");
        std::env::set_current_dir(member).unwrap();
        cl::run_shell_command("cargo auto;");
        cl::run_shell_command("rm -rf ./automation_tasks_rs/target/debug/*/");
        cl::run_shell_command("rm -rf ./automation_tasks_rs/target/debug/.fingerprint/");
        std::env::set_current_dir(workspace_dir).unwrap();
    }
    
    for member in MEMBERS.iter(){
        internal_fn_cd_member_cargo_auto(member);  
    }
            
    cl::run_shell_command("rm -rf ./automation_tasks_rs/target/debug/*/");
    cl::run_shell_command("rm -rf ./automation_tasks_rs/target/debug/.fingerprint/");
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
    {YELLOW}Welcome to cargo-auto !
    This program automates your custom tasks when developing a Rust project.{RESET}

    User defined tasks in automation_tasks_rs:{RESET}
{GREEN}cargo auto{RESET} - {YELLOW}builds the automation_tasks_rs for every member{RESET}
{GREEN}cargo auto build{RESET} - {YELLOW}builds the crate in debug mode, fmt, increment version{RESET}

    {YELLOW}© 2025 bestia.dev  MIT License github.com/automation-tasks-rs/cargo-auto{RESET}
"#
    );
    /*
    cargo auto release{RESET} - {YELLOW}builds the crate in release mode, fmt, increment version{RESET}
{GREEN}cargo auto doc{RESET} - {YELLOW}builds the docs, copy to docs directory{RESET}
{GREEN}cargo auto test{RESET} - {YELLOW}runs all the tests{RESET}
{GREEN}cargo auto commit_and_push "message"{RESET} - {YELLOW}commits with message and push with mandatory message
    (If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.){RESET}
{GREEN}cargo auto publish_to_web{RESET} - {YELLOW}publish to my google VM, git tag
    (You need credentials for publishing. I use ssh-agent and ssh-add to store my credentials for SSH.){RESET} */
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build"];
        /*, "release", "doc", "test", "commit_and_push","publish_to_web" */
       cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
       cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// Build every member of pseudo-workspace. 
fn task_build() {
    fn internal_fn_cd_member_cargo_auto_build(member:&str) {
        let workspace_dir = std::env::current_dir().unwrap();
        println!("Processing member {member}");
        std::env::set_current_dir(member).unwrap();
        cl::run_shell_command("cargo auto build;");
        std::env::set_current_dir(workspace_dir).unwrap();
    }
            
    // build every member separately. 
    for member in MEMBERS.iter(){
        internal_fn_cd_member_cargo_auto_build(member);  
    }
    
    cl::run_shell_command(&format!("rsync -a --info=progress2 --delete-after tier1_browser_wasm/pkg/ web_server_folder/{APP_MAIN_ROUTE}/pkg/"));
            
    println!(
        r#"{YELLOW}
    After `cargo auto build`, run the compiled binary, examples and/or tests
cd web_server_folder ; ../tier2_webpage_hits_admin/target/debug/{APP_MAIN_ROUTE} ; cd ..
    In the browser or in curl open 
http://localhost:8080/{APP_MAIN_ROUTE}/b1_authn_signup_mod/b1_authn_signup
http://localhost:8080/{APP_MAIN_ROUTE}/b2_authn_login_mod/b2_authn_login
http://localhost:8080/{APP_MAIN_ROUTE}/a4_system_mod/a4_string_encrypt_decrypt
http://localhost:8080/{APP_MAIN_ROUTE}/c1_webpage_hits_mod/c1_webpage_hits_list
    If ok then
cargo auto release
{RESET}"#
    );
}

/*
/// build release every member of workspace. One is wasm project, so instead of cargo build, I use wam-pack build
/// this workspace is basically one single application splitted into 3 projects
/// it deserves the same version number for the release build. It means that it will build all members. 
/// A little slower than only build.
fn task_release() {
    // let cargo_toml = cl::CargoToml::read();
    //auto_check_micro_xml(&format!("web_server_folder/{APP_MAIN_ROUTE}"));
    auto_version_increment_semver_or_date_forced();    
    cl::run_shell_command("cargo fmt");

    //cl::run_shell_command("cd tier1_browser_wasm;wasm-pack build --target web --release;cd ..");
    // copy to web_server_folder/pkg
    //cl::run_shell_command(&format!("rsync -a --info=progress2 --delete-after tier1_browser_wasm/pkg/ web_server_folder/{APP_MAIN_ROUTE}/pkg/"));

    cl::auto_cargo_toml_to_md();

    cl::auto_lines_of_code("");

    // when I use --exclude tier1_browser_wasm, cargo rebuild a bunch of dependencies !?!
    cl::run_shell_command("cargo build --release --workspace ");    
    cl::run_shell_command(&format!("strip target/release/{APP_MAIN_ROUTE}"));
    cl::run_shell_command("cd tier1_browser_wasm; wasm-pack build --target web --release ; cd ..");
    cl::run_shell_command(&format!("rsync -a --info=progress2 --delete-after tier1_browser_wasm/pkg/ web_server_folder/{APP_MAIN_ROUTE}/pkg/"));

    println!(
        r#"{YELLOW}
    After `cargo auto release`, run the compiled binary, examples and/or tests
cd web_server_folder ; ../target/release/{APP_MAIN_ROUTE}; cd ..
    In the browser or in curl open 
http://localhost:8080/{APP_MAIN_ROUTE}/c1_webpage_hits_mod/c1_webpage_hits_list
    If ok then
cargo auto doc
{RESET}"#,
    );
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");
    cl::auto_plantuml(&cargo_toml.package_repository().unwrap());
    cl::auto_playground_run_code();
    cl::auto_md_to_doc_comments();

    cl::run_shell_command("cargo doc --no-deps --document-private-items");
    // copy target/doc into docs/ because it is github standard
    cl::run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
    // Create simple index.html file in docs directory
    cl::run_shell_command(&format!(
        r#"printf "<meta http-equiv=\"refresh\" content=\"0; url={}/index.html\" />" > docs/index.html\n"#,
        cargo_toml.package_name().replace("-", "_")
    ));
    // pretty html
    cl::auto_doc_tidy_html().unwrap();
    cl::run_shell_command("cargo fmt");
    // message to help user with next move
    println!(
        r#"
    {YELLOW}After `cargo auto doc`, check `docs/index.html`. If ok then test the documentation code examples{RESET}
{GREEN}cargo auto test{RESET}
"#
    );
}

/// cargo test
fn task_test() {
    cl::run_shell_command("cargo test");
    println!(
r#"
    {YELLOW}After `cargo auto test`. If ok then {RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}
    {YELLOW}with mandatory commit message{RESET}
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    let Some(message) = arg_2 else {
        eprintln!("{RED}Error: Message for commit is mandatory. Exiting.{RESET}");
        // early exit
        return;
    };

    // init repository if needed. If it is not init then normal commit and push.
    if !cl::init_repository_if_needed(&message) {
        // separate commit for docs if they changed, to not make a lot of noise in the real commit
        if std::path::Path::new("docs").exists() {
            cl::run_shell_command(r#"git add docs && git diff --staged --quiet || git commit -m "update docs" "#);
        }
        cl::add_message_to_unreleased(&message);
        // the real commit of code
        cl::run_shell_command(&format!( r#"git add -A && git diff --staged --quiet || git commit -m "{message}" "#));
        cl::run_shell_command("git push");
        println!(
r#"
    {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET}
"#
        );
    }
}

/// publish to web for podman container and git tag
fn task_publish_to_web() {
    println!(r#"{YELLOW}Use ssh-agent and ssh-add to store the credentials.{RESET}"#);
    let cargo_toml = cl::CargoToml::read();
    // git tag
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = cargo_toml.package_version()
    );
    cl::run_shell_command(&shell_command);

    // rsync files
    //cl::run_shell_command("rsync -e ssh -a --info=progress2 ./target/release/{APP_MAIN_ROUTE} luciano_bestia@bestia.dev:/var/www/transfer_folder/{APP_MAIN_ROUTE}/");
    //cl::run_shell_command("rsync -e ssh -a --info=progress2 ./.env luciano_bestia@bestia.dev:/var/www/transfer_folder/{APP_MAIN_ROUTE}/");
    //cl::run_shell_command("rsync -e ssh -a --info=progress2 ./deploy/buildah_image_{APP_MAIN_ROUTE}.sh luciano_bestia@bestia.dev:/var/www/transfer_folder/{APP_MAIN_ROUTE}/");
    //cl::run_shell_command("rsync -e ssh -a --info=progress2 ./deploy/{APP_MAIN_ROUTE}_pod_create.sh luciano_bestia@bestia.dev:/var/www/transfer_folder/{APP_MAIN_ROUTE}/");

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
 */
// endregion: tasks

video tutorial: https://www.youtube.com/watch?v=3yA18aZJRlg

About: This is a video about Auto Generate candid file
Level: Beginner 
- 00:23 add the test snippet
- 00:49 add cargo dependces
- 01:04 test command in termnil
- 01:28 add the candid type
- 02:01 add candid type to structs

quick rust start: https://internetcomputer.org/docs/current/developer-docs/backend/rust/rust-quickstart

snippet: https://curly-grouse-3e9.notion.site/Autogenerate-candid-284335afac5d4469a65a92ffe2c396b7?pvs=4

source code: https://github.com/aliscie/auto_generate_canid


```rs

use ic_cdk::export::candid::{
    candid_method, CandidType, check_prog, Deserialize, export_service, IDLProg, TypeEnv,
};

#[derive(CandidType)]
struct Example {
    name: String,
    age: i32,
}

#[candid_method(query)]
#[ic_cdk::query]
fn greet(name: String) -> Example {
    Example {
        name,
        age: 26,
    }
}


#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::fs::{create_dir_all, write};
    use std::path::PathBuf;
    use ic_cdk::{api, update};
    use std::env;
    use std::path::Path;

    use ic_cdk::api::management_canister::main::CanisterSettings;
    use ic_cdk::export::candid::{Principal};


    use ic_cdk::export::candid::{
        candid_method, CandidType, check_prog, Deserialize, export_service, IDLProg, TypeEnv,
    };

    use super::*;

    #[test]
    fn save_candid_2() {
        #[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
        fn export_candid() -> String {
            export_service!();
            __export_service()
        }

        let dir: PathBuf = env::current_dir().unwrap();
        let canister_name: Cow<str> = dir.file_name().unwrap().to_string_lossy();

        match create_dir_all(&dir) {
            Ok(_) => println!("Successfully created directory"),
            Err(e) => println!("Failed to create directory: {}", e),
        }

        let res = write(dir.join(format!("{:?}.did", canister_name).replace("\"", "")), export_candid());
        println!("-------- Wrote to {:?}", dir);
        println!("-------- res {:?}", canister_name);
    }
}
```

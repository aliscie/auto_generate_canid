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
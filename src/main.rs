extern crate clap;

use clap::{Arg, App};
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::path::Path;


fn main() {

    build_cmd();
    

    // // We will iterate through the references to the element returned by
    // // env::vars_os();
    // for (key, value) in env::vars_os() {
    //     println!("{:?}: {:?}", key, value);
    // }

    // //检查当前目录下是否有.envrc文件
    // match check_envrc_file() {
    //     true => load_rc(),
    //     false => println!("nothing todo"),
    // }
}

fn build_cmd(){
    let matches = App::new("direnv")
                            .version("1.0")
                            .author("Robbin Han. <luckyhanbin@gmail.com>")
                            .about("direnv for rust")
                            .arg(Arg::with_name("hook")
                                .short("h")
                                .long("hook")
                                .takes_value(true))
                            .arg(Arg::with_name("exports")
                                .short("e")
                                .long("exports")
                                .takes_value(true))
                            .get_matches();

                        

    if let Some(hook) = matches.value_of("hook") {
        // reg hook
        register_hook(hook.to_string());
    }

    if let Some(exports) = matches.value_of("exports") {
        if check_envrc_file() == true {
            load_rc();
        }
    }
}

fn register_hook(hook :String) {
    if hook == "bash" {
        println!("export PROMPT_COMMAND='eval \"$(direnv -e bash)\"'")
    }
}

fn check_envrc_file() -> bool {
    return Path::new(".envrc").exists();
}
// 加载文件到当前环境
fn load_rc() -> std::result::Result<(),Box<std::error::Error>> {
    let mut f = File::open(".envrc")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    println!("{}", buffer);

    Ok(())
}
/**
 * @Author: plucky
 * @Date: 2022-07-17 17:58:28
 * @LastEditTime: 2022-07-24 19:10:06
 * @Description: 
 */


use derive_builder::*;

#[derive(Builder2)]
pub struct Command {
    executable: String,
    #[builder(each = "arg",default="vec![]")]
    args: Vec<String>,
    #[builder(each = "env")]
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    // let command = Command::builder()
//         .executable("cargo".to_owned())
//         .arg("build".to_owned())
//         .arg("--release".to_owned())
//         .build()
//         .unwrap();

//     assert_eq!(command.executable, "cargo");
//     assert_eq!(command.args, vec!["build", "--release"]);
}

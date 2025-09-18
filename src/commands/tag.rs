use crate::{utils::run_git};

pub fn tag(desc: &str){
    run_git(&["tag", "--annotate", "%s", "--message={}".replace("{}", desc).as_str()]);
    run_git(&["push", "origin", "--tags"]);
}
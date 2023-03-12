use wasm_bindgen::prelude::*;

#[wasm_bindgen(module="/index.js")]
extern {
    pub fn clear();
    pub fn output(msg: &str);
    pub fn pause();
    pub fn play(msg: &str);
    pub fn redirect(msg: &str);
    pub fn set_input(cmd: &str);
}

const HELP_MSG: &'static str = "ls, cat, and bash are all you'll need. Good luck!<br>I'm merely simulating a bash shell, so many commands don't work. I didn't implement <i>everything.";
const FILENAMES: [&'static str; 5] = ["intro.txt", "work-exp.txt", "projects.txt", "contact.sh", "ysaye4.m4a"];
const LEGAL_PATHS: [&'static str; 7] = ["~", ".", "~/.", "/home/guest", "/home/guest/.", "~/", "/home/guest/",];
const TXT_CONTENTS: [&'static str; 3] = [
    "",
    "",
    ""
];

const EMPTY_STRING: String = String::new();
static mut CMD_HISTORY: [String; 10] = [EMPTY_STRING; 10];
static mut HEAD: usize = 0;
static mut ITER: usize = 0;

fn print_generic_error_msg(cmd: &str) {
    for c in cmd.chars() {
        if c == '<' && cmd.len() > 1 {
            output("<span class=\"warn\">Trying to inject something, are we? ;)");
            return;
        }
        
        if !c.is_ascii_alphabetic() {
            output(format!("{}: Symbol not allowed", c).as_str());
            return;
        }
    }

    output(format!("{}: Command not found", cmd).as_str());
}

fn exec_contact_sh(split_cmd: Vec<&str>) {
    if split_cmd.len() != 3 {
        output("USAGE: bash contact.sh [email|linkedin]");
    } else if split_cmd[2] == "email" {
        redirect("mailto:aakhalid@umich.edu");
    } else if split_cmd[2] == "linkedin" {
        redirect("https://linkedin.com/in/aman-khalid");
    } else {
        output("USAGE: bash contact.sh [email|linkedin]");
    }
}

fn exec_bash(split_cmd: Vec<&str>) {
    if split_cmd.len() == 1 {
        output("bash: Must specify executable");
        return;
    }

    if split_cmd[1] == "contact.sh" {
        exec_contact_sh(split_cmd);
        return;
    }

    for path in LEGAL_PATHS[..5].iter() {
        if split_cmd[1] == format!("{}/contact.sh", path) {
            exec_contact_sh(split_cmd);
            return;
        }
    }

    output("bash: No such file or directory");
}

fn exec_cat(split_cmd: Vec<&str>) {
    if split_cmd.len() != 2 {
        output("cat: Requires exactly 1 argument");
        return;
    }

    for i in 0..3 {
        if split_cmd[1] == FILENAMES[i] {
            output(TXT_CONTENTS[i]);
        }
    }
}

fn exec_cd(split_cmd: Vec<&str>) {
    if split_cmd.len() == 1 {
        output("");
    } else if split_cmd.len() != 2 {
        output("cd: Too many arguments");
    } else if !LEGAL_PATHS.contains(&split_cmd[1]) {
        output("cd: Permission denied");
    } else {
        output("");
    }
}

fn exec_ls(split_cmd: Vec<&str>) {
    if split_cmd.len() > 2 {
        output("ls: Too many arguments");
    } else if split_cmd.len() == 1 || LEGAL_PATHS.contains(&split_cmd[1]) {
        output(FILENAMES.join("<br>").as_str());
    } else {
        output("ls: Permission denied");
    }
}

fn exec_play(split_cmd: Vec<&str>) {
    if split_cmd.len() != 2 {
        output("play: Requires exactly 1 argument");
        return;
    }

    if split_cmd[1] == "ysaye4.m4a" {
        play("ysaye");
        return;
    } else if split_cmd[1] == "machine.mp3" {
        play("floyd");
        return;
    }

    for path in LEGAL_PATHS[..5].iter() {
        if split_cmd[1] == format!("{}/ysaye4.m4a", path) {
            play("ysaye");
        } else if split_cmd[1] == format!("{}/machine.mp3", path) {
            play("floyd");
            return;
        }
    }

    output("play: No such file or directory");
}

unsafe fn add_to_history(cmd: &str) {
    CMD_HISTORY[HEAD] = cmd.to_string();
    HEAD = (HEAD + 1) % CMD_HISTORY.len();
    ITER = HEAD;
}

#[wasm_bindgen]
pub unsafe fn init_history() {
    CMD_HISTORY[CMD_HISTORY.len() - 1] = "\0".to_string();
}

#[wasm_bindgen]
pub unsafe fn get_older_cmd() {
    let next: usize = if ITER == 0 { CMD_HISTORY.len() - 1 } else { ITER - 1 };
    let cmd: &str = CMD_HISTORY[next].as_str();

    if cmd != "\0" && next != HEAD {
        ITER = next;
        set_input(cmd);
    }
}

#[wasm_bindgen]
pub unsafe fn get_newer_cmd() {
    if ITER == HEAD {
        return;
    }

    ITER = (ITER + 1) % CMD_HISTORY.len();
    if ITER == HEAD {
        set_input("");
    } else {
        set_input(CMD_HISTORY[ITER].as_str());
    }
}

#[wasm_bindgen]
pub fn exec(cmd: &str) {
    let parsed_cmd = cmd.trim().to_ascii_lowercase();

    if parsed_cmd.is_empty() {
        output("");
        return;
    }

    let split_cmd: Vec<&str> = parsed_cmd.split(" ").collect();

    match split_cmd[0] {
        "bash" => exec_bash(split_cmd),
        "cat" => exec_cat(split_cmd),
        "cd" => exec_cd(split_cmd),
        "clear" => clear(),
        "exit" => redirect("about:blank"),
        "help" => output(HELP_MSG),
        "ls" => exec_ls(split_cmd),
        "pause" => pause(),
        "play" => exec_play(split_cmd),
        "pwd" => output("/home/guest"),
        "uname" => output("I use Arch btw"),
        "whoami" => output("guest"),
        _ => print_generic_error_msg(split_cmd[0])
    };

    unsafe { add_to_history(cmd); }
}
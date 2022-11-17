use std::{net::TcpStream, fs};

use ssh_rs::{ssh, Session};

// Will download file from lab46 to 'puzzles/{semester}/{puzzle}.{extension}
fn dump_remote(session: &mut Session<TcpStream>, path: &str, semester: &str, puzzle: &str, extension: &str) {
    let puzzles_path = format!("./puzzles/{semester}/");
    // Create dir if doesn't exist already
    fs::create_dir_all(&puzzles_path).unwrap();
    let scp = session.open_scp().unwrap();
    scp.download(format!("{puzzles_path}{puzzle}.{extension}").as_str(), path)
        .expect(format!("Issue grabbing {extension}").as_str());
}

fn get_manifest(session: &mut Session<TcpStream>, semester: &str, class: &str, puzzle: &str) {
    dump_remote(session, format!("/var/public/{semester}/{class}/pct{puzzle}/MANIFEST").as_str(), semester, puzzle, "manifest");
}

fn get_puzzle(session: &mut Session<TcpStream>, semester: &str, class: &str, puzzle: &str, user: &str) {
    dump_remote(session, format!("/var/public/{semester}/{class}/pct{puzzle}/{user}/puzzle").as_str(), semester, puzzle, "puzzle");
}

fn main() {
    // Parse config
    let config = fs::read_to_string("config.json").expect("NO 'config.json' FOUND IN CURRENT DIR");
    let config: serde_json::Value = serde_json::from_str(&config).unwrap();
    let mut missing = String::new();
    let server = config["server"].as_str().unwrap_or_else(|| { missing.push_str(" server"); "" });
    let user = config["user"].as_str().unwrap_or_else(|| { missing.push_str(" user"); "" });
    let pass = config["pass"].as_str().unwrap_or_else(|| { missing.push_str(" pass"); "" });
    let semester = config["semester"].as_str().unwrap_or_else(|| { missing.push_str(" semester"); "" });
    let class = config["class"].as_str().unwrap_or_else(|| { missing.push_str(" class"); "" });
    if !missing.is_empty() {
        panic!("config is missing{missing}")
    }
    
    let mut session = ssh::create_session();
    session.set_user_and_password(user, pass);
    session.connect(format!("{}:22", server)).unwrap();

    let puzzle = "1";

    get_manifest(&mut session, semester, class, puzzle);
    get_puzzle(&mut session, semester, class, puzzle, "pleblanc");
    session.close().unwrap();   
}
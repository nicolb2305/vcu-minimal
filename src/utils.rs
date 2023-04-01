use base64::{engine::general_purpose, Engine as _};
use lazy_static::lazy_static;
use regex::Regex;
use sysinfo::{ProcessExt, System, SystemExt};

use crate::client::{ApiError, ApiResult};

fn find_process(system: &System) -> ApiResult<String> {
    system
        .processes()
        .iter()
        .filter(|(_, p)| p.name() == "LeagueClientUx.exe")
        .map(|(_, p)| p.cmd().join(" "))
        .next()
        .ok_or(ApiError::ProcessNotFoundError)
}

fn extract_info(cmd_args: String) -> ApiResult<(String, u16)> {
    lazy_static! {
        static ref PORT_RE: Regex = Regex::new(r"--app-port=([0-9]*)").unwrap();
        static ref TOKEN_RE: Regex = Regex::new(r"--remoting-auth-token=([\w-]*)").unwrap();
    }

    let port = PORT_RE
        .captures(&cmd_args)
        .and_then(|x| x.get(1))
        .and_then(|x| x.as_str().parse::<u16>().ok())
        .ok_or(ApiError::PortNotFoundError)?;

    let token = TOKEN_RE
        .captures(&cmd_args)
        .and_then(|x| x.get(1))
        .map(|x| x.as_str())
        .ok_or(ApiError::TokenNotFoundError)?;

    Ok((token.to_owned(), port))
}

fn encode_token(remote_token: &str) -> String {
    let token = format!("riot:{}", remote_token);
    general_purpose::STANDARD.encode(token)
}

pub fn port_and_auth() -> ApiResult<(u16, String)> {
    let mut system = System::new();
    system.refresh_processes();
    let process = find_process(&system)?;
    let (token, port) = extract_info(process)?;
    let auth = encode_token(&token);

    Ok((port, auth))
}

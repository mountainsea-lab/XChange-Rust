use crate::exchange_specification::ExchangeSpecification;
use base64::{Engine, engine::general_purpose};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct AuthUtils;

impl AuthUtils {
    /// Generates a BASE64 Basic Authentication String
    pub fn get_basic_auth(user: &str, pass: &str) -> String {
        let auth = format!("{}:{}", user, pass);
        let encoded = general_purpose::STANDARD.encode(auth.as_bytes());
        format!("Basic {}", encoded)
    }

    /// Set API & Secret key from default "secret.keys"
    pub fn set_api_and_secret_key(spec: &mut ExchangeSpecification, prefix: Option<&str>) {
        Self::set_api_and_secret_key_with_prefix(spec, prefix);
    }

    /// Set API & Secret key from "{prefix}-secret.keys" or "secret.keys"
    pub fn set_api_and_secret_key_with_prefix(
        spec: &mut ExchangeSpecification,
        prefix: Option<&str>,
    ) {
        if let Some(props) = Self::get_secret_properties(prefix) {
            if let Some(api_key) = props.get("apiKey") {
                spec.api_key = Some(api_key.clone());
            }
            if let Some(secret_key) = props.get("secretKey") {
                spec.secret_key = Some(secret_key.clone());
            }
        }
    }

    /// Read secret properties from file "{prefix}-secret.keys" or "secret.keys"
    pub fn get_secret_properties(prefix: Option<&str>) -> Option<HashMap<String, String>> {
        let resource = match prefix {
            Some(p) => format!("{}-secret.keys", p),
            None => "secret.keys".to_string(),
        };

        // 1️⃣ Try to read from executable's resources folder (if bundled)
        let mut in_stream: Option<File> = None;
        if let Some(Ok(file)) = std::env::current_exe()
            .ok()
            .map(|mut path| {
                path.set_file_name(&resource);
                path
            })
            .map(File::open)
        {
            in_stream = Some(file);
        }

        // 2️⃣ Try user's home .ssh folder
        if in_stream.is_none() {
            if let Some(home) = dirs::home_dir() {
                let mut keyfile = PathBuf::from(&home);
                keyfile.push(".ssh");
                keyfile.push(&resource);
                if keyfile.is_file() {
                    if let Ok(file) = File::open(keyfile) {
                        in_stream = Some(file);
                    }
                }
            }
        }

        // 3️⃣ Load properties
        if let Some(mut file) = in_stream {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                let mut map = HashMap::new();
                for line in contents.lines() {
                    let line = line.trim();
                    if line.starts_with('#') || line.is_empty() {
                        continue;
                    }
                    if let Some(pos) = line.find('=') {
                        let key = line[..pos].trim().to_string();
                        let value = line[pos + 1..].trim().to_string();
                        map.insert(key, value);
                    }
                }
                return Some(map);
            }
        }

        None
    }
}

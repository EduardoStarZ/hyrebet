use sha2::Sha256;
use sha2::Digest;

pub fn hash(input : String) -> String {
    return Sha256::digest(input).into_iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
}

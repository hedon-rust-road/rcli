use clap::Parser;
use enum_dispatch::enum_dispatch;

use super::verify_file;
use crate::{
    process::{
        get_input,
        jwt::{process_jwt_sign, process_jwt_verify},
    },
    utils::print_verify_result,
    CmdExector,
};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(about = "Generate a jwt.")]
    Sign(JwtSignOpts),
    #[command(about = "Verify the jwt.")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    /// Secret key.
    #[arg(value_parser = verify_file, default_value = "-")]
    pub key: String,
    /// Subject ID.
    #[arg(long)]
    pub sub: String,
    /// Audience.
    #[arg(long)]
    pub aud: String,
    // Expiration Time, default time unit is `s`.
    // Support time unit like `s(second)`, `m(minute)`, `h(hour)`, `d(day)`.
    #[arg(long, required = false, value_parser = parse_exp, default_value_t = 7200)]
    pub exp: u64,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    /// Secret key.
    #[arg(value_parser = verify_file, default_value = "-")]
    pub key: String,
    /// JSON Web Token.
    #[arg(short = 't', long)]
    pub jwt: String,
}

fn parse_exp(duration: &str) -> Result<u64, String> {
    let duration = duration.to_lowercase();
    let mut total_second: u64 = 0;
    let mut current_number = String::new();
    for ch in duration.chars() {
        if ch.is_ascii_digit() {
            current_number.push(ch);
        } else {
            let multiplier = match ch {
                's' => 1,
                'm' => 60,
                'h' => 60 * 60,
                'd' => 24 * 60 * 60,
                _ => return Err("invalid exp".to_string()),
            };
            let value: u64 = current_number
                .parse()
                .map_err(|e| format!("{}", e).to_string())?;
            total_second += value * multiplier;
            current_number.clear();
        }
    }

    if !current_number.is_empty() {
        let value: u64 = current_number
            .parse()
            .map_err(|e| format!("{}", e).to_string())?;
        total_second += value;
    }
    Ok(total_second)
}

impl CmdExector for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = get_input(&self.key)?;
        let key = key.as_slice();
        let res = process_jwt_sign(key, self.sub, self.aud, self.exp)?;
        println!("\n{res}");
        Ok(())
    }
}

impl CmdExector for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = get_input(&self.key)?;
        let key = key.as_slice();
        let res = process_jwt_verify(key, self.jwt)?;
        print_verify_result(res);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::jwt::parse_exp;

    #[test]
    fn test_parse_exp() -> Result<(), String> {
        assert_eq!(parse_exp("1")?, 1);
        assert_eq!(parse_exp("1s")?, 1);
        assert_eq!(parse_exp("1S")?, 1);
        assert_eq!(parse_exp("1m")?, 60);
        assert_eq!(parse_exp("1s1m")?, 61);
        assert_eq!(parse_exp("1h")?, 3600);
        assert_eq!(parse_exp("1s100m1h")?, 1 + 100 * 60 + 3600);
        assert_eq!(parse_exp("1s1h1d")?, 1 + 3600 + 3600 * 24);
        assert!(parse_exp("1asa").is_err());
        assert!(parse_exp("xxx").is_err());
        Ok(())
    }
}

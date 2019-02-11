use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, AppSettings, Arg,
};
use clipboard::{ClipboardContext, ClipboardProvider};
use failure::Error;
use log::*;
use passwords::PasswordGenerator;
use pretty_env_logger;

pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let matches = app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::from_usage("-l --length [LENGTH] 'password length'").default_value("12"))
        .get_matches();

    debug!("{:?}", &matches);

    let pg = PasswordGenerator {
        length: matches.value_of("length").unwrap().parse().unwrap(),
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        strict: true,
    };

    let pw = pg.generate_one().unwrap();

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    // Print now clipboard.
    debug!("{:?}", ctx.get_contents());
    ctx.set_contents(pw.to_owned()).unwrap();

    Ok(println!("{}", pw))
}

#[cfg(test)]
mod tests {
    use super::*;

}

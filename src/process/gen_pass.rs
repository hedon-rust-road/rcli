use crate::opts::GenPassOpts;

pub fn process_genpass(opts: &GenPassOpts) -> anyhow::Result<()> {
    println!("{:?}", opts);
    Ok(())
}

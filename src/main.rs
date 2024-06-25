use anyhow::Result;
use mockall_double::double;
mod unicorn_gambit;

#[cfg(test)]
mod tests;

#[double]
use unicorn_gambit::UnicornPup;

fn start_something(unicorn_gambit: &UnicornPup, pup1: String, pup2: String) -> Result<()> {
    unicorn_gambit.start(pup1, pup2)?;

    println!("do something else");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let unicorn_gambit = UnicornPup::new_server();

    start_something(
        &unicorn_gambit,
        "real_addr_1".to_owned(),
        "real_addr_2".to_owned(),
    )?;

    Ok(())
}

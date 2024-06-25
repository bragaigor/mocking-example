use crate::start_something;
use anyhow::Result;
use mockall_double::double;
use tokio;

#[double]
use crate::unicorn_gambit::UnicornPup;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_start_something_mocked() -> Result<()> {
        let mut mock_unicorn_pup = UnicornPup::default();
        mock_unicorn_pup.expect_start().returning(|_pup1, _pup2| {
            println!("This is the mock version of start!!!");
            Ok(())
        });

        // This should print "This is the mock version of start!!!" to the screen instead of
        // Hello there this is the real implementation using unicorn pups: ...
        start_something(
            &mock_unicorn_pup,
            "addr111".to_owned(),
            "addr333".to_owned(),
        )?;

        Ok(())
    }
}

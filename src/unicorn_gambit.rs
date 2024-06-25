use anyhow::Result;
use mockall::automock;

pub(crate) struct UnicornPup {}

#[automock]
impl UnicornPup {
    pub fn start(&self, server_addr: String, listen_addr: String) -> Result<()> {
        println!(
            "Hello there this is the real implementation using unicorn pups: {} and listen_addr: {}",
            &server_addr, &listen_addr
        );

        Ok(())
    }

    pub fn new_server() -> Self {
        Self {}
    }
}

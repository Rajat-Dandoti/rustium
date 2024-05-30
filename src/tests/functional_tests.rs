
//use async_std::task;
use thirtyfour::By;
use crate::base::TestBase;
use crate::locators::demoLoc::EXAMPLE_LOCATOR;
use anyhow::Result;

#[tokio::test]
pub async fn example_test() -> Result<()> {
    let test_base = TestBase::new().await;
    let driver = &test_base.driver;

    driver.get("http://example.com").await?;
    let element = driver.find(By::XPath(EXAMPLE_LOCATOR)).await?;
    assert!(element.is_displayed().await?);
    test_base.close().await;
    Ok(())
}

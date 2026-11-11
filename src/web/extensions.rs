use log::info;
use thirtyfour::{Cookie, WebDriver, error::WebDriverResult};

pub trait WebDriverCookieExtension {
    fn add_all_cookies<'a, T: Iterator<Item = &'a Cookie> + Send>(
        &self,
        cookies: T,
    ) -> impl std::future::Future<Output = WebDriverResult<()>> + Send;
}

impl WebDriverCookieExtension for WebDriver {
    fn add_all_cookies<'a, T: Iterator<Item = &'a Cookie> + Send>(
        &self,
        cookies: T,
    ) -> impl std::future::Future<Output = WebDriverResult<()>> + Send {
        async {
            for cookie in cookies {
                let mut cookie = cookie.clone();

                cookie.set_secure(true);

                info!("Cookie: {:?}", cookie);

                self.add_cookie(cookie).await?;
            }
            Ok(())
        }
    }
}

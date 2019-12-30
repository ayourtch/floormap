#![allow(non_snake_case)]

use super::imports::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PageKey {
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PageState {
}

type MyPageAuth = CookiePageAuth;

impl RspState<PageKey, MyPageAuth> for PageState {
    fn get_state(req: &mut Request, auth: &MyPageAuth, key: PageKey) -> PageState {
        PageState {
            ..Default::default()
        }
    }
}

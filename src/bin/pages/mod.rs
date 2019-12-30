mod imports;

pub use imports::CookiePageAuth;

pub fn get_router() -> router::Router {
    use crate::rsp10::RspState;
    use router::Router;

    let mut r = Router::new();
    rsp10_page!(r, "/login", login, "login.rs");
    rsp10_page!(r, "/logout", logout, "logout.rs");
    rsp10_page!(r, "/arrange", arrange, "arrange.rs");
    rsp10_page!(r, "/", root, "root.rs");
    return r;
}

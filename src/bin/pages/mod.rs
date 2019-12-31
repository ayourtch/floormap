mod imports;

pub use imports::CookiePageAuth;

mod fileupload;

pub fn get_router() -> router::Router {
    use crate::rsp10::RspState;
    use router::Router;

    let mut r = Router::new();
    rsp10_page!(r, "/login", login, "login.rs");
    rsp10_page!(r, "/logout", logout, "logout.rs");
    rsp10_page!(r, "/arrange", arrange, "arrange.rs");
    rsp10_page!(r, "/", root, "root.rs");

    use multipart::server::iron::Intercept;
    use multipart::server::Entries;

    let mut multpart_chain = iron::Chain::new(fileupload::PageState::handler);
    multpart_chain.link_before(Intercept::default().file_size_limit(64000000));
    r.get("/fileupload", multpart_chain, "GET/fileupload handler");
    let mut multpart_chain = iron::Chain::new(fileupload::PageState::handler);
    multpart_chain.link_before(Intercept::default().file_size_limit(64000000));
    r.post("/fileupload", multpart_chain, "POST/fileupload handler");

    return r;
}

mod imports;
use rspten::RspState;

mod teststate;



pub fn get_router() -> router::Router {
   let mut router = router::Router::new();
   router.get("/xxx", teststate::PageState::handler, "root_page2");
   router
}



#![allow(non_snake_case)]

use super::imports::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PageKey {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PageState {}

type MyPageAuth = CookiePageAuth;

impl RspState<PageKey, MyPageAuth> for PageState {
    fn get_state(req: &mut Request, auth: &MyPageAuth, key: PageKey) -> PageState {
        PageState {
            ..Default::default()
        }
    }
    fn event_handler(
        ri: RspInfo<Self, PageKey, MyPageAuth>,
    ) -> RspEventHandlerResult<Self, PageKey> {
        use multipart::server::Entries;
        let mut action = rsp10::RspAction::Render;
        let mut initial_state = ri.initial_state;
        let mut state = ri.state;
        println!("FileUpload action!");

        if let Some(entries) = ri.req.extensions.get::<Entries>() {
            println!("Upload handler called: {:#?}", &entries);
            if let Some(file) = entries.files.get("test") {
                if file.len() == 1 {
                    let file = &file[0];
                    println!("File found: {:#?}, size: {}", &file, file.size);
                    if file.size > 0 {
                        println!("Handling multpart...");
                        let orig_fname: String = file
                            .filename
                            .as_ref()
                            .unwrap_or(&"unknown".to_string())
                            .to_string();
                        let real_fname = file.path.to_str().unwrap_or("x").to_string();
                        // handle_multpart(&orig_fname, &real_fname);
                    }
                }
            }
        } else {
            println!("multpart handler called but no entries");
        }

        RspEventHandlerResult {
            initial_state,
            state,
            action,
        }
    }
}

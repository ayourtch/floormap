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
        use floormap::db::db_insert_new_upload;
        use floormap::flexuuid::FlexUuid;
        use multipart::server::Entries;
        use std::str::FromStr;
        let mut action = rsp10::RspAction::Render;
        let mut initial_state = ri.initial_state;
        let mut state = ri.state;
        println!("FileUpload action, state: {:?}", &state);

        if let Some(entries) = ri.req.extensions.get::<Entries>() {
            println!("Upload handler called: {:#?}", &entries);
            if let (Some(file), Some(upload_for), Some(upload_comments)) = (
                entries.files.get("item_file_upload"),
                entries.fields.get("upload_for"),
                entries.fields.get("upload_comments"),
            ) {
                let upload_for_uuid = FlexUuid::from_str(&upload_for).ok();

                if file.len() == 1 && upload_for_uuid.is_some() {
                    let file = &file[0];
                    let upload_for_uuid = upload_for_uuid.unwrap();
                    println!("File found: {:#?}, size: {}", &file, file.size);
                    if file.size > 0 {
                        println!("Handling multpart...");
                        let orig_fname: String = file
                            .filename
                            .as_ref()
                            .unwrap_or(&"unknown".to_string())
                            .to_string();
                        let real_fname = file.path.to_str().unwrap_or("x").to_string();
                        let upload_res = db_insert_new_upload(
                            &ri.auth.username,
                            &upload_for_uuid,
                            &real_fname,
                            &orig_fname,
                            &upload_comments,
                        );
                        println!("Upload result: {:?}", &upload_res);

                        // handle_multpart(&orig_fname, &real_fname);
                    }
                }
            } else {
                println!(
                    "item_file_upload: {:?}, upload_for: {:?}",
                    entries.files.get("item_file_upload"),
                    entries.fields.get("upload_for")
                );
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

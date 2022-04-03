use std::rc::Rc;

use dioxus::{
    fermi::{AtomRoot, Readable},
    hooks::UnboundedReceiver,
    prelude::Atom,
    router::RouterService,
};
use futures::StreamExt;

use crate::{
    settings::path,
    types::{GenericError, SignUpFormDataRequest, UserResponse},
};

pub static SIGN_IN_ERROR: Atom<Option<GenericError>> = |_| None;

pub enum AuthService {
    SignUp(SignUpFormDataRequest),
}

pub async fn auth_service(
    mut rx: UnboundedReceiver<AuthService>,
    atoms: Rc<AtomRoot>,
    router: RouterService,
) {
    while let Some(msg) = rx.next().await {
        match msg {
            AuthService::SignUp(form_data) => {
                log::debug!("{:?}", form_data);

                let result = reqwest::Client::new()
                    .post("https://api.realworld.io/api/users")
                    .json(&form_data)
                    .send()
                    .await;

                match result {
                    Ok(response) => {
                        if response.status().is_success() {
                            match response.json::<UserResponse>().await {
                                Ok(data) => {
                                    let window = web_sys::window().expect("window");
                                    log::debug!("window: {:#?}", window);
                                    log::debug!("data: {:#?}", data);

                                    window
                                        .local_storage()
                                        .unwrap()
                                        .unwrap()
                                        .set_item("jwt", &data.user.token)
                                        .unwrap();

                                    router.push_route(path::HOME, None, None);
                                }
                                Err(_) => todo!(),
                            }
                        } else {
                            match response.json::<GenericError>().await {
                                Ok(data) => atoms.set(SIGN_IN_ERROR.unique_id(), Some(data)),
                                Err(_) => todo!(),
                            }
                        }
                    }
                    Err(_) => todo!(),
                }
            }
        }
    }
}

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
    utils::local_storage,
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
                let response = reqwest::Client::new()
                    .post("https://api.realworld.io/api/users")
                    .json(&form_data)
                    .send()
                    .await
                    .unwrap();

                if response.status().is_success() {
                    match response.json::<UserResponse>().await {
                        Ok(data) => {
                            log::debug!("data: {:#?}", data);

                            local_storage::set_item("jwt", &data.user.token);

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
        }
    }
}

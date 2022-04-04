use std::rc::Rc;

use dioxus::{
    fermi::{AtomRoot, Readable},
    hooks::UnboundedReceiver,
    prelude::Atom,
    router::RouterService,
};
use futures::StreamExt;

use crate::{
    settings::{path, JWT_KEY},
    types::{GenericError, SignUpFormDataRequest, UserResponse},
    utils::local_storage,
};

use super::api::API;

pub static SIGN_IN_ERROR: Atom<Option<GenericError>> = |_| None;

pub enum AuthService {
    SignUp(SignUpFormDataRequest),
}

pub async fn auth_service(
    mut rx: UnboundedReceiver<AuthService>,
    api: API,
    atoms: Rc<AtomRoot>,
    router: RouterService,
) {
    while let Some(msg) = rx.next().await {
        match msg {
            AuthService::SignUp(form_data) => {
                let response = api
                    .client
                    .post(API::create_url("/users"))
                    .json(&form_data)
                    .send()
                    .await
                    .unwrap();

                if response.status().is_success() {
                    match response.json::<UserResponse>().await {
                        Ok(data) => {
                            log::debug!("data: {:#?}", data);

                            local_storage::set_item(JWT_KEY, &data.user.token);

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

# ![RealWorld Example App](logo.png)

> ### [Dioxus](https://dioxuslabs.com/) codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.

### [Demo](https://demo.realworld.io/)&nbsp;&nbsp;&nbsp;&nbsp;[RealWorld](https://github.com/gothinkster/realworld)

This codebase was created to demonstrate a fully fledged fullstack application built with **Dioxus** including CRUD operations, authentication, routing, pagination, and more.

We've gone to great lengths to adhere to the **Dioxus** community styleguides & best practices.

For more information on how to this works with other frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.

## How it works

> Describe the general architecture of your app here

## Getting started

Setup rust tools:

```sh
cargo install trunk

rustup target add wasm32-unknown-unknown
```

Run dev server:

```sh
trunk serve
```

Build release:

```sh
trunk build --release
```

## TODO

- [ ] Authenticate users via JWT (login/signup pages + logout button on settings page)
- [ ] CRU\* users (sign up & settings page - no deleting required)
- [ ] CRUD Articles
- [ ] CR\*D Comments on articles (no updating required)
- [ ] GET and display paginated lists of articles
- [ ] Favorite articles
- [ ] Follow other users

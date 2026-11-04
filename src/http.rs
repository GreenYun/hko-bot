// Copyright (c) 2024 - 2026 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::sync::LazyLock;

use reqwest::Client;

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(init_client);

pub fn client() -> Client {
	HTTP_CLIENT.clone()
}

fn init_client() -> Client {
	static USER_AGENT: &str = concat!("curl/8 ", env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

	let client = Client::builder().tls_backend_rustls().user_agent(USER_AGENT).build();
	match client {
		Ok(client) => client,
		Err(e) => {
			log::error!("{e}");
			panic!("{e}");
		}
	}
}

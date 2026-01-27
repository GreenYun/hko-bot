// Copyright (c) 2024 - 2026 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::sync::OnceLock;

use reqwest::Client;

static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

pub fn client() -> Client {
	HTTP_CLIENT.get_or_init(init_client).clone()
}

fn init_client() -> Client {
	static USER_AGENT: &str = concat!("curl/8 ", env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

	let client = Client::builder().user_agent(USER_AGENT).build();
	match client {
		Ok(client) => client,
		Err(e) => {
			log::error!("{e}");
			panic!("{e}");
		}
	}
}

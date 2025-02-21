// Copyright (c) 2024 - 2025 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::sync::OnceLock;

use reqwest::Client;
use rustls::{ClientConfig, RootCertStore};
use webpki_roots::TLS_SERVER_ROOTS;

static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

pub fn client() -> Client {
	HTTP_CLIENT.get_or_init(init_client).clone()
}

fn init_client() -> Client {
	static USER_AGENT: &str = concat!("curl/8 ", env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

	let cert_store: RootCertStore = TLS_SERVER_ROOTS.iter().cloned().collect();
	let mut tls = ClientConfig::builder().with_root_certificates(cert_store).with_no_client_auth();
	tls.enable_early_data = true;
	tls.alpn_protocols = vec!["h2".into(), "http/1.1".into()];

	let client = Client::builder().use_preconfigured_tls(tls).user_agent(USER_AGENT).build();
	match client {
		Ok(client) => client,
		Err(e) => {
			log::error!("{e}");
			panic!("{e}");
		}
	}
}

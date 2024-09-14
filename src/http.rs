// Copyright (c) 2024 GreenYun Organization
// SPDX-License-Identifier: MIT

use std::sync::OnceLock;

use reqwest::Client;
use rustls::{ClientConfig, RootCertStore};
use webpki_roots::TLS_SERVER_ROOTS;

static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

pub fn client() -> Client {
	HTTP_CLIENT
		.get_or_init(|| match Client::builder().use_preconfigured_tls(make_tls()).build() {
			Ok(client) => client,
			Err(e) => {
				log::error!("{e}");
				panic!("{e}");
			}
		})
		.clone()
}

fn make_tls() -> ClientConfig {
	let mut cert_store = RootCertStore::empty();
	cert_store.extend(TLS_SERVER_ROOTS.iter().cloned());
	ClientConfig::builder().with_root_certificates(cert_store).with_no_client_auth()
}

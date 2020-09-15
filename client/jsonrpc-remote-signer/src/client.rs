use async_trait::async_trait;
use tokio::sync::RwLock;
use sp_core::{
	crypto::{CryptoTypePublicPair, KeyTypeId },
	traits::{CryptoStore, Error as CryptoStoreError},
	sr25519::{Public as Sr25519Public},
	vrf::{VRFTranscriptData, VRFSignature},
};
use sp_application_crypto::{ed25519, sr25519, ecdsa};

use futures::compat::Future01CompatExt;

use crate::gen_client::Client;
use jsonrpc_core_client::transports::http;


/// A remote based keystore that is either memory-based or filesystem-based.
pub struct RemoteKeystore {
	client: RwLock<Option<Client>>,
	url: String,
	max_retry: u8,
}

impl RemoteKeystore {
	/// Create a local keystore from filesystem.
	pub fn open(url: String, max_retry: Option<u8>) -> Result<Self, ()> {
		Ok(RemoteKeystore{
			client: RwLock::new(None),
			url: url,
			max_retry: max_retry.unwrap_or(10),
		})
	}

	/// Create a local keystore in memory.
	async fn ensure_connected(&self) -> Result<(), CryptoStoreError> {
		let mut w = self.client.write().await;
		if w.is_some() {
			return Ok(())
		}

		log::info!{
			target: "remote_keystore" ,
			"Attempting to connect to {:}", self.url
		};

		let mut counter = 0;
		loop {

			match http::connect::<Client>(&self.url).compat().await? {
				Ok(client) => {
					*w = Some(client);
					return Ok(())
				}
				Err(e) => {
					log::warn!{
						target: "remote_keystore",
						"Attempt {} failed: {}", counter, e
					}
				}
			}

			counter += 1;
			if self.max_retry > 0 && counter >= self.max_retry {
				log::error!{
					target: "remote_keystore",
					"Retrying to connect {:} failed {} times. Quitting.", self.url, counter
				}
				return Err(CryptoStoreError::Unavailable)
			}
		}


	}
}

#[async_trait]
impl CryptoStore for RemoteKeystore {
	async fn keys(
		&self,
		id: KeyTypeId
	) -> std::result::Result<Vec<CryptoTypePublicPair>, CryptoStoreError> {
		self.ensure_connected().await?;
		let client = self.client.read().await;
		client
			.as_ref()
			.ok_or(CryptoStoreError::Unavailable)?
			.keys(id)
			.compat()
			.await
			.map_err(|e|CryptoStoreError::Other(format!("{:}", e)) )
	}

	async fn supported_keys(
		&self,
		id: KeyTypeId,
		keys: Vec<CryptoTypePublicPair>
	) -> std::result::Result<Vec<CryptoTypePublicPair>, CryptoStoreError> {
		self.ensure_connected().await?;
		let client = self.client.read().await;
		client
			.as_ref()
			.ok_or(CryptoStoreError::Unavailable)?
			.supported_keys(id, keys)
			.compat()
			.await
			.map_err(|e|CryptoStoreError::Other(format!("{:}", e)) )
	}

	async fn sign_with(
		&self,
		id: KeyTypeId,
		key: &CryptoTypePublicPair,
		msg: &[u8],
	) -> std::result::Result<Vec<u8>, CryptoStoreError> {
		self.ensure_connected().await?;
		let client = self.client.read().await;
		client
			.as_ref()
			.ok_or(CryptoStoreError::Unavailable)?
			.sign_with(id, key.clone(), msg.to_vec())
			.compat()
			.await
			.map_err(|e|CryptoStoreError::Other(format!("{:}", e)) )
	}

	async fn sr25519_public_keys(&self, key_type: KeyTypeId) -> Vec<sr25519::Public> {
		if self.ensure_connected().await.is_err() {
			return vec![]
		};

		let client = self.client.read().await;
		match client.as_ref() {
			Some(c) => c
				.sr25519_public_keys(key_type)
				.compat()
				.await
				.unwrap_or(vec![]),
			_ => unreachable!()
		}
	}

	async fn sr25519_generate_new(
		&self,
		id: KeyTypeId,
		seed: Option<&str>,
	) -> std::result::Result<sr25519::Public, CryptoStoreError> {
		self.ensure_connected().await?;

		let client = self.client.read().await;
		client
			.as_ref()
			.ok_or(CryptoStoreError::Unavailable)?
			.sr25519_generate_new(id, seed.map(|s| s.to_string()))
			.compat()
			.await
			.map_err(|e|CryptoStoreError::Other(format!("{:}", e)) )
	}

	async fn ed25519_public_keys(&self, key_type: KeyTypeId) -> Vec<ed25519::Public> {
		if self.ensure_connected().await.is_err() {
			return vec![]
		};

		let client = self.client.read().await;
		match client.as_ref() {
			Some(c) => c
				.ed25519_public_keys(key_type)
				.compat()
				.await
				.unwrap_or(vec![]),
			_ => unreachable!()
		}
	}

	async fn ed25519_generate_new(
		&self,
		id: KeyTypeId,
		seed: Option<&str>,
	) -> std::result::Result<ed25519::Public, CryptoStoreError> {
		self.ensure_connected().await?;

		let client = self.client.read().await;
		client
			.as_ref()
			.ok_or(CryptoStoreError::Unavailable)?
			.ed25519_generate_new(id, seed.map(|s| s.to_string()))
			.compat()
			.await
			.map_err(|e|CryptoStoreError::Other(format!("{:}", e)) )
	}

	async fn ecdsa_public_keys(&self, key_type: KeyTypeId) -> Vec<ecdsa::Public> {
		if self.ensure_connected().await.is_err() {
			return vec![]
		};

		let client = self.client.read().await;
		match client.as_ref() {
			Some(c) => c
				.ecdsa_public_keys(key_type)
				.compat()
				.await
				.unwrap_or(vec![]),
			_ => unreachable!()
		}

	}

	async fn ecdsa_generate_new(
		&self,
		id: KeyTypeId,
		seed: Option<&str>,
	) -> std::result::Result<ecdsa::Public, CryptoStoreError> {
		self.ensure_connected().await?;

		let client = self.client.read().await;
		client
			.as_ref()
			.ok_or(CryptoStoreError::Unavailable)?
			.ecdsa_generate_new(id, seed.map(|s| s.to_string()))
			.compat()
			.await
			.map_err(|e|CryptoStoreError::Other(format!("{:}", e)) )
	}

	async fn insert_unknown(&self, key_type: KeyTypeId, suri: &str, public: &[u8])
		-> std::result::Result<(), ()>
	{
		self.ensure_connected().await.map_err(|_|())?;

		let client = self.client.read().await;
		client
			.as_ref()
			.ok_or(())?
			.insert_unknown(key_type, suri.to_string(), public.to_vec())
			.compat()
			.await
			.map_err(|_| () )
	}

	async fn has_keys(&self, public_keys: &[(Vec<u8>, KeyTypeId)]) -> bool {
		if self.ensure_connected().await.is_err() {
			return false
		};

		let client = self.client.read().await;
		match client.as_ref() {
			Some(c) => c
				.has_keys(public_keys.to_vec())
				.compat()
				.await
				.unwrap_or(false),
			_ => false
		}
	}

	async fn sr25519_vrf_sign(
		&self,
		key_type: KeyTypeId,
		public: &Sr25519Public,
		transcript_data: VRFTranscriptData,
	) -> std::result::Result<VRFSignature, CryptoStoreError> {
		todo! { }
	}
}

use std::{
	collections::{HashMap, HashSet},
	fs::{self, File},
	io::Write,
	path::PathBuf,
	sync::Arc,
};
use async_trait::async_trait;
use parking_lot::RwLock;
use sp_core::{
	crypto::{CryptoTypePublicPair, KeyTypeId, Pair as PairT, ExposeSecret, SecretString, Public},
	traits::{CryptoStore, Error as TraitError, SyncCryptoStore},
	sr25519::{Public as Sr25519Public, Pair as Sr25519Pair},
	vrf::{VRFTranscriptData, VRFSignature, make_transcript},
	Encode,
};
use sp_application_crypto::{ed25519, sr25519, ecdsa};

#[cfg(test)]
use sp_core::crypto::IsWrappedBy;
#[cfg(test)]
use sp_application_crypto::{AppPublic, AppKey, AppPair};

use crate::{Result, Error};


/// A local based keystore that is either memory-based or filesystem-based.
pub struct RemoteKeystore(RwLock<KeystoreInner>);

impl RemoteKeystore {
	/// Create a local keystore from filesystem.
	pub fn open<T: Into<PathBuf>>(path: T, password: Option<SecretString>) -> Result<Self> {
		let inner = KeystoreInner::open(path, password)?;
		Ok(Self(RwLock::new(inner)))
	}

	/// Create a local keystore in memory.
	pub fn in_memory() -> Self {
		let inner = KeystoreInner::new_in_memory();
		Self(RwLock::new(inner))
	}
}

#[async_trait]
impl CryptoStore for RemoteKeystore {
	async fn keys(
		&self,
		id: KeyTypeId
	) -> std::result::Result<Vec<CryptoTypePublicPair>, TraitError> {
		let raw_keys = self.0.read().raw_public_keys(id)?;
		Ok(raw_keys.into_iter()
			.fold(Vec::new(), |mut v, k| {
				v.push(CryptoTypePublicPair(sr25519::CRYPTO_ID, k.clone()));
				v.push(CryptoTypePublicPair(ed25519::CRYPTO_ID, k.clone()));
				v.push(CryptoTypePublicPair(ecdsa::CRYPTO_ID, k));
				v
			}))
	}

	async fn supported_keys(
		&self,
		id: KeyTypeId,
		keys: Vec<CryptoTypePublicPair>
	) -> std::result::Result<Vec<CryptoTypePublicPair>, TraitError> {
		let all_keys = self.keys(id).await?
			.into_iter()
			.collect::<HashSet<_>>();
		Ok(keys.into_iter()
		   .filter(|key| all_keys.contains(key))
		   .collect::<Vec<_>>())
	}

	async fn sign_with(
		&self,
		id: KeyTypeId,
		key: &CryptoTypePublicPair,
		msg: &[u8],
	) -> std::result::Result<Vec<u8>, TraitError> {
		match key.0 {
			ed25519::CRYPTO_ID => {
				let pub_key = ed25519::Public::from_slice(key.1.as_slice());
				let key_pair: ed25519::Pair = self.0.read()
					.key_pair_by_type::<ed25519::Pair>(&pub_key, id)
					.map_err(|e| TraitError::from(e))?;
				Ok(key_pair.sign(msg).encode())
			}
			sr25519::CRYPTO_ID => {
				let pub_key = sr25519::Public::from_slice(key.1.as_slice());
				let key_pair: sr25519::Pair = self.0.read()
					.key_pair_by_type::<sr25519::Pair>(&pub_key, id)
					.map_err(|e| TraitError::from(e))?;
				Ok(key_pair.sign(msg).encode())
			},
			ecdsa::CRYPTO_ID => {
				let pub_key = ecdsa::Public::from_slice(key.1.as_slice());
				let key_pair: ecdsa::Pair = self.0.read()
					.key_pair_by_type::<ecdsa::Pair>(&pub_key, id)
					.map_err(|e| TraitError::from(e))?;
				Ok(key_pair.sign(msg).encode())
			}
			_ => Err(TraitError::KeyNotSupported(id))
		}
	}

	async fn sr25519_public_keys(&self, key_type: KeyTypeId) -> Vec<sr25519::Public> {
		self.0.read().raw_public_keys(key_type)
			.map(|v| {
				v.into_iter()
				 .map(|k| sr25519::Public::from_slice(k.as_slice()))
				 .collect()
			})
			.unwrap_or_default()
	}

	async fn sr25519_generate_new(
		&self,
		id: KeyTypeId,
		seed: Option<&str>,
	) -> std::result::Result<sr25519::Public, TraitError> {
		let pair = match seed {
			Some(seed) => self.0.write().insert_ephemeral_from_seed_by_type::<sr25519::Pair>(seed, id),
			None => self.0.write().generate_by_type::<sr25519::Pair>(id),
		}.map_err(|e| -> TraitError { e.into() })?;

		Ok(pair.public())
	}

	async fn ed25519_public_keys(&self, key_type: KeyTypeId) -> Vec<ed25519::Public> {
		self.0.read().raw_public_keys(key_type)
			.map(|v| {
				v.into_iter()
				 .map(|k| ed25519::Public::from_slice(k.as_slice()))
				 .collect()
			})
    		.unwrap_or_default()
	}

	async fn ed25519_generate_new(
		&self,
		id: KeyTypeId,
		seed: Option<&str>,
	) -> std::result::Result<ed25519::Public, TraitError> {
		let pair = match seed {
			Some(seed) => self.0.write().insert_ephemeral_from_seed_by_type::<ed25519::Pair>(seed, id),
			None => self.0.write().generate_by_type::<ed25519::Pair>(id),
		}.map_err(|e| -> TraitError { e.into() })?;

		Ok(pair.public())
	}

	async fn ecdsa_public_keys(&self, key_type: KeyTypeId) -> Vec<ecdsa::Public> {
		self.0.read().raw_public_keys(key_type)
			.map(|v| {
				v.into_iter()
					.map(|k| ecdsa::Public::from_slice(k.as_slice()))
					.collect()
			})
			.unwrap_or_default()
	}

	async fn ecdsa_generate_new(
		&self,
		id: KeyTypeId,
		seed: Option<&str>,
	) -> std::result::Result<ecdsa::Public, TraitError> {
		let pair = match seed {
			Some(seed) => self.0.write().insert_ephemeral_from_seed_by_type::<ecdsa::Pair>(seed, id),
			None => self.0.write().generate_by_type::<ecdsa::Pair>(id),
		}.map_err(|e| -> TraitError { e.into() })?;

		Ok(pair.public())
	}

	async fn insert_unknown(&self, key_type: KeyTypeId, suri: &str, public: &[u8])
		-> std::result::Result<(), ()>
	{
		self.0.write().insert_unknown(key_type, suri, public).map_err(|_| ())
	}

	async fn has_keys(&self, public_keys: &[(Vec<u8>, KeyTypeId)]) -> bool {
		public_keys.iter().all(|(p, t)| self.0.read().key_phrase_by_type(&p, *t).is_ok())
	}

	async fn sr25519_vrf_sign(
		&self,
		key_type: KeyTypeId,
		public: &Sr25519Public,
		transcript_data: VRFTranscriptData,
	) -> std::result::Result<VRFSignature, TraitError> {
		let transcript = make_transcript(transcript_data);
		let pair = self.0.read().key_pair_by_type::<Sr25519Pair>(public, key_type)
			.map_err(|e| TraitError::PairNotFound(e.to_string()))?;

		let (inout, proof, _) = pair.as_ref().vrf_sign(transcript);
		Ok(VRFSignature {
			output: inout.to_output(),
			proof,
		})
	}
}

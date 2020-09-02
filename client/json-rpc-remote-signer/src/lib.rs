
use jsonrpc_derive::rpc;
use jsonrpc_core::{IoHandler, BoxFuture, Error as RpcError};

use core::{
	task::{Context, Poll},
	pin::Pin
};

use futures::{
	channel::{
		oneshot,
		mpsc::{UnboundedSender, UnboundedReceiver, channel},
	},
	compat::Future01CompatExt,
	future::{Future as Future03, FutureExt, TryFutureExt},
	stream::Stream,
	sink::SinkExt,
};
use sp_core::{
	crypto::{KeyTypeId, CryptoTypePublicPair},
	vrf::{VRFTranscriptData, VRFSignature},
	ed25519, sr25519, ecdsa, traits::CryptoStore
};

pub mod server;

/// Substrate Remote Signer API
/// matches `sp-core::CryptoStore`
#[rpc]
pub trait RemoteSignerApi {

	/// Returns all sr25519 public keys for the given key type.
	fn sr25519_public_keys(&self, id: KeyTypeId) -> BoxFuture<Vec<sr25519::Public>>;
	/// Generate a new sr25519 key pair for the given key type and an optional seed.
	///
	/// If the given seed is `Some(_)`, the key pair will only be stored in memory.
	///
	/// Returns the public key of the generated key pair.
	fn sr25519_generate_new(
		&mut self,
		id: KeyTypeId,
		seed: Option<&str>,
	) -> BoxFuture<sr25519::Public>;
	/// Returns all ed25519 public keys for the given key type.
	fn ed25519_public_keys(&self, id: KeyTypeId) -> BoxFuture<Vec<ed25519::Public>>;
	/// Generate a new ed25519 key pair for the given key type and an optional seed.
	///
	/// If the given seed is `Some(_)`, the key pair will only be stored in memory.
	///
	/// Returns the public key of the generated key pair.
	fn ed25519_generate_new(
		&mut self,
		id: KeyTypeId,
		seed: Option<&str>,
	) -> BoxFuture<ed25519::Public>;
	/// Returns all ecdsa public keys for the given key type.
	fn ecdsa_public_keys(&self, id: KeyTypeId) -> BoxFuture<Vec<ecdsa::Public>>;
	/// Generate a new ecdsa key pair for the given key type and an optional seed.
	///
	/// If the given seed is `Some(_)`, the key pair will only be stored in memory.
	///
	/// Returns the public key of the generated key pair.
	fn ecdsa_generate_new(
		&mut self,
		id: KeyTypeId,
		seed: Option<&str>,
	) -> BoxFuture<ecdsa::Public>;

	/// Insert a new key. This doesn't require any known of the crypto; but a public key must be
	/// manually provided.
	///
	/// Places it into the file system store.
	///
	/// `Err` if there's some sort of weird filesystem error, but should generally be `Ok`.
	fn insert_unknown(&mut self, key_type: KeyTypeId, suri: &str, public: &[u8]) -> BoxFuture<()>;

	/// Find intersection between provided keys and supported keys
	///
	/// Provided a list of (CryptoTypeId,[u8]) pairs, this would return
	/// a filtered set of public keys which are supported by the keystore.
	fn supported_keys(
		&self,
		id: KeyTypeId,
		keys: Vec<CryptoTypePublicPair>
	) -> BoxFuture<Vec<CryptoTypePublicPair>>;

	/// List all supported keys
	///
	/// Returns a set of public keys the signer supports.
	#[rpc(name="keys", returns = "Vec<CryptoTypePublicPair>")]
	fn keys(&self, id: KeyTypeId) -> BoxFuture<Vec<CryptoTypePublicPair>>;

	/// Checks if the private keys for the given public key and key type combinations exist.
	///
	/// Returns `true` iff all private keys could be found.
	fn has_keys(&self, public_keys: &[(Vec<u8>, KeyTypeId)]) -> BoxFuture<bool>;

	/// Sign with key
	///
	/// Signs a message with the private key that matches
	/// the public key passed.
	///
	/// Returns the SCALE encoded signature if key is found & supported,
	/// an error otherwise.
	fn sign_with(
		&self,
		id: KeyTypeId,
		key: &CryptoTypePublicPair,
		msg: &[u8],
	) -> BoxFuture<Vec<u8>>;

	/// Sign with any key
	///
	/// Given a list of public keys, find the first supported key and
	/// sign the provided message with that key.
	///
	/// Returns a tuple of the used key and the SCALE encoded signature.
	fn sign_with_any(
		&self,
		id: KeyTypeId,
		keys: Vec<CryptoTypePublicPair>,
		msg: &[u8]
	) -> BoxFuture<(CryptoTypePublicPair, Vec<u8>)>;

	/// Sign with all keys
	///
	/// Provided a list of public keys, sign a message with
	/// each key given that the key is supported.
	///
	/// Returns a list of `BoxFuture`s each representing the SCALE encoded
	/// signature of each key or a Error for non-supported keys.
	fn sign_with_all(
		&self,
		id: KeyTypeId,
		keys: Vec<CryptoTypePublicPair>,
		msg: &[u8],
	) -> BoxFuture<Vec<BoxFuture<Vec<u8>>>>;

	/// Generate VRF signature for given transcript data.
	///
	/// Receives KeyTypeId and Public key to be able to map
	/// them to a private key that exists in the keystore which
	/// is, in turn, used for signing the provided transcript.
	///
	/// Returns a result containing the signature data.
	/// Namely, VRFOutput and VRFProof which are returned
	/// inside the `VRFSignature` container struct.
	///
	/// This function will return an error in the cases where
	/// the public key and key type provided do not match a private
	/// key in the keystore. Or, in the context of remote signing
	/// an error could be a network one.
	fn sr25519_vrf_sign<'a>(
		&'a self,
		key_type: KeyTypeId,
		public: &sr25519::Public,
		transcript_data: VRFTranscriptData,
	) -> BoxFuture<VRFSignature>;
}

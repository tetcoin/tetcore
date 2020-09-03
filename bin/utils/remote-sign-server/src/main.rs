use structopt::StructOpt;

use jsonrpc_http_server::jsonrpc_core::IoHandler;
use jsonrpc_http_server::ServerBuilder;

use tokio;

use sc_cli::KeystoreParams;
use sc_service::config::KeystoreConfig;
use sc_keystore::LocalKeystore;
use sc_remote_signer::{
    RemoteSignerApi,
    server::GenericRemoteSignerServer
};

#[derive(Debug, StructOpt)]
#[structopt(name = "substrate-remote-sign-server", about = "keystore Server for Substrate's JSON-RPC Remote Signing Protocol")]
struct Opt {
    #[structopt(flatten)]
    keystore: KeystoreParams,
    #[structopt(long = "port", short="-p", default_value="33033")]
    port: u32,
    #[structopt(long = "interface", short="-i", default_value="127.0.0.1")]
    interface: String,
}
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let base_path = std::env::current_dir().unwrap();
    let keystore = match opt.keystore.keystore_config(&base_path) {
        Ok(KeystoreConfig::Path { path, password }) => {
            LocalKeystore::open(path, password).map_err(|e| format!("{:}", e))
        },
        Err(e) => Err(format!("{:}", e)),
        Ok(KeystoreConfig::InMemory) => unreachable!()
    }.unwrap();

    let server_addr = format!("{}:{}", opt.interface, opt.port).parse()
        .expect("Could not parse interface/port");

    let (remote_server, receiver) = GenericRemoteSignerServer::proxy(keystore);

    let mut io = IoHandler::new();
    io.extend_with(RemoteSignerApi::to_delegate(remote_server));

	let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&server_addr)
        .unwrap();

    tokio::spawn(receiver);
    let _ = tokio::task::spawn_blocking(move || {
        println!("Serving Remote Signer at {:}", server_addr);
        server.wait()
    }).await;
}
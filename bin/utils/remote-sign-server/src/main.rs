use structopt::StructOpt;

use jsonrpc_http_server::jsonrpc_core::IoHandler;
use jsonrpc_http_server::ServerBuilder;

use tokio::stream::StreamExt;

use sc_cli::KeystoreParams;
use sc_service::config::KeystoreConfig;
use sc_keystore::LocalKeystore;
use sc_remote_signer::{
    RemoteSignerApi,
    server::GenericRemoteSignerServer
};

use tokio;
use env_logger;

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
    env_logger::init();
    let opt = Opt::from_args();
    let base_path = std::env::current_dir().unwrap();
    let keystore = match opt.keystore.keystore_config(&base_path) {
        Ok(KeystoreConfig::Path { path, password }) => {
            LocalKeystore::open(path, password).map_err(|e| format!("{:}", e))
        },
        Err(e) => Err(format!("{:}", e)),
        Ok(_) => Err(format!("Only Local-Keystore Paramters supported")),
    }.unwrap();

    let server_addr = format!("{}:{}", opt.interface, opt.port).parse()
        .expect("Could not parse interface/port");

    let (remote_server, mut receiver) = GenericRemoteSignerServer::proxy(keystore);

    let mut io = IoHandler::new();
    io.extend_with(RemoteSignerApi::to_delegate(remote_server));

	let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&server_addr)
        .unwrap();

    tokio::spawn(async move {
        loop {
            if receiver.next().await == None {
                break
            }
        }
    });
    let _ = tokio::task::spawn_blocking(move || {
        println!("Serving Remote Signer at {:}", server_addr);
        server.wait()
    }).await;
}
#!/bin/bash

function rust_rename() {
    sed -i "s/$1/$2/g" `grep -Rl --include="*.rs" --include="*.stderr" "$1" *` > /dev/null
}

function cargo_rename() {
    find . -name "Cargo.toml" -exec sed -i "s/\(^\|[^\/]\)$1/\1$2/g" {} \;
}

function rename_gitlabci() {
    sed -i "s/$1/$2/g" .gitlab-ci.yml
}

function rename() {
    old=$(echo $1 | cut -f1 -d\ );
    new=$(echo $1 | cut -f2 -d\ );

    echo "Renaming $old to $new"
    # rename in Cargo.tomls
    cargo_rename $old $new
    rename_gitlabci $old $new
    # and it appears, we have the same syntax in rust files
    rust_rename $old $new

    # but generally we have the snail case syntax in rust files
    old=$(echo $old | sed s/-/_/g );
    new=$(echo $new | sed s/-/_/g );

    echo " > $old to $new"
    rust_rename $old $new
}

TO_RENAME=(
    # OLD-CRATE-NAME NEW-CRATE-NAME

    # post initial rename fixes
    "sc-application-crypto tc-application-crypto"
    "tc-transaction-pool-api tc-transaction-pool"
    "tc-transaction-pool-runtime-api tc-transaction-pool"
    "tc-core-storage tc-storage"
    "transaction-factory node-transaction-factory"
    "tc-finality-granpda tc-finality-grandpa"
    "tc-sesssion tc-session"
    "tc-tracing-pool tc-transaction-pool"
    "sc-basic-authority sc-basic-authorship"
    "sc-api sc-client-api"
    "sc-database sc-client-db"

    # PRIMITIVES
    "tetcore-application-crypto tc-application-crypto"
    "tetcore-authority-discovery-primitives tc-authority-discovery"
    "tetcore-block-builder-runtime-api tc-block-builder"
    "tetcore-consensus-aura-primitives tc-consensus-aura"
    "tetcore-consensus-babe-primitives tc-consensus-babe"
    "tetcore-consensus-common tc-consensus"
    "tetcore-consensus-pow-primitives tc-consensus-pow"
    "tetcore-primitives tc-core"
    "tetcore-debug-derive tc-debug-derive"
    "tetcore-primitives-storage tc-storage"
    "tetcore-externalities tc-externalities"
    "tetcore-finality-grandpa-primitives tc-finality-grandpa"
    "tetcore-inherents tc-inherents"
    "tetcore-keyring tc-keyring"
    "tetcore-offchain-primitives tc-offchain"
    "tetcore-panic-handler tc-panic-handler"
    "tetcore-phragmen tc-npos-elections"
    "tetcore-rpc-primitives tc-rpc"
    "tetcore-runtime-interface tc-runtime-interface"
    "tetcore-runtime-interface-proc-macro tc-runtime-interface-proc-macro"
    "tetcore-runtime-interface-test-wasm tc-runtime-interface-test-wasm"
    "tetcore-serializer tc-serializer"
    "tetcore-session tc-session"
    "sr-api tc-api"
    "sr-api-proc-macro tc-api-proc-macro"
    "sr-api-test tc-api-test"
    "sr-arithmetic tc-arithmetic"
    "sr-arithmetic-fuzzer tc-arithmetic-fuzzer"
    "sr-io tc-io"
    "sr-primitives tc-runtime"
    "sr-sandbox tc-sandbox"
    "sr-staking-primitives tc-staking"
    "sr-std tc-std"
    "sr-version tc-version"
    "tetcore-state-machine tc-state-machine"
    "tetcore-transaction-pool-runtime-api tc-transaction-pool"
    "tetcore-trie tc-trie"
    "tetcore-wasm-interface tc-wasm-interface"

    # # CLIENT
    "tetcore-client sc-client"
    "tetcore-client-api sc-client-api"
    "tetcore-authority-discovery sc-authority-discovery"
    "tetcore-basic-authorship sc-basic-authorship"
    "tetcore-block-builder sc-block-builder"
    "tetcore-chain-spec sc-chain-spec"
    "tetcore-chain-spec-derive sc-chain-spec-derive"
    "tetcore-cli sc-cli"
    "tetcore-consensus-aura sc-consensus-aura"
    "tetcore-consensus-babe sc-consensus-babe"
    "tetcore-consensus-pow sc-consensus-pow"
    "tetcore-consensus-slots sc-consensus-slots"
    "tetcore-consensus-uncles sc-consensus-uncles"
    "tetcore-client-db sc-client-db"
    "tetcore-executor sc-executor"
    "tetcore-runtime-test sc-runtime-test"
    "tetcore-finality-grandpa sc-finality-grandpa"
    "tetcore-keystore sc-keystore"
    "tetcore-network sc-network"
    "tetcore-offchain sc-offchain"
    "tetcore-peerset sc-peerset"
    "tetcore-rpc-servers sc-rpc-server"
    "tetcore-rpc sc-rpc"
    "tetcore-service sc-service"
    "tetcore-service-test sc-service-test"
    "tetcore-state-db sc-state-db"
    "tetcore-telemetry sc-telemetry"
    "tetcore-test-primitives tc-test-primitives"
    "tetcore-tracing sc-tracing"

);

for rule in "${TO_RENAME[@]}"
do
	rename "$rule";
done

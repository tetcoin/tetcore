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
    "tc-application-crypto tp-application-crypto"
    "tp-transaction-pool-api tp-transaction-pool"
    "tp-transaction-pool-runtime-api tp-transaction-pool"
    "tp-core-storage tetcore-storage"
    "transaction-factory node-transaction-factory"
    "tp-finality-granpda tp-finality-grandpa"
    "tp-sesssion tp-session"
    "tp-tracing-pool tp-transaction-pool"
    "tc-basic-authority tc-basic-authorship"
    "tc-api tc-client-api"
    "tc-database tc-client-db"

    # PRIMITIVES
    "tetcore-application-crypto tp-application-crypto"
    "tetcore-authority-discovery-primitives tp-authority-discovery"
    "tetcore-block-builder-runtime-api tp-block-builder"
    "tetcore-consensus-aura-primitives tp-consensus-aura"
    "tetcore-consensus-babe-primitives tp-consensus-babe"
    "tetcore-consensus-common tp-consensus"
    "tetcore-consensus-pow-primitives tp-consensus-pow"
    "tetcore-primitives tet-core"
    "tetcore-debug-derive debug-derive"
    "tetcore-primitives-storage tetcore-storage"
    "tetcore-externalities externalities"
    "tetcore-finality-grandpa-primitives tp-finality-grandpa"
    "tetcore-inherents tp-inherents"
    "tetcore-keyring tp-keyring"
    "tetcore-offchain-primitives tp-offchain"
    "tetcore-panic-handler panic-handler"
    "tetcore-phragmen tp-npos-elections"
    "tetcore-rpc-primitives tp-rpc"
    "tetcore-runtime-interface tp-runtime-interface"
    "tetcore-runtime-interface-proc-macro tp-runtime-interface-proc-macro"
    "tetcore-runtime-interface-test-wasm tp-runtime-interface-test-wasm"
    "tetcore-serializer serializer"
    "tetcore-session tp-session"
    "sr-api tp-api"
    "sr-api-proc-macro tp-api-proc-macro"
    "sr-api-test tp-api-test"
    "sr-arithmetic arithmetic"
    "sr-arithmetic-fuzzer arithmetic-fuzzer"
    "sr-io tet-io"
    "sr-primitives tp-runtime"
    "sr-sandbox tp-sandbox"
    "sr-staking-primitives tp-staking"
    "sr-std tetcore-std"
    "sr-version tp-version"
    "tetcore-state-machine tp-state-machine"
    "tetcore-transaction-pool-runtime-api tp-transaction-pool"
    "tetcore-trie tp-trie"
    "tetcore-wasm-interface tetcore-wasm-interface"

    # # CLIENT
    "tetcore-client tc-client"
    "tetcore-client-api tc-client-api"
    "tetcore-authority-discovery tc-authority-discovery"
    "tetcore-basic-authorship tc-basic-authorship"
    "tetcore-block-builder tc-block-builder"
    "tetcore-chain-spec tc-chain-spec"
    "tetcore-chain-spec-derive tc-chain-spec-derive"
    "tetcore-cli tc-cli"
    "tetcore-consensus-aura tc-consensus-aura"
    "tetcore-consensus-babe tc-consensus-babe"
    "tetcore-consensus-pow tc-consensus-pow"
    "tetcore-consensus-slots tc-consensus-slots"
    "tetcore-consensus-uncles tc-consensus-uncles"
    "tetcore-client-db tc-client-db"
    "tetcore-executor tc-executor"
    "tetcore-runtime-test tc-runtime-test"
    "tetcore-finality-grandpa tc-finality-grandpa"
    "tetcore-keystore tc-keystore"
    "tetcore-network tc-network"
    "tetcore-offchain tc-offchain"
    "tetcore-peerset tc-peerset"
    "tetcore-rpc-servers tc-rpc-server"
    "tetcore-rpc tc-rpc"
    "tetcore-service tc-service"
    "tetcore-service-test tc-service-test"
    "tetcore-state-db tc-state-db"
    "tetcore-telemetry tc-telemetry"
    "tetcore-test-primitives tp-test-primitives"
    "tetcore-tracing tc-tracing"

);

for rule in "${TO_RENAME[@]}"
do
	rename "$rule";
done

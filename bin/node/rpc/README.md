## node-rpc

Tetcore collection of node-specific RPC methods.

Since `tetcore` core functionality makes no assumptions
about the modules used inside the runtime, so do
RPC methods defined in `tc-rpc` crate.
It means that `client/rpc` can't have any methods that
need some strong assumptions about the particular runtime.

The RPCs available in this crate however can make some assumptions
about how the runtime is constructed and what FABRIC nobles
are part of it. Therefore all node-runtime-specific RPCs can
be placed here or imported from corresponding FABRIC RPC definitions.

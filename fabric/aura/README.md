# Aura Module

- [`aura::Trait`](https://docs.rs/noble-aura/latest/noble_aura/trait.Trait.html)
- [`Module`](https://docs.rs/noble-aura/latest/noble_aura/struct.Module.html)

## Overview

The Aura module extends Aura consensus by managing offline reporting.

## Interface

### Public Functions

- `slot_duration` - Determine the Aura slot-duration based on the Timestamp module configuration.

## Related Modules

- [Timestamp](https://docs.rs/noble-timestamp/latest/noble_timestamp/): The Timestamp module is used in Aura to track
consensus rounds (via `slots`).

## References

If you're interested in hacking on this module, it is useful to understand the interaction with
`tetcore/primitives/inherents/src/lib.rs` and, specifically, the required implementation of
[`ProvideInherent`](https://docs.rs/tp-inherents/latest/tp_inherents/trait.ProvideInherent.html) and
[`ProvideInherentData`](https://docs.rs/tp-inherents/latest/tp_inherents/trait.ProvideInherentData.html) to create and check inherents.

License: Apache-2.0

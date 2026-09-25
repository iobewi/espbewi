# espbewi

Atomic `no_std` ESP platform workspace for the IOBEWI / Embewi ecosystem.

`espbewi` centralizes ESP-specific hardware integration while keeping domain
semantics in their own projects.

```text
ESP hardware / HAL
       |
     espbewi
       |
       +-- espbewi-flash
       +-- espbewi-nvs
       +-- espbewi-partitions
       +-- espbewi-platform
       +-- espbewi-boot
       |
       +-- future: wifi / tls / time / rng
```

The repository is a workspace, **not** one monolithic crate. Consumers depend
only on the hardware capability they need.

## Current crates

### `espbewi-flash`

Owns the single process-wide ESP `FlashStorage` instance and serializes raw
flash access.

It contains no NVS, OTA, ConfigSpace or application persistence semantics.

### `espbewi-nvs`

Hardware bridge between `esp-nvs` and `espbewi-flash`.

It contains no namespace policy, quota model, record framing or configuration
schema.

### `espbewi-partitions`

Policy-free ESP-IDF partition-table lookup and bounded raw erase helpers.

It contains no OTA slot-selection, rollback or firmware transaction semantics.

### `espbewi-platform`

Pure hardware descriptors such as chip IDs and boot memory geometry. It has no
HAL dependency and is intentionally host-testable. Concrete SoC facts live
here instead of in domain projects such as FiBeWI.

### `espbewi-boot`

Low-level second-stage boot hardware primitives: ROM flash access, flash-size
setup, watchdog handoff and cache/MMU mapping. It deliberately contains no
EWBT, rollback or slot-selection policy.

## Boundary

Domain projects remain responsible for their own semantics:

- `config-space-manager`: ConfigSpace ownership, quotas, generations and framing;
- FiBeWI: firmware transactions, EWBT, A/B decisions and rollback;
- applications: provisioning, HTTP/TLS policy, identity and product behavior.

## Targets

Initial CI-gated target:

- ESP32-C3

Feature scaffolding is also provided for ESP32-S3; target-specific validation
will be added as that hardware path is brought up.

## License

MIT.

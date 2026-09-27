# espbewi

Atomic `no_std` ESP platform workspace for the IOBEWI / Embewi ecosystem.

`espbewi` centralizes ESP-specific implementation while keeping generic domain
semantics in their own projects.

The repository is organized by responsibility:

```text
espbewi/
├── hardware/
│   ├── platform
│   ├── flash
│   ├── nvs
│   ├── partitions
│   └── boot
├── adapters/
│   ├── ota
│   └── config-space
├── services/
│   ├── wifi
│   └── tls
└── bootloader/
    └── esp
```

- `hardware/` contains policy-free ESP hardware/platform primitives.
- `adapters/` implements generic external domain contracts on ESP.
- `services/` provides reusable ESP-facing services above the raw hardware layer.
- `bootloader/` contains executable artifacts rather than reusable library crates.

Package names remain stable; the directory hierarchy expresses ownership and
dependency direction.

## Hardware

### `espbewi-platform`

Pure hardware descriptors such as chip IDs and boot memory geometry. It has no
HAL dependency and is intentionally host-testable.

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

### `espbewi-boot`

Low-level second-stage boot hardware primitives: ROM flash access, flash-size
setup, watchdog handoff and cache/MMU mapping. It deliberately contains no
EWBT, rollback or slot-selection policy.

## Adapters

### `espbewi-ota`

Concrete ESP partition/NOR-flash adapter for `fibewi::ArtifactStorage`.
It owns ESP slot lookup, erase geometry and physical artifact writes while
FiBeWI keeps transactional OTA policy and restart-safe reconciliation.

### `espbewi-config-space`

Concrete ESP/NVS implementation of `config_space_manager::ConfigBackend`.
It owns ConfigSpace record framing, NVS capacity accounting and backend health
checks while `config-space-manager` remains hardware-agnostic.

## Services


### `espbewi-wifi`

Reusable ESP Wi-Fi station integration for esp-radio and Embassy networking.

### `espbewi-tls`

Reusable ESP HAL / MbedTLS integration with Embassy networking adapters.

## Executables

### `bootloader/esp`

The ESP second-stage executable. It owns the HAL runtime, linker layout,
ROM/MMU/watchdog execution and final jump, and consumes `fibewi::boot`
for EWBT/A-B/rollback decisions.

## Dependency direction

```text
generic domains
      ↑
   adapters
      ↑
hardware/platform

services
   ↑
hardware/platform
```

Hardware crates never depend on adapters or services.

Domain projects remain responsible for their own semantics:

- `config-space-manager`: ConfigSpace ownership, quotas and generations;
- FiBeWI: firmware transactions, EWBT, A/B decisions and rollback;
- applications: provisioning, HTTP/TLS policy, identity and product behavior.

## Targets

CI-gated target:

- ESP32-C3

ESP32-S3 platform, TLS and bootloader support are also present.

## License

MIT.

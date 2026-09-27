# PedCalc

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust: stable](https://img.shields.io/badge/rust-stable-orange.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Leptos v0.7](https://img.shields.io/badge/Leptos-v0.7-blue.svg)](https://leptos.dev)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/suradet-ps/ped-calc/issues)

---

## ◆ PULSE

A child is not a small adult, and a pediatric dose is not a fraction.
PedCalc computes dosage ranges from actual body weight for nine common
pediatric drugs - amoxicillin to oseltamivir - with clinical alerts for
contraindications and renal or hepatic adjustments, mg-to-mL conversion
from real formulations, and history kept in the browser. The math is
`rust_decimal`-exact, the data is cited to formularies, and the page
runs entirely on the machine in front of the clinician.

| 9 drugs ▣ | mg-to-mL ▣ | Alerts ▣ | History ▣ |
|---|---|---|---|

*The calculator - weight in, verified range out - is sealed.*

> Built with Rust + Leptos, computed with `rust_decimal`, cited to
> BNFc, Harriet Lane, and the Thai Pediatric Formulary.
>
> **suradet-ps**, artifact keeper

---

## ◆ IGNITION

One target, one tool, one command.

```
⟫ rustup target add wasm32-unknown-unknown
⟫ cargo install trunk
⟫ trunk serve
```

Open [http://127.0.0.1:8080](http://127.0.0.1:8080).

The release artifact: `⟫ trunk build --release` - output in `dist/`,
deployed to Vercel as a static site (`build.sh` installs Rust, the WASM
target, and Trunk on the build machine).

<details>
<summary>Prerequisites</summary>

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021)
- [Trunk](https://trunkrs.dev/) - installed above
- The `wasm32-unknown-unknown` target

</details>

---

## ◆ ANATOMY

Five layers, one number, no server in the path.

- **Calculates** - `logic/calculator.rs` derives dosage ranges from
  actual body weight with `rust_decimal` arithmetic - money-style
  precision for a dose that cannot be rounded wrong.
- **Adjusts** - `logic/adjuster.rs` applies the clinical modifiers:
  renal and hepatic impairment shift the range, and the alert says so
  before the dose does.
- **Validates** - `logic/validator.rs` refuses the impossible: an
  empty weight, a missing age, a dose outside the formulary's limits.
- **Converts** - mg becomes mL against the drug's formulation - the
  syringe sees the number the brain computed.
- **Remembers** - calculation history persists in `localStorage`; the
  drug database is static, in-memory Rust (`data/drugs.rs`), cited to
  five named sources and updated deliberately.
- **Renders** - calculator, drug reference, and about pages in
  `pages/`, separated from the logic so a UI change never rewrites a
  dose.

---

## ◆ RITUALS

**The core ceremony** - the pediatric dose:

1. Enter weight and age. The form accepts nothing incomplete.
2. Select the drug. The database answers with its range, its alerts,
   and its adjustments.
3. Review: the clinically validated limits, the mg-to-mL conversion,
   the contraindication notes - all on the page before the pen moves.
4. Verify against the child's chart, then administer. The disclaimer
   is not decoration: this tool is a second pair of eyes, not a
   verdict.

**The ceremony of the source** - every dosing constant traces to a
named reference: BNF for Children, Harriet Lane, the Thai Pediatric
Formulary, UpToDate, WHO guidelines. The data is cited so it can be
audited.

**The ceremony of the client** - nothing is sent anywhere: no server,
no account, no telemetry. The calculation happens where the patient
is, in the browser that opened it.

---

## ◆ ECHOES

**Where this artifact is heading**

```
calc     ▸ weight-based ranges, rust_decimal math ──────────────────── ▸ sealed
adjust   ▸ renal/hepatic modifiers, clinical alerts ────────────────── ▸ sealed
convert  ▸ mg-to-mL via formulations ───────────────────────────────── ▸ sealed
remember ▸ localStorage history ────────────────────────────────────── ▸ sealed
deliver  ▸ static WASM on Vercel ───────────────────────────────────── ▸ sealed
```

**Raising the artifact** - the contribution path and code of conduct
live in `CONTRIBUTING.md`; the optimization flags in `Cargo.toml` and
`Trunk.toml` (including `wasm-opt`). Open an issue first to discuss a
change.

**Status** - dependencies are maintained through Renovate; deployment
is a Vercel connect away.

---

```
  ─────────────────────────────────────────
   A pediatric dose is a number
   that must survive the rounding.
  ─────────────────────────────────────────
```

Licensed under the [MIT License](LICENSE).
//! Long-form project content: prose, design notes, data tables, figures,
//! install snippets and canonical links.
//!
//! Figures are quoted from the projects' own benchmark reports and are marked as
//! such on the page; nothing here is an independent measurement.

use crate::route::Project;

#[derive(Clone, Copy)]
pub struct DataTable {
    pub title: &'static str,
    pub note: &'static str,
    pub caption: &'static str,
    pub headers: [&'static str; 3],
    pub rows: &'static [(&'static str, &'static str, &'static str)],
}

#[derive(Clone, Copy)]
pub struct Figure {
    pub src: &'static str,
    pub alt: &'static str,
    pub caption: &'static str,
}

pub struct ProjectDetail {
    pub intro: &'static [&'static str],
    pub mechanisms: &'static [(&'static str, &'static str)],
    pub table: Option<DataTable>,
    pub figures: &'static [Figure],
    pub install_title: &'static str,
    pub install_html: &'static str,
    pub install_plain: &'static str,
    pub install_note: &'static str,
    /// (label, sub-label, href)
    pub links: &'static [(&'static str, &'static str, &'static str)],
    pub platform_note: &'static str,
    pub art_note: &'static str,
}

pub fn get(p: Project) -> &'static ProjectDetail {
    match p {
        Project::Dtact => &DTACT,
        Project::Ensia => &ENSIA,
        Project::Bincode => &BINCODE,
        Project::Rssn => &RSSN,
        Project::CargoSlide => &CARGO_SLIDE,
        Project::Virtue => &VIRTUE,
        Project::Aevia => &AEVIA,
    }
}

static DTACT: ProjectDetail = ProjectDetail {
    intro: &[
        "dtact is a non-preemptive, stackful coroutine runtime. Instead of compiling async code into state machines, it gives every task its own stack and switches between them with hand-written assembly. That one decision makes the rest possible: a suspended task keeps its C stack, so foreign code can suspend and resume it.",
        "Scheduling is a peer-to-peer mesh rather than a global work-stealing queue. Workers exchange work through bounded lock-free mailboxes, so a saturated core deflects its backlog into a neighbouring cache domain instead of contending on one shared structure.",
        "Contexts are carved out of a pre-allocated arena, which turns spawning into an index allocation, and futures small enough to fit are placed on the fiber's own stack. The project is explicit about the trade-off: a yield is a full register switch, heavier than a stackless runtime, and worth it when the work between yields is substantial or when C has to participate.",
    ],
    mechanisms: &[
        (
            "The spawn path",
            "A spawn takes a slot from the context arena, writes the future into the slot's zero-copy buffer and publishes the handle. There is no allocator call and no queue insertion on the critical path.",
        ),
        (
            "Deflection instead of stealing",
            "A full mailbox hands work to a neighbour rather than waiting: each worker pushes to peers and pulls only from its own mailbox, so contention stays inside a cache domain.",
        ),
        (
            "Affinity that is enforced",
            "SameCore pins, SameCCX and SameNUMA confine deflection, Any lets work travel. P2PMesh, Global and Pinned set the runtime default; topology_mode overrides it per fiber.",
        ),
        (
            "Priorities with consequences",
            "Low and High bias the deflection threshold. Critical is never deflected, enters at the front of its worker's queue, and parks rather than migrating when the queue is full.",
        ),
        (
            "Per-architecture switchers",
            "Assembly context switches for x86_64, AArch64 and RISC-V, with variants that skip floating-point state or switch across threads — you pay for the state you need.",
        ),
        (
            "Stack safety levels",
            "Safety0 performs no overflow check, Safety1 checks a canary at termination, Safety2 checks at every suspension. Trips are counted, and named fibers appear in diagnostics.",
        ),
    ],
    table: Some(DataTable {
        title: "Measured performance",
        note: "project benchmarks",
        caption: "Criterion benchmarks from benches/scheduler_efficiency.rs: 4-core x86_64, 4 workers per runtime, 50 samples of 5 s after a 2 s warm-up. Per-task cost is total time divided by N, read from the published charts.",
        headers: ["Workload", "dtact", "Note"],
        rows: &[
            ("Spawn + join, N = 1,000", "≈195 ns per task", "≈5.3× cheaper than Tokio"),
            ("Spawn + join, N = 100,000", "≈450 ns per task", "advantage narrows with batch size"),
            ("Hot-core deflection", "ahead at both extremes", "dips near N ≈ 4,000–16,000"),
            ("Yield cost", "full register switch", "stackful by design — amortise the work"),
        ],
    }),
    figures: &[Figure {
        src: "/assets/img/bench/spawn_join.png",
        alt: "Two charts: per-task spawn and join cost in nanoseconds for dtact and Tokio from N=1,000 to 100,000, and the ratio between them.",
        caption: "Spawn + join: median per-task cost in nanoseconds, and Tokio's cost divided by dtact's. Above 1.0 means dtact is faster.",
    }],
    install_title: "shell — cargo",
    install_html: "<span class=\"c\"># the runtime</span>\ncargo add dtact\n\n<span class=\"c\"># async I/O on the same scheduler</span>\ncargo add dtact-util",
    install_plain: "cargo add dtact\ncargo add dtact-util",
    install_note: "Requires Rust 1.98.1. The C API ships as dtact.h; dtact-util exposes io, fs, process, signal, stream and timer through the same ABI, with io_uring, IOCP and kqueue backends.",
    links: &[
        ("crates.io", "dtact", "https://crates.io/crates/dtact"),
        ("docs.rs", "API reference", "https://docs.rs/dtact"),
        ("Benchmark workflow", "CI report", "https://github.com/Apich-Organization/dtact/actions/workflows/benchmark.yml"),
        ("DOI", "10.5281/zenodo.20002105", "https://doi.org/10.5281/zenodo.20002105"),
        ("Software Heritage", "archived source", "https://archive.softwareheritage.org/swh:1:dir:62bd3423ead4bec1ae8dea921046e96a7e4a1ebc"),
        ("This Week in Rust", "issue 653", "https://this-week-in-rust.org/blog/2026/05/27/this-week-in-rust-653/"),
        ("OpenSSF Best Practices", "project 12962", "https://www.bestpractices.dev/projects/12962"),
        ("C header", "dtact.h", "https://github.com/Apich-Organization/dtact/blob/main/dtact.h"),
    ],
    platform_note: "x86_64 · AArch64 · RISC-V",
    art_note: "stackful · mesh-scheduled",
};

static ENSIA: ProjectDetail = ProjectDetail {
    intro: &[
        "Ensia (OLLVM-Next) is an LLVM-based obfuscation framework that continues the Hikari lineage with a transformation engine rebuilt for LLVM 21 and later. It targets Linux, Windows (MSVC, clang-cl, MinGW), macOS, iOS and Android, with first-class support for x86_64, AArch64 and i386.",
        "The design rejects the virtualizer's single point of failure. There is no central interpreter to devirtualise; fifteen passes are interleaved across a twelve-stage cascade, so the output of each pass is the input of the next and an analyst has to survive the composition rather than defeat one mechanism.",
        "Support is tiered rather than claimed uniformly: Linux and Windows on x86_64 and AArch64 are release blockers, macOS and FreeBSD follow, and the rest — OpenBSD, iOS, Android, RISC-V, i386 — is best-effort with issue response only.",
    ],
    mechanisms: &[
        (
            "Cascade order, not a pass list",
            "Anti-hook and anti-class-dump run first, then wrappers and call obfuscation, then the per-function loop, then a second constant-encryption phase and branch encryption. Each stage assumes the previous one already ran.",
        ),
        (
            "Per-function loop",
            "Inside the loop the order is fixed: substitution, MBA, block splitting, bogus control flow, then either the chaos state machine or classic flattening, then vector lifting. Splitting before flattening is what forces MBA expressions across block boundaries.",
        ),
        (
            "Two-phase constant encryption",
            "Programmer literals are encrypted before the CFG is rewritten, and the state constants produced by BCF, CSM and CFF are encrypted afterwards, so the keys themselves are not left in the clear.",
        ),
        (
            "Cleanup and symbol scrambling",
            "Temporary sentinels are erased, DWARF and COMDATs are stripped, the translation-unit path is anonymised, and internal symbols are rewritten to hashed names.",
        ),
        (
            "Policy engine",
            "Ordered rules match module paths and function symbols with regular expressions; the first match wins, so a hot function can be excluded while the rest of the binary is treated at full strength.",
        ),
        (
            "Toolchain integration",
            "The pass plugin loads through clang's -fpass-plugin or rustc's -Z llvm-plugins. Configuration is discovered from ensia.toml, ENSIA_CONFIG or -mllvm, and presets run from low to max.",
        ),
    ],
    table: Some(DataTable {
        title: "Measured expansion",
        note: "project benchmarks",
        caption: "Mean binary code footprint expansion under the maximum profile, by cryptographic domain, across eight domains. Individual binaries range from 5.57× (ML-KEM-768) to 139.19× (RSA-2048).",
        headers: ["Domain", "Mean expansion", "Note"],
        rows: &[
            ("Post-quantum (PQC)", "5.6×", "smallest footprint"),
            ("Lightweight / AEAD", "39.0×", "—"),
            ("Block cipher", "48.2×", "—"),
            ("Hash / digest", "48.5×", "—"),
            ("Stream cipher", "50.2×", "—"),
            ("MAC / authenticator", "55.6×", "—"),
            ("KDF / password", "64.1×", "—"),
            ("Asymmetric / PKC", "103.6×", "largest footprint"),
        ],
    }),
    figures: &[Figure {
        src: "/assets/img/bench/size_expansion_light.svg",
        alt: "Horizontal bar chart of binary code size expansion per cryptographic domain, from 5.6x for post-quantum to 103.6x for asymmetric cryptography.",
        caption: "Mean binary footprint expansion by domain, with the minimum and maximum of each domain marked.",
    }],
    install_title: "shell — clang",
    install_html: "<span class=\"c\"># build the pass plugin, then load it</span>\nclang -fpass-plugin=/path/to/libEnsia.so \\\n      -O2 main.c -o main\n\n<span class=\"c\"># pick a preset instead of a full config</span>\nENSIA_PRESET=mid clang -fpass-plugin=/path/to/libEnsia.so \\\n      -O2 main.c -o main",
    install_plain: "clang -fpass-plugin=/path/to/libEnsia.so -O2 main.c -o main\nENSIA_PRESET=mid clang -fpass-plugin=/path/to/libEnsia.so -O2 main.c -o main",
    install_note: "Presets run from low to max. Configuration is discovered from ./ensia.toml, ENSIA_CONFIG, or -mllvm -ensia-config. A Rust frontend plugin is available for cargo builds. Read ETHICS.md before use: this is dual-use tooling.",
    links: &[
        ("Repository", "source + docs", "https://github.com/Apich-Organization/ensia"),
        ("Ethics notice", "read first", "https://github.com/Apich-Organization/ensia/blob/main/ETHICS.md"),
        ("DOI", "10.5281/zenodo.20149843", "https://doi.org/10.5281/zenodo.20149843"),
        ("Software Heritage", "archived source", "https://archive.softwareheritage.org/swh:1:dir:8e603e7611495a327313e6ce0da65b059f168209"),
        ("Crackme challenge", "official competition", "https://github.com/Apich-Organization/ensia#official-crackme-competition"),
        ("Pass reference", "fifteen passes", "https://github.com/Apich-Organization/ensia#comprehensive-pass-suite-15-passes"),
        ("Configuration", "ensia.toml", "https://github.com/Apich-Organization/ensia#4-structured-toml-configuration-ensiatoml"),
    ],
    platform_note: "LLVM 21 · 22 · 23+",
    art_note: "15 passes · 12 stages",
};

static BINCODE: ProjectDetail = ProjectDetail {
    intro: &[
        "bincode-next is the maintained continuation of bincode: a binary encoder and decoder pair that adds no framing, no field names and no padding. The encoded object is at most the size it occupies in memory, and usually less.",
        "The fork exists for a practical reason — RSSN and the wider ecosystem needed a serialization crate that was still maintained — and it has since grown its own engine: SIMD varint scanning, relative-pointer zero-copy, bit-level packing, schema fingerprinting and an RFC 8949 CBOR front end.",
        "The format is specified rather than implied. A 64-bit schema fingerprint covers field names, types, order and configuration, so a format change (bincode to CBOR, endianness, integer encoding) produces a different fingerprint instead of silently corrupt data.",
    ],
    mechanisms: &[
        (
            "Configuration is explicit",
            "There is no implicit default: every encode and decode call takes a Configuration value, so the wire format is a property of the call site rather than of a global.",
        ),
        (
            "Varint layout",
            "Integers below 251 occupy one byte; larger values take a marker byte plus a fixed-width integer. The choice between varint and fixint is part of the configuration, and part of the fingerprint.",
        ),
        (
            "Zero-copy decoding",
            "A relative pointer stores a byte offset instead of an address, so a decoded structure can borrow directly from the input buffer and remain valid after the buffer moves.",
        ),
        (
            "Fingerprint before payload",
            "When enabled, the 64-bit schema hash is written ahead of the payload and checked first, so a mismatched reader fails with a specific error rather than a partial decode.",
        ),
        (
            "Compatibility surface",
            "The 3.x wire format matches bincode 2.x under identical configuration, and config::legacy() reads the 1.x layout for migration.",
        ),
        (
            "no_std layering",
            "The core is core-only; std adds io::Read/Write streaming, and alloc adds the owned containers. Each layer is a feature, not a separate crate.",
        ),
    ],
    table: Some(DataTable {
        title: "Format at a glance",
        note: "specification",
        caption: "From the crate documentation and docs/spec.md. “Feature” entries are Cargo features; “config” entries are Configuration methods.",
        headers: ["Capability", "Enabled by", "What it buys"],
        rows: &[
            ("Zero-copy borrow", "feature: zero-copy", "decode &str, &[u8] and nested structs without allocating"),
            ("Bit packing", "config: with_bit_packing()", "sub-byte fields, with PACKED_MAX_SIZE as the bound"),
            ("Static bound", "feature: static-size", "worst-case encoded size known at compile time"),
            ("Fingerprint", "config: with_fingerprint()", "64-bit schema hash covering format and configuration"),
            ("CBOR", "config: with_cbor_format()", "RFC 8949 encoding with deterministic modes"),
            ("Async", "feature: async-fiber", "zero-cost async decoding via UFA"),
            ("Serde bridge", "feature: serde", "Encode/Decode for types that derive serde traits"),
        ],
    }),
    figures: &[],
    install_title: "toml — Cargo.toml",
    install_html: "[dependencies]\n<span class=\"k\">bincode-next</span> = { version = <span class=\"s\">\"{v}\"</span>, features = [<span class=\"s\">\"zero-copy\"</span>] }",
    install_plain: "[dependencies]\nbincode-next = { version = \"{v}\", features = [\"zero-copy\"] }",
    install_note: "The crate is no_std compatible; zero-copy and bit packing are opt-in. Migration notes from 1.x and 2.x are in docs/migration_guide.md.",
    links: &[
        ("crates.io", "bincode-next", "https://crates.io/crates/bincode-next"),
        ("docs.rs", "API reference", "https://docs.rs/bincode-next"),
        ("Specification", "wire format", "https://github.com/Apich-Organization/bincode/blob/main/docs/spec.md"),
        ("Migration guide", "from 1.x / 2.x", "https://github.com/Apich-Organization/bincode/blob/main/docs/migration_guide.md"),
        ("DOI", "10.6084/m9.figshare.31410402", "https://doi.org/10.6084/m9.figshare.31410402"),
        ("This Week in Rust", "issue 640", "https://this-week-in-rust.org/blog/2026/02/25/this-week-in-rust-640/"),
        ("OpenSSF Best Practices", "project 12960", "https://www.bestpractices.dev/projects/12960"),
    ],
    platform_note: "no_std · x86_64 · AArch64",
    art_note: "zero-fluff binary",
};

static RSSN: ProjectDetail = ProjectDetail {
    intro: &[
        "rssn is a scientific computing library for Rust built around one idea: every symbolic operation is an identity transformation. Differentiating, integrating, solving and simplifying are all requests — diff(f, x), solve(eq, x) — rewritten by rule sets on a hash-consed e-graph until an equal closed form, or a number, is found.",
        "That engine carries the rest of the crate. Calculus, algebra, differential equations, analysis and physics rule sets all lower to the same representation, and every scalar term carries an inferred dimension, so an answer reports its units as well as its value.",
        "The same closed forms can then be compiled: to an SSA tape with reverse gradients and interval enclosures, or through Cranelift to machine code, or across rows on a CPU pool or a wgpu device. Symbolic and numeric paths share the operator registry rather than living in separate worlds.",
    ],
    mechanisms: &[
        (
            "Rule scheduling",
            "Rewriting runs in tiers — reduce, normalise, explore — so cheap identities settle before expensive searches start, and the fixpoint terminates without unbounded exploration.",
        ),
        (
            "Extraction by target",
            "The engine keeps every equivalent form and picks one at the end: the most compact closed form for symbolic work, or the numerically stable variant for a requested tolerance.",
        ),
        (
            "Kernel manifest",
            "Every operator has a deterministic, self-describing manifest entry — signature, unit, constants — which is what plugins, the C ABI and the bincode-next wire format are built on.",
        ),
        (
            "Compilation backends",
            "Closed forms lower to an SSA tape (gradients, Hessian-vector products, interval enclosures) or to Cranelift machine code with caching and tiered compilation.",
        ),
        (
            "Host callbacks",
            "Checked callback signatures for quadrature, Brent, non-linear least squares and four ODE integrators, including dimensions and error propagation across the ABI.",
        ),
        (
            "Simulation layer",
            "Time-stepping and discretised fields live outside the identity engine: FDM, FEM, FVM, BEM, CFD, molecular dynamics, SPH, spectral and multigrid solvers with ready-made scenarios.",
        ),
    ],
    table: Some(DataTable {
        title: "Coverage at a glance",
        note: "from docs/MATH_COVERAGE.md",
        caption: "A summary of the domains the rule sets and kernels cover. The full operator list, including boundary behaviour, is in the documentation.",
        headers: ["Domain", "Representative capability", "Notes"],
        rows: &[
            ("Calculus", "derivatives, definite and improper integrals, limits, series", "Hermite reduction for algebraic integrands"),
            ("Algebra", "factorisation, Gröbner bases, CAD quantifier elimination", "Buchberger, F4 and F5"),
            ("Differential equations", "ODE classification and solution, PDE on boxes and shells", "Dirichlet, Neumann, Robin, periodic"),
            ("Numerics", "quadrature, stiff ODE solvers, Krylov, FFT, statistics", "Dormand–Prince, Radau IIA, BDF"),
            ("Physics", "classical, EM, relativity, QM, QFT, solid state", "plus FDM, FEM, FVM, BEM, CFD, MD, SPH"),
            ("Output", "text, LaTeX, Typst, Unicode, plots, .npy", "round-trippable"),
        ],
    }),
    figures: &[],
    install_title: "shell — cargo",
    install_html: "<span class=\"c\"># the library</span>\ncargo add rssn\n\n<span class=\"c\"># optional: JIT compilation of closed forms</span>\ncargo add rssn --features jit",
    install_plain: "cargo add rssn\ncargo add rssn --features jit",
    install_note: "A C and C++ interface is generated with cbindgen (rssn.h, rssn.hpp) and built with scripts/build_ffi.sh. GPU execution is available behind the gpu-wgpu feature.",
    links: &[
        ("crates.io", "rssn", "https://crates.io/crates/rssn"),
        ("docs.rs", "API reference", "https://docs.rs/rssn"),
        ("Architecture", "ARCHITECTURE.md", "https://github.com/Apich-Organization/rssn/blob/v0.3/ARCHITECTURE.md"),
        ("Math coverage", "operator list", "https://github.com/Apich-Organization/rssn/blob/v0.3/docs/MATH_COVERAGE.md"),
        ("Backends", "execution model", "https://github.com/Apich-Organization/rssn/blob/v0.3/docs/BACKENDS.md"),
        ("DOI", "10.6084/m9.figshare.31044715", "https://doi.org/10.6084/m9.figshare.31044715"),
        ("This Week in Rust", "issue 653", "https://this-week-in-rust.org/blog/2026/05/27/this-week-in-rust-653/"),
    ],
    platform_note: "Linux · macOS · Windows · wgpu",
    art_note: "symbolic · numeric · compiled",
};

static CARGO_SLIDE: ProjectDetail = ProjectDetail {
    intro: &[
        "cargo-slide is a presentation engine for people who would rather write a deck than lay one out. Slides are Typst markup; playback is a native Rust runtime on tiny-skia, so there is no browser engine, no Electron and no 60 MB of JavaScript between you and the next slide.",
        "A deck can leave the machine in three shapes: a single executable that runs anywhere, a compressed .slide archive that plays in the viewer, or a static HTML/WASM bundle with no inline JavaScript for the web.",
        "The editor is part of the tool. It renders the slides as vector graphics in the document flow, lets you edit the Typst inline on the canvas with debounced recompilation, and keeps an outline, speaker notes and a pacing estimate beside the deck.",
    ],
    mechanisms: &[
        (
            "Compile, then blit",
            "Typst produces SVG, usvg normalises it, and tiny-skia rasterises the result into the window. Playback is a tight loop over prepared pages, which is where the 60 FPS comes from.",
        ),
        (
            "Archive format",
            "A .slide file is an LZMA2 archive of the compiled deck plus its assets: about a megabyte for a typical talk, playable without a toolchain.",
        ),
        (
            "WASM target",
            "The same runtime compiles to WebAssembly for online talks and embedding, producing a static bundle with no inline JavaScript.",
        ),
        (
            "SQL-backed charts",
            "Charts are queried from a data source at build or play time, so a deck can show live numbers instead of a screenshot.",
        ),
        (
            "Animation as a trait",
            "Transitions and shader-like effects are implemented as traits, so custom motion lives in Rust next to the deck rather than in a GUI.",
        ),
        (
            "Encrypted decks",
            "A PIV token or a software key pair wraps the deck; the archive is decrypted into memory and the key material is zeroized afterwards.",
        ),
    ],
    table: Some(DataTable {
        title: "Three deliverables",
        note: "delivery formats",
        caption: "From the project README. Sizes are the project's own figures for a typical deck; runtime dependencies are what the audience machine needs.",
        headers: ["Format", "Size", "Runtime requirement"],
        rows: &[
            ("Standalone binary", "~14–15 MB", "none — fully self-contained"),
            (".slide archive", "~1 MB", "slide-viewer"),
            ("WebAssembly bundle", "static HTML + WASM", "any modern browser"),
        ],
    }),
    figures: &[],
    install_title: "shell — cargo",
    install_html: "<span class=\"c\"># install the CLI and editor</span>\ncargo install cargo-slide\n\n<span class=\"c\"># scaffold a five-file project and play it</span>\ncargo slide init talk\ncd talk && cargo slide present",
    install_plain: "cargo install cargo-slide\ncargo slide init talk\ncd talk && cargo slide present",
    install_note: "Rust 1.89 or newer. slide-editor provides the WYSIWYG canvas; slide-viewer plays .slide archives on machines without a toolchain.",
    links: &[
        ("crates.io", "cargo-slide", "https://crates.io/crates/cargo-slide"),
        ("docs.rs", "API reference", "https://docs.rs/cargo-slide"),
        ("Repository", "source + examples", "https://github.com/Apich-Organization/cargo-slide"),
        ("Typst", "typesetting engine", "https://typst.app/"),
        ("tiny-skia", "rendering backend", "https://github.com/RazrFalcon/tiny-skia"),
    ],
    platform_note: "Linux · macOS · Windows · WASM",
    art_note: "typst in · pixels out",
};

static VIRTUE: ProjectDetail = ProjectDetail {
    intro: &[
        "virtue is a helper for writing proc-macro derives. A derive macro is mostly boilerplate: parse the item, walk its fields, emit an impl, and get the generics, where-clauses and lifetimes right. virtue makes that declarative.",
        "You parse the input, get a Generator, and describe the impl you want — functions, arguments, bodies, attributes — through a typed API. The library owns the token plumbing, so a mistake is usually a compile error in your macro rather than a malformed expansion in your user's crate.",
        "It is deliberately small. There are no required dependencies (proc-macro2 is optional), which keeps the cost of adding it to a proc-macro crate close to zero, and minor releases keep the API stable. The version it actually needs is recorded in the crate metadata; the policy only caps how high that can go.",
    ],
    mechanisms: &[
        (
            "Parse and generate",
            "Parse::new reads the derive input; into_generator hands back a Generator plus the item body to match on.",
        ),
        (
            "Typed builder chain",
            "generate_impl, generate_fn, with_self_arg and body builders replace quote! templates with method calls the compiler can check.",
        ),
        (
            "Attribute handling",
            "Helper attributes are read through the same parse layer, so #[some(attr)] can change what is generated without a second parser.",
        ),
        (
            "Generic plumbing",
            "Generics, where-clauses and lifetimes are copied into the generated impl correctly, including the bounds the trait requires.",
        ),
        (
            "Spanned errors",
            "Errors carry a span, so a bad derive points at the offending token instead of the macro invocation.",
        ),
        (
            "Optional dependency",
            "proc-macro2 is behind a feature: crates that already depend on it enable it, and everyone else pays nothing.",
        ),
    ],
    table: None,
    figures: &[],
    install_title: "toml — Cargo.toml",
    install_html: "[lib]\nproc-macro = <span class=\"k\">true</span>\n\n[dependencies]\n<span class=\"k\">virtue-next</span> = <span class=\"s\">\"{v}\"</span>",
    install_plain: "[lib]\nproc-macro = true\n\n[dependencies]\nvirtue-next = \"{v}\"",
    install_note: "The crate is published as virtue-next. It is used by bincode-next's derive crate and by other derive-heavy crates in the organization.",
    links: &[
        ("crates.io", "virtue-next", "https://crates.io/crates/virtue-next"),
        ("docs.rs", "API reference", "https://docs.rs/virtue-next"),
        ("Repository", "source + tests", "https://github.com/Apich-Organization/virtue"),
    ],
    platform_note: "proc-macro · stable Rust",
    art_note: "declarative derives",
};

static AEVIA: ProjectDetail = ProjectDetail {
    intro: &[
        "Aevia is a functional language for scientific computing whose type system knows about dimensions. An expression such as 3 m + 2 s is rejected before the program runs, because metres and seconds do not add — and the compiler can say so at the point of the mistake.",
        "The language is small on purpose and leans on RSSN for the hard parts: symbolic transformation, numerical algorithms and physical-unit metadata all come from the library, so an expression can be inspected symbolically in the REPL and evaluated numerically in the same session.",
        "The toolchain is a CLI, a language server and a package manager, with an installer that verifies a SHA-256 checksum and the GitHub release attestation before anything is placed on PATH. The book, the standard-library reference and the language specification are published with the platform.",
    ],
    mechanisms: &[
        (
            "The check pipeline",
            "Source is parsed, names resolved, then types and dimensions are checked together: a unit is part of the type, so a dimensional error is reported by the same pass that reports a type error.",
        ),
        (
            "Lowering to Core IR",
            "Checked code becomes a small core language that the interpreter runs with typed operator dispatch and tail calls — the surface syntax never reaches the evaluator.",
        ),
        (
            "Module cache",
            "Checked interfaces are cached with bincode-next and a schema fingerprint; an incompatible cache entry is rebuilt rather than misread.",
        ),
        (
            "Package registry",
            "aevia-pkg resolves dependencies, writes a lockfile, and publishes signed packages: a short-lived publish statement binds name, version and digest, and its nonce is single-use.",
        ),
        (
            "Documentation without execution",
            "The documentation builder parses source as data and never runs package code, so a published page cannot execute anything on the reader's machine.",
        ),
        (
            "Sandboxed tutorials",
            "The optional tutorial runner executes examples in a rootless container with no network and no host mounts.",
        ),
    ],
    table: None,
    figures: &[],
    install_title: "shell — install",
    install_html: "<span class=\"c\"># verify, then install the toolchain</span>\ncurl -fsSL https://aevia.apich.org/install.sh | sh\n\n<span class=\"c\"># check a model and run it</span>\naevia check examples/kinematics.aev\naevia run examples/kinematics.aev",
    install_plain: "curl -fsSL https://aevia.apich.org/install.sh | sh\naevia check examples/kinematics.aev\naevia run examples/kinematics.aev",
    install_note: "The platform — registry, book, standard library and playground — lives at aevia.apich.org. The installer requires the GitHub CLI with release attestation support.",
    links: &[
        ("aevia.apich.org", "platform + docs", "https://aevia.apich.org/"),
        ("The Aevia Book", "interactive edition", "https://aevia.apich.org/book?lang=en"),
        ("Standard library", "reference", "https://aevia.apich.org/stdlib?lang=en"),
        ("Language specification", "single page", "https://github.com/Apich-Organization/aevia/blob/v0.1/Aevia-Language-Specification.en.html"),
        ("Repository", "source + examples", "https://github.com/Apich-Organization/aevia"),
        ("llms.txt", "for text clients", "https://aevia.apich.org/llms.txt"),
    ],
    platform_note: "Linux · macOS · Windows",
    art_note: "3 m + 2 s is a type error",
};

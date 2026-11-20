//! Project data shared by cards, tables, structured data and the JSON export.
//!
//! Every figure here is taken from the project's manifest, README, or its
//! published benchmark report. These values are also emitted into JSON-LD and
//! `/data/projects.json`, where they are read by machines — keep them factual.

use crate::content::versions;
use crate::route::Project;

pub struct Feature {
    pub icon: &'static str,
    pub title: &'static str,
    pub body: &'static str,
}

pub struct ProjectContent {
    pub id: Project,
    /// One-line positioning, used on cards and in `<title>`s.
    pub tagline: &'static str,
    /// Two or three sentences of neutral description.
    pub summary: &'static str,
    pub category: &'static str,
    pub version: &'static str,
    pub license: &'static str,
    pub repo: &'static str,
    pub docs: &'static str,
    pub doi: &'static str,
    pub crate_url: &'static str,
    /// Open Collective project page (falls back to the organization account).
    pub funding: &'static str,
    /// Lowest monthly tier on the project collective, e.g. "$5".
    pub funding_from: &'static str,
    pub languages: &'static [&'static str],
    pub platforms: &'static [&'static str],
    /// (value, label, note)
    pub stats: &'static [(&'static str, &'static str, &'static str)],
    pub features: &'static [Feature],
}

pub fn get(p: Project) -> &'static ProjectContent {
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

pub fn all() -> impl Iterator<Item = &'static ProjectContent> {
    Project::ALL.into_iter().map(get)
}

/// Version string with the conventional `v` prefix. A value that does not start
/// with a digit is shown verbatim, which keeps a toolchain label readable.
pub fn version_label(p: Project) -> String {
    let v = get(p).version;
    if v.starts_with(|c: char| c.is_ascii_digit()) {
        format!("v{v}")
    } else {
        v.to_string()
    }
}

static DTACT: ProjectContent = ProjectContent {
    id: Project::Dtact,
    tagline: "The universal topology-affinity async runtime",
    summary: "A non-preemptive, stackful coroutine runtime built for hardware-level control and heterogeneous orchestration: a lock-free context arena, peer-to-peer mesh scheduling, and hand-written assembly context switchers.",
    category: "Runtime",
    version: versions::DTACT,
    license: "MIT OR Apache-2.0",
    repo: "https://github.com/Apich-Organization/dtact",
    docs: "https://docs.rs/dtact",
    doi: "https://doi.org/10.5281/zenodo.20002105",
    crate_url: "https://crates.io/crates/dtact",
    funding: "https://opencollective.com/apich-organization/projects/dtact",
    funding_from: "$5",
    languages: &["Rust", "C", "C++"],
    platforms: &["x86_64", "AArch64", "RISC-V", "Linux", "macOS", "Windows"],
    stats: &[
        ("0.19 µs", "Per task at N=1,000", "spawn + join"),
        ("5.4×", "Cheaper than Tokio", "same benchmark"),
        ("7", "Spawn-time knobs", "priority · affinity · kind"),
        ("3", "Architectures", "x86_64 · AArch64 · RISC-V"),
    ],
    features: &[
        Feature {
            icon: "layers",
            title: "Lock-free context arena",
            body: "Fiber contexts come from a pre-allocated arena, so spawning is an index allocation rather than a heap allocation — even at high spawn rates.",
        },
        Feature {
            icon: "grid",
            title: "P2P mesh scheduling",
            body: "Workers exchange work through bounded, lock-free mailboxes instead of one global work-stealing queue, which keeps contention local to a cache domain.",
        },
        Feature {
            icon: "compass",
            title: "Topology-aware affinity",
            body: "SameCore, SameCCX, SameNUMA or Any: deflection is confined to the neighbourhood you choose, per runtime or per fiber.",
        },
        Feature {
            icon: "cpu",
            title: "Assembly context switchers",
            body: "Hand-written switchers per architecture, with floating-point and cross-thread variants, so the cost of a switch matches the workload.",
        },
        Feature {
            icon: "zap",
            title: "Zero-copy futures",
            body: "Small and medium futures are placed directly on the fiber's own stack, removing a heap allocation from the spawn path.",
        },
        Feature {
            icon: "terminal",
            title: "C and C++ FFI",
            body: "Fibers have their own stacks, so C code can launch and await them. dtact-util exposes io, fs, process, signal, stream and timer through the same ABI.",
        },
    ],
};

static ENSIA: ProjectContent = ProjectContent {
    id: Project::Ensia,
    tagline: "The next-generation LLVM obfuscator",
    summary: "A high-resilience compiler obfuscation framework for modern LLVM toolchains, rebuilding the transformation engine behind the Hikari lineage for LLVM 21 and later.",
    category: "Security",
    version: versions::ENSIA,
    license: "AGPL-3.0-only",
    repo: "https://github.com/Apich-Organization/ensia",
    docs: "https://github.com/Apich-Organization/ensia#usage--integration",
    doi: "https://doi.org/10.5281/zenodo.20149843",
    crate_url: "",
    funding: "https://opencollective.com/apich-organization/projects/ensia",
    funding_from: "$5",
    languages: &["C++", "LLVM IR"],
    platforms: &["Linux", "Windows", "macOS", "FreeBSD", "Android", "iOS"],
    stats: &[
        ("15", "Obfuscation passes", "interleaved, not stacked"),
        ("12", "Pipeline stages", "cascading"),
        ("5.6–103.6×", "Binary size expansion", "by domain"),
        ("2³²", "CSM state space", "Q32 fixed point"),
    ],
    features: &[
        Feature {
            icon: "shield",
            title: "No single point of failure",
            body: "Ensia rejects the virtualizer's central interpreter. Fifteen passes are composed so that removing one does not reveal the rest.",
        },
        Feature {
            icon: "layers",
            title: "Chaos state machine",
            body: "Control flow is replaced by a logistic-map dynamical system in Q32 fixed point, with a 2³² state space and cellular-automaton attractors.",
        },
        Feature {
            icon: "cpu",
            title: "Hardware opaque predicates",
            body: "CPUID feature bits, RDTSC parity and CNTPCT_EL0 make branch conditions impossible to fold without executing them.",
        },
        Feature {
            icon: "lock",
            title: "String and constant encryption",
            body: "Vernam OTP over a Rijndael GF(2⁸) layer, decrypted by inlined stubs that zeroize on return; constants use MBA shares or Feistel mixing.",
        },
        Feature {
            icon: "git",
            title: "Import elimination",
            body: "Function-call obfuscation resolves calls at runtime through dlopen/dlsym or GetProcAddress, so the import table stops describing the binary.",
        },
        Feature {
            icon: "file",
            title: "Metadata elimination",
            body: "DWARF, COMDATs, llvm.ident and internal symbols are stripped or scrambled, and every function is stamped noinline optnone to survive LTO.",
        },
    ],
};

static BINCODE: ProjectContent = ProjectContent {
    id: Project::Bincode,
    tagline: "High-performance binary serialization for Rust",
    summary: "A zero-fluff binary encoder and decoder pair, modernized from the original bincode with SIMD varint scanning, zero-copy relative pointers, bit-packing, and a CBOR front end.",
    category: "Serialization",
    version: versions::BINCODE,
    license: "MIT OR Apache-2.0",
    repo: "https://github.com/Apich-Organization/bincode",
    docs: "https://docs.rs/bincode-next",
    doi: "https://doi.org/10.6084/m9.figshare.31410402",
    crate_url: "https://crates.io/crates/bincode-next",
    funding: "https://opencollective.com/apich-organization/projects/bincode-next",
    funding_from: "$5",
    languages: &["Rust"],
    platforms: &["no_std", "x86_64", "AArch64", "WASM"],
    stats: &[
        ("{version}", "Current release", "stable toolchain"),
        ("no_std", "Supported", "core only"),
        ("64-bit", "Schema fingerprint", "format + configuration"),
        ("2", "Wire formats", "bincode · CBOR"),
    ],
    features: &[
        Feature {
            icon: "zap",
            title: "SIMD varint scanning",
            body: "SSE2 on x86_64 and NEON on AArch64 scan varint streams in bulk instead of byte by byte.",
        },
        Feature {
            icon: "layers",
            title: "Zero-copy structures",
            body: "Relative pointers and const alignment let nested borrowed data decode in place, with no intermediate allocation.",
        },
        Feature {
            icon: "grid",
            title: "Bit-level packing",
            body: "Opt-in bit packing shrinks structs whose fields do not need a whole byte, with a compile-time bound on the worst case.",
        },
        Feature {
            icon: "check",
            title: "Schema fingerprinting",
            body: "A 64-bit hash over field names, types, order and configuration catches format drift before it becomes corrupt data.",
        },
        Feature {
            icon: "package",
            title: "Static size bounds",
            body: "StaticSize and PACKED_MAX_SIZE give worst-case byte counts at compile time, which is what embedded buffers need.",
        },
        Feature {
            icon: "globe",
            title: "CBOR and async decoding",
            body: "RFC 8949 CBOR with deterministic modes, plus zero-cost async decoding through the fiber-backed UFA path.",
        },
    ],
};

static RSSN: ProjectContent = ProjectContent {
    id: Project::Rssn,
    tagline: "Scientific computing for Rust",
    summary: "An open-source scientific computing library: a computer algebra system, numerical kernels, and physics simulations behind one small API.",
    category: "Science",
    version: versions::RSSN,
    license: "Apache-2.0",
    repo: "https://github.com/Apich-Organization/rssn",
    docs: "https://docs.rs/rssn",
    doi: "https://doi.org/10.6084/m9.figshare.31044715",
    crate_url: "https://crates.io/crates/rssn",
    funding: "https://opencollective.com/apich-organization/projects/rssn",
    funding_from: "$5",
    languages: &["Rust", "C", "C++"],
    platforms: &["Linux", "macOS", "Windows", "wgpu"],
    stats: &[
        ("1", "Entry point", "Session::compute"),
        ("6", "Output formats", "text · LaTeX · Typst · plots"),
        ("3", "Execution backends", "tape · Cranelift · GPU"),
        ("C/C++", "FFI", "rssn.h · rssn.hpp"),
    ],
    features: &[
        Feature {
            icon: "layers",
            title: "E-graph algebra",
            body: "Every operation is an identity transformation on a hash-consed DAG: differentiate, integrate, solve and simplify rewrite to an equal closed form.",
        },
        Feature {
            icon: "compass",
            title: "Dimensions in the terms",
            body: "Each scalar carries an inferred dimension, and operators declare how dimensions combine — SI and affine units, symbolic exponents, strict or advisory.",
        },
        Feature {
            icon: "chart",
            title: "Calculus and algebra",
            body: "Definite and improper integrals, Meijer G, Gröbner bases (Buchberger, F4, F5), CAD quantifier elimination, and ODE/PDE solution families.",
        },
        Feature {
            icon: "cpu",
            title: "Numeric kernels",
            body: "Adaptive quadrature, stiff ODE solvers, Krylov methods, Chebyshev and AAA approximation, FFT, sparse linear algebra and statistics.",
        },
        Feature {
            icon: "zap",
            title: "Compiled expressions",
            body: "Closed forms compile to an SSA tape with reverse gradients and interval enclosures, or to Cranelift machine code with the jit feature.",
        },
        Feature {
            icon: "globe",
            title: "Physics and simulation",
            body: "Rule sets for classical mechanics through QFT, plus FDM, FEM, FVM, BEM, CFD, molecular dynamics and spectral solvers.",
        },
    ],
};

static CARGO_SLIDE: ProjectContent = ProjectContent {
    id: Project::CargoSlide,
    tagline: "Code-driven presentations, rendered natively",
    summary: "A presentation engine that pairs Typst typesetting with a native Rust runtime, delivering decks as a standalone binary, a portable archive, or a WebAssembly bundle.",
    category: "Tooling",
    version: versions::CARGO_SLIDE,
    license: "AGPL-3.0-or-later",
    repo: "https://github.com/Apich-Organization/cargo-slide",
    docs: "https://docs.rs/cargo-slide",
    doi: "",
    crate_url: "https://crates.io/crates/cargo-slide",
    funding: "https://opencollective.com/apich-organization/projects/cargo-slide",
    funding_from: "$5",
    languages: &["Rust", "Typst"],
    platforms: &["Linux", "macOS", "Windows", "WASM"],
    stats: &[
        ("60", "Frames per second", "software rendering"),
        ("~14 MB", "Standalone binary", "no runtime dependencies"),
        ("~1 MB", ".slide archive", "LZMA2 compressed"),
        ("3", "Delivery formats", "binary · archive · WASM"),
    ],
    features: &[
        Feature {
            icon: "file",
            title: "Typst authoring",
            body: "Slides are Typst markup, compiled to vector graphics in under a second, with first-class mathematics and reusable macros.",
        },
        Feature {
            icon: "zap",
            title: "Native playback",
            body: "A tiny-skia renderer holds 60 FPS without a browser engine or Electron, in a single executable.",
        },
        Feature {
            icon: "terminal",
            title: "WYSIWYG editor",
            body: "Edit Typst inline on the rendered canvas, with an outline sidebar, insertion bar and debounced live recompilation.",
        },
        Feature {
            icon: "chart",
            title: "SQL charts",
            body: "Charts are queried, not drawn: bind a data source and the slide renders from the result set.",
        },
        Feature {
            icon: "users",
            title: "Speaker tools",
            body: "Notes separated from comments, word-count pacing, density analysis and a presenter view.",
        },
        Feature {
            icon: "globe",
            title: "Three deliverables",
            body: "A standalone binary, a compressed .slide archive, or a static WASM bundle with no inline JavaScript.",
        },
    ],
};

static VIRTUE: ProjectContent = ProjectContent {
    id: Project::Virtue,
    tagline: "A sinless derive macro helper",
    summary: "A small library for writing proc-macro derives declaratively: parse the item, generate an impl, and let the type system catch what you got wrong.",
    category: "Libraries",
    version: versions::VIRTUE,
    license: "MIT OR Apache-2.0",
    repo: "https://github.com/Apich-Organization/virtue",
    docs: "https://docs.rs/virtue-next",
    doi: "",
    crate_url: "https://crates.io/crates/virtue-next",
    funding: "https://opencollective.com/apich-organization",
    funding_from: "$5",
    languages: &["Rust"],
    platforms: &["proc-macro", "stable Rust"],
    stats: &[
        ("0", "Required dependencies", "proc-macro2 is optional"),
        ("{version}", "Current release", "semver-stable"),
        ("2024", "Rust edition", "modern toolchain"),
        ("declarative", "Generation style", "typed builder API"),
    ],
    features: &[
        Feature {
            icon: "package",
            title: "Zero required dependencies",
            body: "Compile-time cost is the whole point: the crate pulls nothing in unless you opt into proc-macro2.",
        },
        Feature {
            icon: "git",
            title: "Declarative generation",
            body: "A Generator builds impls, functions, arguments and bodies through a typed API instead of quote! templates.",
        },
        Feature {
            icon: "check",
            title: "Typesystem checking",
            body: "Generics, where-clauses, lifetimes and visibility are handled by the library, so generated code compiles or fails clearly.",
        },
        Feature {
            icon: "layers",
            title: "Attributes and bodies",
            body: "Parse struct and enum bodies, read helper attributes, and branch on shape without hand-writing a parser.",
        },
        Feature {
            icon: "file",
            title: "Readable errors",
            body: "Errors carry spans, so a bad derive points at the token that caused it rather than at the macro invocation.",
        },
        Feature {
            icon: "book",
            title: "Semver discipline",
            body: "Minor releases keep the API stable. The supported Rust version is listed in the crate metadata, not fixed on this page.",
        },
    ],
};

static AEVIA: ProjectContent = ProjectContent {
    id: Project::Aevia,
    tagline: "A functional language with dimensions in its type system",
    summary: "A functional language for scientific computing that rejects dimensionally inconsistent expressions before the program runs, backed by RSSN's symbolic and numerical engine.",
    category: "Language",
    version: versions::AEVIA,
    license: "Apache-2.0",
    repo: "https://github.com/Apich-Organization/aevia",
    docs: "https://aevia.apich.org/",
    doi: "",
    crate_url: "",
    funding: "https://opencollective.com/apich-organization/projects/aevia",
    funding_from: "$5",
    languages: &["Aevia", "Rust"],
    platforms: &["Linux", "macOS", "Windows"],
    stats: &[
        ("{version}", "Language release", "stable toolchain"),
        ("3", "Toolchain binaries", "aevia · aevia-lsp · aevia-pkg"),
        ("SI", "Units", "plus affine units"),
        ("Static", "Dimension checks", "before execution"),
    ],
    features: &[
        Feature {
            icon: "compass",
            title: "Dimensions in the type",
            body: "3 m + 2 s does not compile. Units are part of the type, so a dimensional error is a type error, not a runtime surprise.",
        },
        Feature {
            icon: "layers",
            title: "Functional core",
            body: "Expressions are values; evaluation is rewriting. The same expression can be inspected symbolically or evaluated numerically.",
        },
        Feature {
            icon: "zap",
            title: "RSSN underneath",
            body: "Symbolic transformations, numerical algorithms and physical-unit metadata come from RSSN, so the language and the library agree.",
        },
        Feature {
            icon: "terminal",
            title: "REPL and CLI",
            body: "aevia check, run and test, with an interactive prompt that answers :type for any expression.",
        },
        Feature {
            icon: "package",
            title: "Packages",
            body: "aevia.toml manifests, a lockfile, local dependencies, documentation and signed publishing through aevia-pkg.",
        },
        Feature {
            icon: "book",
            title: "Editor support",
            body: "aevia-lsp provides diagnostics, navigation and tasks, with a published book, standard-library reference and specification.",
        },
    ],
};

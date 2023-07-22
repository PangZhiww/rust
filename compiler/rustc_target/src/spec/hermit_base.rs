use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, TargetOptions, TlsModel};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "hermit".into(),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        linker: Some("rust-lld".into()),
        has_thread_local: true,
        panic_strategy: PanicStrategy::Abort,
        position_independent_executables: true,
        static_position_independent_executables: true,
        tls_model: TlsModel::InitialExec,
        ..Default::default()
    }
}

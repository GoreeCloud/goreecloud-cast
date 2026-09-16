#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportProfile {
    LocalDirect,
    LocalRelay,
    RemoteDirect,
    RemoteRelay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransportPolicy {
    pub allow_local_direct: bool,
    pub allow_local_relay: bool,
    pub allow_remote_direct: bool,
    pub allow_remote_relay: bool,
}

impl TransportPolicy {
    pub const LOCAL_ONLY: Self = Self {
        allow_local_direct: true,
        allow_local_relay: true,
        allow_remote_direct: false,
        allow_remote_relay: false,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathAvailability {
    pub local_direct: bool,
    pub local_relay: bool,
    pub remote_direct: bool,
    pub remote_relay: bool,
}

pub fn select_transport(
    policy: TransportPolicy,
    paths: PathAvailability,
) -> Option<TransportProfile> {
    [
        (
            policy.allow_local_direct && paths.local_direct,
            TransportProfile::LocalDirect,
        ),
        (
            policy.allow_local_relay && paths.local_relay,
            TransportProfile::LocalRelay,
        ),
        (
            policy.allow_remote_direct && paths.remote_direct,
            TransportProfile::RemoteDirect,
        ),
        (
            policy.allow_remote_relay && paths.remote_relay,
            TransportProfile::RemoteRelay,
        ),
    ]
    .into_iter()
    .find_map(|(allowed, profile)| allowed.then_some(profile))
}

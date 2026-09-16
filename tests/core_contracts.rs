use goreecloud_cast_core::{
    capability::{Capability, CapabilitySet, CastMode},
    discovery::{DeviceCategory, DiscoveryAdvertisement},
    handoff::{preflight, GateDecision, HandoffFailure, HandoffRequest, PlatformDecisions},
    identity::{ActorContext, DeviceId, TrustLevel},
    state_capsule::{PlatformPrivacyDecision, PrivacyClass, StateCapsule, StateEntry},
    transport::{PathAvailability, TransportPolicy, TransportProfile},
};

fn actor(authenticated: bool) -> ActorContext {
    ActorContext {
        device_id: DeviceId::parse("device:controller:0001").unwrap(),
        trust_level: TrustLevel::ApprovedDevice,
        authenticated,
    }
}

fn state() -> StateCapsule {
    let mut capsule = StateCapsule::new("com.goreecloud.video").unwrap();
    capsule
        .push(StateEntry {
            key: "position_ms".into(),
            value: 42_000_u64.to_be_bytes().to_vec(),
            privacy_class: PrivacyClass::SessionMetadata,
        })
        .unwrap();
    capsule
}

fn allowed_platform() -> PlatformDecisions {
    PlatformDecisions {
        identity: GateDecision::Allowed,
        privacy: GateDecision::Allowed,
        security: GateDecision::Allowed,
    }
}

#[test]
fn receiver_playback_is_preferred_over_mirroring() {
    let request = HandoffRequest {
        actor: actor(true),
        source_capabilities: CapabilitySet::new([
            Capability::ReceiverPlayback,
            Capability::DisplayMirror,
        ]),
        receiver_capabilities: CapabilitySet::new([
            Capability::ReceiverPlayback,
            Capability::DisplayMirror,
        ]),
        policy_capabilities: CapabilitySet::new([
            Capability::ReceiverPlayback,
            Capability::DisplayMirror,
        ]),
        state: state(),
        platform: allowed_platform(),
        privacy: PlatformPrivacyDecision {
            allowed: true,
            allow_sensitive: false,
        },
        transport_policy: TransportPolicy::LOCAL_ONLY,
        path_availability: PathAvailability {
            local_direct: true,
            local_relay: true,
            remote_direct: true,
            remote_relay: true,
        },
    };

    let plan = preflight(&request).unwrap();
    assert_eq!(plan.mode, CastMode::ReceiverPlayback);
    assert_eq!(plan.transport, TransportProfile::LocalDirect);
}

#[test]
fn platform_unknown_fails_closed() {
    let mut platform = allowed_platform();
    platform.privacy = GateDecision::Unknown;
    let request = HandoffRequest {
        actor: actor(true),
        source_capabilities: CapabilitySet::new([Capability::ReceiverPlayback]),
        receiver_capabilities: CapabilitySet::new([Capability::ReceiverPlayback]),
        policy_capabilities: CapabilitySet::new([Capability::ReceiverPlayback]),
        state: state(),
        platform,
        privacy: PlatformPrivacyDecision {
            allowed: true,
            allow_sensitive: false,
        },
        transport_policy: TransportPolicy::LOCAL_ONLY,
        path_availability: PathAvailability {
            local_direct: true,
            local_relay: false,
            remote_direct: false,
            remote_relay: false,
        },
    };
    assert_eq!(
        preflight(&request),
        Err(HandoffFailure::PlatformDecisionNotAllowed)
    );
}

#[test]
fn restricted_state_never_leaves_capsule() {
    let mut capsule = state();
    capsule
        .push(StateEntry {
            key: "credential".into(),
            value: b"must-not-transfer".to_vec(),
            privacy_class: PrivacyClass::Restricted,
        })
        .unwrap();
    let entries = capsule
        .transferable_entries(&PlatformPrivacyDecision {
            allowed: true,
            allow_sensitive: true,
        })
        .unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].key, "position_ms");
}

#[test]
fn discovery_advertisement_contains_only_bounded_public_shape() {
    let ad = DiscoveryAdvertisement::new(
        "rotating-id-0123456789",
        DeviceCategory::Display,
        0xA11CE,
        true,
    )
    .unwrap();
    assert_eq!(ad.category, DeviceCategory::Display);
    assert!(ad.requires_confirmation);
    assert!(!ad.rotating_discovery_id.contains("Living Room"));
}

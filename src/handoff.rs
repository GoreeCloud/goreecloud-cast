use crate::{
    capability::{CapabilitySet, CastMode},
    identity::{ActorContext, TrustLevel},
    state_capsule::{PlatformPrivacyDecision, StateCapsule},
    transport::{select_transport, PathAvailability, TransportPolicy, TransportProfile},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateDecision {
    Allowed,
    Denied,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformDecisions {
    pub identity: GateDecision,
    pub privacy: GateDecision,
    pub security: GateDecision,
}

impl PlatformDecisions {
    pub fn all_allowed(self) -> bool {
        [self.identity, self.privacy, self.security]
            .into_iter()
            .all(|decision| decision == GateDecision::Allowed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandoffRequest {
    pub actor: ActorContext,
    pub source_capabilities: CapabilitySet,
    pub receiver_capabilities: CapabilitySet,
    pub policy_capabilities: CapabilitySet,
    pub state: StateCapsule,
    pub platform: PlatformDecisions,
    pub privacy: PlatformPrivacyDecision,
    pub transport_policy: TransportPolicy,
    pub path_availability: PathAvailability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandoffPlan {
    pub mode: CastMode,
    pub transport: TransportProfile,
    pub transferred_state_entries: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandoffFailure {
    UnauthenticatedActor,
    PlatformDecisionNotAllowed,
    NoAuthorizedCastMode,
    PrivacyAuthorizationRequired,
    NoAuthorizedTransport,
}

pub fn preflight(request: &HandoffRequest) -> Result<HandoffPlan, HandoffFailure> {
    if !request.actor.authenticated && request.actor.trust_level != TrustLevel::Guest {
        return Err(HandoffFailure::UnauthenticatedActor);
    }
    if !request.platform.all_allowed() {
        return Err(HandoffFailure::PlatformDecisionNotAllowed);
    }

    let common = intersect(&request.source_capabilities, &request.receiver_capabilities);
    let mode = common
        .select_mode(&request.policy_capabilities)
        .ok_or(HandoffFailure::NoAuthorizedCastMode)?;

    let state = request
        .state
        .transferable_entries(&request.privacy)
        .map_err(|_| HandoffFailure::PrivacyAuthorizationRequired)?;

    let transport = select_transport(request.transport_policy, request.path_availability)
        .ok_or(HandoffFailure::NoAuthorizedTransport)?;

    Ok(HandoffPlan {
        mode,
        transport,
        transferred_state_entries: state.len(),
    })
}

fn intersect(left: &CapabilitySet, right: &CapabilitySet) -> CapabilitySet {
    use crate::capability::Capability;

    const ALL: &[Capability] = &[
        Capability::Audio,
        Capability::Video,
        Capability::Queue,
        Capability::Seek,
        Capability::MultipleControllers,
        Capability::Grouping,
        Capability::ReceiverPlayback,
        Capability::DirectMediaStream,
        Capability::ApplicationCast,
        Capability::DisplayMirror,
    ];

    CapabilitySet::new(
        ALL.iter()
            .copied()
            .filter(|capability| left.supports(*capability) && right.supports(*capability)),
    )
}

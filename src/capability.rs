#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CastMode {
    ReceiverPlayback,
    DirectMediaStream,
    ApplicationCast,
    DisplayMirror,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    Audio,
    Video,
    Queue,
    Seek,
    MultipleControllers,
    Grouping,
    ReceiverPlayback,
    DirectMediaStream,
    ApplicationCast,
    DisplayMirror,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CapabilitySet {
    items: Vec<Capability>,
}

impl CapabilitySet {
    pub fn new(items: impl IntoIterator<Item = Capability>) -> Self {
        let mut set = Self::default();
        for item in items {
            if !set.items.contains(&item) {
                set.items.push(item);
            }
        }
        set
    }

    pub fn supports(&self, capability: Capability) -> bool {
        self.items.contains(&capability)
    }

    pub fn select_mode(&self, allowed: &CapabilitySet) -> Option<CastMode> {
        const ORDER: &[(Capability, CastMode)] = &[
            (Capability::ReceiverPlayback, CastMode::ReceiverPlayback),
            (Capability::DirectMediaStream, CastMode::DirectMediaStream),
            (Capability::ApplicationCast, CastMode::ApplicationCast),
            (Capability::DisplayMirror, CastMode::DisplayMirror),
        ];

        ORDER.iter().find_map(|(capability, mode)| {
            (self.supports(*capability) && allowed.supports(*capability)).then_some(*mode)
        })
    }
}

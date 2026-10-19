use crate::tree::NodeId;

/// Which pipeline stages a node needs to re-run. A bit set with a tiny hand-rolled API.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Dirty(u8);

impl Dirty {
    pub const NONE: Dirty = Dirty(0);
    pub const LAYOUT: Dirty = Dirty(1);
    pub const PAINT: Dirty = Dirty(2);
    pub const SEMANTICS: Dirty = Dirty(4);
    pub const ALL: Dirty = Dirty(7);
    pub fn contains(self, o: Dirty) -> bool {
        self.0 & o.0 == o.0
    }
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub fn bits(self) -> u8 {
        self.0
    }
}
impl std::ops::BitOr for Dirty {
    type Output = Dirty;
    fn bitor(self, o: Dirty) -> Dirty {
        Dirty(self.0 | o.0)
    }
}
impl std::ops::BitOrAssign for Dirty {
    fn bitor_assign(&mut self, o: Dirty) {
        self.0 |= o.0;
    }
}

/// One entry of the scene's dirty log: a node and the flags newly raised on it since the last drain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirtyEntry {
    pub node: NodeId,
    pub flags: Dirty,
}

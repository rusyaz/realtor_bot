use teloxide::types::FileId;

/// Lead being filled in. Each form step sets one field.
#[derive(Clone, Default, Debug)]
pub struct Lead {
    pub owner: bool,
    pub region: String,
    pub address: String,
    pub floor: String,
    pub area: f32,
    /// 0 = studio.
    pub rooms: u8,
    pub repair: String,
    pub photos: Vec<FileId>,
}

/// Dialogue state = "which question are we waiting an answer for".
///
/// Flow:
/// MainMenu ─(Сдать квартиру)→ Owner → Region → Address → Floor → Area
///          → Rooms → Repair → Photos → Phone ─(lead sent)→ MainMenu
///
/// Every state after `Owner` carries the `Lead` collected so far.
/// New features: add a button to the main menu that leads into their own states.
#[derive(Clone, Default)]
pub enum State {
    /// Main menu, nothing in progress. Default for new users.
    #[default]
    MainMenu,
    /// Asked: are you the owner?
    Owner,
    /// Asked: Moscow or Moscow region?
    Region { lead: Lead },
    /// Asked: address.
    Address { lead: Lead },
    /// Asked: floor.
    Floor { lead: Lead },
    /// Asked: area in m².
    Area { lead: Lead },
    /// Asked: number of rooms.
    Rooms { lead: Lead },
    /// Asked: renovation type.
    Repair { lead: Lead },
    /// Collecting photos until "Done" (min `MIN_PHOTOS`).
    Photos { lead: Lead },
    /// Asked: phone. After this the lead goes to the realtor.
    Phone { lead: Lead },
}

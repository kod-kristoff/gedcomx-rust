#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NamePartType {
    Given,
    Surname,
}

/// Enumeration of standard name types.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NameType {
    /// Name given at birth.
    BirthName,

    /// Name used at the time of death.
    DeathName,

    /// Name accepted at marriage.
    MarriedName,

    /// "Also known as" name.
    AlsoKnownAs,

    /// Nickname.
    Nickname,

    /// Name given at adoption.
    AdoptiveName,

    /// A formal name, usually given to distinguish it from a name more commonly used.
    FormalName,

    /// A name given at a religious rite or ceremony.
    ReligiousName,
    // OTHER(String),
}

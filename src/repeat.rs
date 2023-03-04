#[derive(Debug, Copy, Clone)]
pub enum Optional {
    AtMost(usize),
    Any,
}

impl Optional {
    pub fn matched(self) -> Option<Self> {
        match self {
            Optional::AtMost(0) => None,
            Optional::AtMost(n) => Some(Optional::AtMost(n - 1)),
            Optional::Any => Some(Optional::Any),
        }
    }

    pub fn exhausted(&self) -> bool {
        match self {
            Optional::AtMost(0) => true,
            Optional::AtMost(_) => false,
            Optional::Any => false,
        }
    }

    pub fn needed(&self) -> bool {
        false
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Required {
    AtLeast(usize),
    Exactly(usize),
}

impl Required {
    pub fn matched(self) -> Option<Self> {
        match self {
            Required::Exactly(0) => None,
            Required::Exactly(n) => Some(Required::Exactly(n - 1)),
            Required::AtLeast(0) => Some(Required::AtLeast(0)),
            Required::AtLeast(n) => Some(Required::AtLeast(n - 1)),
        }
    }

    pub fn exhausted(&self) -> bool {
        match self {
            Required::Exactly(0) => true,
            Required::Exactly(_) => false,
            Required::AtLeast(0) => false,
            Required::AtLeast(_) => false,
        }
    }

    pub fn needed(&self) -> bool {
        match self {
            Required::Exactly(0) => false,
            Required::Exactly(_) => true,
            Required::AtLeast(0) => false,
            Required::AtLeast(_) => true,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Repeat {
    Optional(Optional),
    Required(Required),
}

impl Repeat {
    pub fn matched(self) -> Option<Self> {
        match self {
            Repeat::Optional(opt) => opt.matched().map(Repeat::Optional),
            Repeat::Required(req) => req.matched().map(Repeat::Required),
        }
    }

    pub fn exhausted(&self) -> bool {
        match self {
            Repeat::Optional(opt) => opt.exhausted(),
            Repeat::Required(req) => req.exhausted(),
        }
    }

    pub fn needed(&self) -> bool {
        match self {
            Repeat::Optional(opt) => opt.needed(),
            Repeat::Required(req) => req.needed(),
        }
    }
}

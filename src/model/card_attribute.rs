#[derive(Clone, Copy, Debug, PartialEq, Hash)]
#[non_exhaustive]
pub enum AttributeType {
    ColorIndicator,
    Loyalty,
    PowerToughness,
    BottomHalf,
    LifeModifier,
    HandModifier,
}

pub enum AttributeInstance {
    ColorIndicator(ColorIndicator),
    Loyalty(Loyalty),
    PowerToughness(PowerToughness),
    BottomHalf(BottomHalf),
    LifeModifier(LifeModifier),
    HandModifier(HandModifier),
}


pub struct ColorIndicator {
    //pub colors: HashSet<colors>,
}

pub struct Loyalty {
}

pub struct PowerToughness {
}

pub struct BottomHalf {
}

pub struct LifeModifier {
}

pub struct HandModifier {
}


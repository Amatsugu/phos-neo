use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
#[require(BackgroundColor)]
pub struct BaseColor(pub Color);

#[derive(Component, Debug, Reflect)]
#[require(BackgroundColor)]
pub struct HoverColor(pub Color);

#[derive(Component, Debug, Reflect)]
#[require(BackgroundColor)]
pub struct PressedColor(pub Color);

#[derive(Component, Debug, Reflect)]
#[require(TextColor)]
pub struct BaseTextColor(pub Color);

#[derive(Component, Debug, Reflect)]
#[require(TextColor)]
pub struct HoverTextColor(pub Color);

#[derive(Component, Debug, Reflect)]
#[require(TextColor)]
pub struct PressedTextColor(pub Color);

#[derive(Component, Debug, Reflect)]
#[require(BorderColor)]
pub struct BaseBorderColor(pub Color);

#[derive(Component, Debug, Reflect)]
#[require(BorderColor)]
pub struct HoverBorderColor(pub Color);

#[derive(Component, Debug, Reflect)]
#[require(BorderColor)]
pub struct PressedBorderColor(pub Color);

//! Controls the force-specific color accents for the in-combat UI.
//! (Blue for player units, red for enemies, etc.)
use unity::engine::Sprite;
use unity::engine::ui::Image;

// Classes
#[unity::class("App", "ForceTextureSetter")]
pub struct ForceTextureSetter {
    pub image: &'static Image,
    pub player_texture: &'static Sprite,
    pub enermy_texture: &'static Sprite,
    pub ally_texture: &'static Sprite,
    pub f4th_texture: &'static Sprite,
}
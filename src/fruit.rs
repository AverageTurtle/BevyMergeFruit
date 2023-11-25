use bevy::prelude::*;

#[derive(Copy, Clone)]
pub enum FruitTypes {
    None = 0,
    Blueberry = 1,
    Cherry = 2,
    Lime = 3,
    Banana = 4,
    Orange = 5,
    Apple = 6
}

impl From<i32> for FruitTypes {
    fn from(int: i32) -> Self {
        match int {
            0 => FruitTypes::None,
            1 => FruitTypes::Blueberry,
            2 => FruitTypes::Cherry,
            3 => FruitTypes::Lime,
            4 => FruitTypes::Banana,
            5 => FruitTypes::Orange,
            6 => FruitTypes::Apple,
            _  => FruitTypes::None
        }
    }
}

impl PartialEq for FruitTypes {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
pub fn get_random_fruit_type() -> FruitTypes {
    let numb = fastrand::i32(1..4);
    FruitTypes::from(numb)
}
pub struct FruitType {
    pub size: f32,
    //TODO this should be a asset handle but idk how do implement that yet :/
    pub texture: String,
    pub value: i32,
    pub next_fruit: Option<FruitTypes>
}

impl From<FruitTypes> for FruitType {
    fn from(fruit_type: FruitTypes) -> Self {
        match fruit_type {
            FruitTypes::Blueberry => Self{ size: 32., texture: "textures/blueberry.png".to_string(), value: 10,next_fruit: Some(FruitTypes::Cherry) },
            FruitTypes::Cherry => Self{ size: 48., texture: "textures/cherry.png".to_string(), value: 20, next_fruit: Some(FruitTypes::Lime) },
            FruitTypes::Lime => Self{ size: 72., texture: "textures/lime.png".to_string(), value: 40, next_fruit: Some(FruitTypes::Banana) },
            FruitTypes::Banana => Self{ size: 108., texture: "textures/banana.png".to_string(), value: 80, next_fruit: Some(FruitTypes::Orange) },
            FruitTypes::Orange => Self{ size: 162., texture: "textures/orange.png".to_string(), value: 160, next_fruit: Some(FruitTypes::Apple) },
            FruitTypes::Apple => Self{ size: 243., texture: "textures/apple.png".to_string(), value: 320, next_fruit: None },

            _ => Self{ size: 64., texture: "textures/debug_circle.png".to_string(), value: -100000, next_fruit: None },
        }
    }
}

#[derive(Component)]
pub struct Fruit {
    pub fruit_type: FruitTypes,
    pub create_time: f32
}

#[derive(Component)]
pub struct PreviewFruit;

#[derive(Component)]
pub struct NextFruitPreview;
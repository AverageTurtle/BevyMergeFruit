use bevy::prelude::*;

#[derive(Copy, Clone)]
pub enum FruitTypes {
    None = 0,
    Blueberry = 1,
    Cherry = 2,
    Strawberry = 3,
    Lime = 4,
    Banana = 5,
    Orange = 6,
    Apple = 7,
    Peach = 8,
    Coconut = 9,
    Cantaloupe = 10,
    Pineapple = 11,
    Watermelon = 12
}

impl From<i32> for FruitTypes {
    fn from(int: i32) -> Self {
        match int {
            0 => FruitTypes::None,
            1 => FruitTypes::Blueberry,
            2 => FruitTypes::Cherry,
            3 => FruitTypes::Strawberry,
            4 => FruitTypes::Lime,
            5 => FruitTypes::Banana,
            6 => FruitTypes::Orange,
            7 => FruitTypes::Apple,
            8 => FruitTypes::Peach,
            9 => FruitTypes::Coconut,
            10 => FruitTypes::Cantaloupe,
            11 => FruitTypes::Pineapple,
            12 => FruitTypes::Watermelon,
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
    let numb = fastrand::i32(1..7);
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
            FruitTypes::Blueberry => Self{ size: 36., texture: "textures/blueberry.png".to_string(), value: 2,next_fruit: Some(FruitTypes::Cherry) },
            FruitTypes::Cherry => Self{ size: 50., texture: "textures/cherry.png".to_string(), value: 4, next_fruit: Some(FruitTypes::Strawberry) },
            FruitTypes::Strawberry => Self{ size: 68., texture: "textures/strawberry.png".to_string(), value: 6, next_fruit: Some(FruitTypes::Lime) },
            FruitTypes::Lime => Self{ size: 86., texture: "textures/lime.png".to_string(), value: 8, next_fruit: Some(FruitTypes::Banana) },
            FruitTypes::Banana => Self{ size: 104., texture: "textures/banana.png".to_string(), value: 10, next_fruit: Some(FruitTypes::Orange) },
            FruitTypes::Orange => Self{ size: 122., texture: "textures/orange.png".to_string(), value: 12, next_fruit: Some(FruitTypes::Apple) },
            FruitTypes::Apple => Self{ size: 144., texture: "textures/apple.png".to_string(), value: 14, next_fruit:  Some(FruitTypes::Peach) },
            FruitTypes::Peach => Self{ size: 166., texture: "textures/peach.png".to_string(), value: 16, next_fruit: Some(FruitTypes::Coconut) },
            FruitTypes::Coconut => Self{ size: 188., texture: "textures/coconut.png".to_string(), value: 18, next_fruit: Some(FruitTypes::Cantaloupe) },
            FruitTypes::Cantaloupe => Self{ size: 212., texture: "textures/cantaloupe.png".to_string(), value: 20, next_fruit: Some(FruitTypes::Pineapple) },
            FruitTypes::Pineapple => Self{ size: 238., texture: "textures/pineapple.png".to_string(), value: 22, next_fruit: Some(FruitTypes::Watermelon) },
            FruitTypes::Watermelon => Self{ size: 276., texture: "textures/watermelon.png".to_string(), value: 100, next_fruit: None },

            _ => Self{ size: 64., texture: "textures/debug_circle.png".to_string(), value: -100000, next_fruit: None },
        }
    }
}
/* 
FruitTypes::Blueberry => Self{ size: 32., texture: "textures/blueberry.png".to_string(), value: 10,next_fruit: Some(FruitTypes::Cherry) },
FruitTypes::Cherry => Self{ size: 48., texture: "textures/cherry.png".to_string(), value: 20, next_fruit: Some(FruitTypes::Lime) },
FruitTypes::Lime => Self{ size: 54., texture: "textures/lime.png".to_string(), value: 40, next_fruit: Some(FruitTypes::Banana) },
FruitTypes::Banana => Self{ size: 108., texture: "textures/banana.png".to_string(), value: 80, next_fruit: Some(FruitTypes::Orange) },
FruitTypes::Orange => Self{ size: 162., texture: "textures/orange.png".to_string(), value: 160, next_fruit: Some(FruitTypes::Apple) },
FruitTypes::Apple => Self{ size: 243., texture: "textures/apple.png".to_string(), value: 320, next_fruit: None },
*/

#[derive(Component)]
pub struct Fruit {
    pub fruit_type: FruitTypes,
    pub create_time: f32,
    pub death_time: f32,
    pub size: f32,
    pub target_size : f32
}
#[derive(Component)]
pub struct PreviewFruit;

#[derive(Component)]
pub struct NextFruitPreview {
    pub fruit_type: FruitTypes
}
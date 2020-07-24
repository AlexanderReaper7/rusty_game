use std::convert::From;

pub struct Player
{
    pub score: u128,
    pub color: PlayerColors,
    pub icon: PlayerIcons,
}

#[repr(u8)]
pub enum PlayerIcons
{
    Cross,
    Circle,
    Square,
    Diamond
}

impl PlayerIcons
{
    pub fn count() -> u8 {4}
}

impl From<u8> for PlayerIcons
{
    fn from(item: u8) -> Self 
    {
        match item
        {
            0 => return Self::Cross,
            1 => return Self::Circle,
            2 => return Self::Square,
            3 => return Self::Diamond,
            _ => panic!()
        }
    }
}
#[repr(u8)]
pub enum PlayerColors
{
    RED, 
    MAROON, 
    YELLOW, 
    OLIVE, 
    LIME, 
    GREEN, 
    AQUA, 
    TEAL, 
    BLUE, 
    NAVY, 
    FUCHSIA, 
    PURPLE
}

impl PlayerColors
{
    pub fn count() -> u8 {12}
}

impl From<u8> for PlayerColors
{
    fn from(item: u8) -> Self 
    {
        match item
        {
            0 => return Self::RED,
            1 => return Self::MAROON,
            2 => return Self::YELLOW,
            3 => return Self::OLIVE,
            4 => return Self::LIME,
            5 => return Self::GREEN,
            6 => return Self::AQUA,
            7 => return Self::TEAL,
            8 => return Self::BLUE,
            9 => return Self::NAVY,
            10 => return Self::FUCHSIA,
            11 => return Self::PURPLE,
            _ => panic!()
        }
    }
}

impl Player
{
    pub fn new(color: PlayerColors, icon: PlayerIcons) -> Self
    {
        return Player{score: 0, color: color, icon: icon}
    }

    pub fn draw(pos_x: f32, pos_y: f32) 
    {

    }

    pub fn generate_all_combinations() -> Vec<Player>
    {
        let player_n = PlayerColors::count() as usize * PlayerIcons::count() as usize;
        let mut players : Vec<Player> = Vec::with_capacity(player_n);

        for x in 0..PlayerIcons::count()-1 {
            for y in 0..PlayerColors::count()-1 {
                players.push(Player::new(PlayerColors::from(y), PlayerIcons::from(x)))
            }
        } 

        return players
    }

    pub fn get_player_color_u8(color: u8) -> [f32;4]
    {
        match color
        {
            0 => return [1.0, 0.0, 0.0, 1.0],
            1 => return [0.5, 0.0, 0.0, 1.0], 
            2 => return [1.0, 1.0, 0.0, 1.0], 
            3 => return [0.5, 0.5, 0.0, 1.0], 
            4 => return [0.0, 1.0, 0.0, 1.0], 
            5 => return [0.0, 0.5, 0.0, 1.0], 
            6 => return [0.0, 1.0, 1.0, 1.0], 
            7 => return [0.0, 0.5, 0.5, 1.0], 
            8 => return [0.0, 0.0, 1.0, 1.0], 
            9 => return [0.0, 0.0, 0.5, 1.0], 
            10 => return [1.0, 0.0, 1.0, 1.0], 
            11 => return [0.5, 0.0, 0.0, 0.5],
            _ => panic!("PlayerColor out of bounds"),
        }
    }

    pub fn get_player_color(color: PlayerColors) -> [f32;4]
    {
        match color
        {
            PlayerColors::RED => return [1.0, 0.0, 0.0, 1.0],
            PlayerColors::MAROON => return [0.5, 0.0, 0.0, 1.0], 
            PlayerColors::YELLOW => return [1.0, 1.0, 0.0, 1.0], 
            PlayerColors::OLIVE => return [0.5, 0.5, 0.0, 1.0], 
            PlayerColors::LIME => return [0.0, 1.0, 0.0, 1.0], 
            PlayerColors::GREEN => return [0.0, 0.5, 0.0, 1.0], 
            PlayerColors::AQUA => return [0.0, 1.0, 1.0, 1.0], 
            PlayerColors::TEAL => return [0.0, 0.5, 0.5, 1.0], 
            PlayerColors::BLUE => return [0.0, 0.0, 1.0, 1.0], 
            PlayerColors::NAVY => return [0.0, 0.0, 0.5, 1.0], 
            PlayerColors::FUCHSIA => return [1.0, 0.0, 1.0, 1.0], 
            PlayerColors::PURPLE => return [0.5, 0.0, 0.0, 0.5],
            _ => panic!("PlayerColor out of bounds"),
        }
    }
    /*
    RED = [1.0, 0.0, 0.0, 1.0],
    MAROON = [0.5, 0.0, 0.0, 1.0], 
    YELLOW = [1.0, 1.0, 0.0, 1.0], 
    OLIVE = [0.5, 0.5, 0.0, 1.0], 
    LIME = [0.0, 1.0, 0.0, 1.0], 
    GREEN = [0.0, 0.5, 0.0, 1.0], 
    AQUA = [0.0, 1.0, 1.0, 1.0], 
    TEAL = [0.0, 0.5, 0.5, 1.0], 
    BLUE = [0.0, 0.0, 1.0, 1.0], 
    NAVY = [0.0, 0.0, 0.5, 1.0], 
    FUCHSIA = [1.0, 0.0, 1.0, 1.0], 
    PURPLE = [0.5, 0.0, 0.0, 0.5],
    */
}



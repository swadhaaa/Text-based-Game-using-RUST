use std::io;

struct Character
{
    name: String,
    health: i16,
    attack: i16,
    defense: i16,
}

impl Character
{
    fn temporary_buff(&mut self, buff_param: i16, attribute_param: i16) -> i16
    {
            buff_param * attribute_param
    }
}

fn main()
{
    let mut valnir_reaper = Character
    {
        name: "Valnir the Reaper".to_string(),
        attack: 27,
        defense: 21,
        health: 150,
    };

    let mut player_char = create_character();
    println!("Name: {}\nHealth: {}\nAttack: {}\nDefense: {}",
    player_char.name, player_char.health, player_char.attack, player_char.defense);

    battle(&mut player_char, &mut valnir_reaper);
}

fn create_character() -> Character
{
    let mut output_character = Character
    {
        name: "a_name".to_string(),
        attack: 0,
        defense: 0,
        health: 0,
    };

    let mut player_char = String::new();
    println!("Type in the name of your character that will do battle against the enemy!");
    io::stdin().read_line(&mut player_char).expect("Failed to get input.");
    player_char = player_char.trim().to_string();
    output_character.name = player_char.clone();

    loop
    {
        let mut remaining_points:i16 = 50;
        let temp_defense_points:i16 = 10;
        let temp_attack_points:i16 = 10;
        let temp_health_points:i16 = 50;

        println!(" abcd, remaining_points);
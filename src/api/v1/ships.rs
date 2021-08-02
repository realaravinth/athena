/*
 * Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{errors::ServiceResult, AppData};

pub const SHIPS: [&str; 262] = [
    "Abe",
    "Ageless Warrior",
    "Aggie",
    "Aggie on Horseback",
    "'A Gin Court",
    "America's Favorite Carrier",
    "Angry Cat",
    "Archdeacon",
    "Athabee",
    "Babe Lincoln",
    "Battle Cat",
    "Battle Schmoozer",
    "Battle Barge",
    "Battle Star",
    "Big Ben",
    "Big D",
    "The Big E",
    "The Big J",
    "Big John",
    "Big Lizzie",
    "Big Mamie",
    "The Big Nasty",
    "The Big Stick",
    "Big Gray Deuce",
    "Billy Ruffian",
    "The Black Dragon",
    "The Blue Ghost",
    "Bonnie",
    "Bonnie Dick",
    "Buckin' Bronco",
    "Building (hull number)",
    "Building 575",
    "Building 597",
    "Building 21",
    "Building 11",
    "Bulldog of the Navy",
    "The Big Risk",
    "Bag Lady",
    "Berzerkly",
    "Broke",
    "Brand X",
    "C-ville",
    "Cannabis",
    "Can o’ Pus",
    "Can Opener",
    "Century One",
    "Charlie Love Five Five",
    "Cheer Up Ship",
    "Christmas Anthem",
    "Chuck Bucket",
    "Chuckie V",
    "Cocoa Boat",
    "Connie",
    "The Count",
    "Curious",
    "Cellblock 70",
    "Despair Ship Remorse",
    "Decrepit",
    "Dirty Cush",
    "Dirty Two-Thirty",
    "Dirty V",
    "Dreado",
    "Dull Ass",
    "Dickover",
    "Dickey B",
    "Dry I",
    "Dippity Do",
    "Eggshells",
    "Enterprison",
    "Evil I",
    "Exploder",
    "The Fightingest Ship in the RCN",
    "The Fighting G",
    "The Fighting J",
    "The Fighting I",
    "The Fighting Lady",
    "The Fighting Sausage",
    "Fighting Mary",
    "Firestal",
    "The Five Mile Sniper",
    "Flatiron",
    "Forrest Fire",
    "Fraser Blade",
    "Freddy",
    "Fleet Starship",
    "The Furry Wet Mound",
    "Galloping Ghost of the Java Coast",
    "George's Legs",
    "Germanclown",
    "Ghetto",
    "Gipper",
    "Gin Palace",
    "The Gold Eagle",
    "The Golden Devil",
    "The Golden Guad",
    "Le Grand Hotel",
    "The Grand Old Lady",
    "The Gray Ghost",
    "The Gray Lady",
    "The Gray Ghost",
    "Grey Ghost",
    "Greenpig",
    "GW",
    "The Ham",
    "Happy Valley",
    "Hairy D",
    "He-Cat",
    "HMAS Can Opener",
    "HMS Me",
    "HMS Refit",
    "HMS Repair",
    "Hiroshima",
    "Holiday Express",
    "Holiday Inn",
    "Horny Maru",
    "HST",
    "Hairy Ass",
    "Hymi G",
    "Happy Harry",
    "Hotel Yamato",
    "Ike",
    "Ikeatraz",
    "Indy",
    "Iron Duck",
    "Fighting I",
    "Iron Woman",
    "Jimmy K",
    "Johnny Reb",
    "Kaarnavene",
    "Kami-ha-ha",
    "King of Tomahawks",
    "Shitty Kitty",
    "Knockwood",
    "Lady Lex",
    "Lady Lou",
    "Long Delayed",
    "The Lord's Own",
    "Lost and Confused",
    "Lucky A",
    "Lucky E",
    "Lucky 26",
    "Lusty",
    "Lucky Number 7",
    "Maggie",
    "The Mighty Hood",
    "The Mighty I",
    "Mighty O",
    "Mighty T",
    "Mighty Y",
    "Mighty Mo",
    "Mighty Moo",
    "Mobile Chernobyl",
    "Moskvitch",
    "Motel 6",
    "Midway Magic",
    "Nasty Asty",
    "Nasty Nick",
    "Nelly",
    "Niffy Jane",
    "NO Boat",
    "Northo",
    "O'Broken",
    "The O-Boat",
    "Old Bones",
    "Old Falling Apart",
    "Old Formy",
    "Old Hoodoo",
    "Old Mary",
    "The Old Grey Ghost of the Borneo Coast",
    "Old Ironsides",
    "The Old Lady",
    "Old Lady of the Sea",
    "Old Salt",
    "One-Eye",
    "Orjalaiva Kurjala",
    "Outrageous",
    "The Oki Boat",
    "Pepper Pot",
    "Pierwolf",
    "The Pool",
    "Proud Pete",
    "Prune Barge",
    "Puffington",
    "Puuhamaa",
    "Sweet Pea",
    "Pig Boat",
    "Pubic Mound",
    "Quarter-mile Island",
    "Queer Barge",
    "Queerfish",
    "Quiet Warrior",
    "The Red-Eye",
    "Refit",
    "Repair",
    "Rezzo",
    "Rosie",
    "Rough Rider",
    "Rodnol",
    "Ruosteensilmä",
    "Rusty-guts",
    "Rusty W",
    "Sara",
    "Saggy Pants",
    "Sally Rand",
    "Seapuppy",
    "Shall Not Perish",
    "Shiny Sheff",
    "The Shitty Dick",
    "Shitty Kitty",
    "Showboat",
    "Sleek and Deadly Duck",
    "Slack Jack",
    "Smiley",
    "The Smoke",
    "Sodak",
    "USS Spring-a-leak",
    "Spurious",
    "Starship Vinson",
    "Steel Cat",
    "Stinkin Lincoln",
    "Suckin' 60 From Dixie",
    "Surunmaa",
    "Swanky Franky",
    "Swayback Maru",
    "Stressex",
    "Steamin' Deuce",
    "Slimey Lymie",
    "Saint Pauline",
    "The Stain",
    "Special K",
    "T2",
    "Tea Boat",
    "Tea Chest",
    "Teacup",
    "Teddy Ruxpin",
    "Tico",
    "Tiddly Quid",
    "Tin Duck",
    "Three-Quarter Mile Island",
    "The Toothless Terror",
    "Toasted O",
    "Traffie",
    "Trawler Mauler",
    "Tullibeast",
    "The Tartan Terror",
    "Tottenham",
    "Tuska class",
    "Tortanic",
    "Top Gun",
    "Uproarious",
    "VDQ",
    "Von Stupid",
    "Vince",
    "Vinny",
    "Weavy",
    "Winter Pig",
    "Wisky",
    "Wicked Witch of the West",
    "The Y",
    "Zippo",
    "Zoo (The) – USS Kalamazoo",
    "Navire canadien de Sa Majesté",
];

struct Exists {
    exists: Option<bool>,
}

pub async fn get_name(data: &AppData) -> ServiceResult<&'static str> {
    for ship in SHIPS.iter() {
        if !victim_exists(&data, &ship).await? {
            return Ok(ship);
        }
    }
    panic!();
}

pub async fn victim_exists(data: &AppData, victim: &str) -> ServiceResult<bool> {
    let exists = sqlx::query_as!(
        Exists,
        "SELECT EXISTS (SELECT 1 from cic_victims WHERE name = $1);",
        victim
    )
    .fetch_one(&data.db)
    .await?;

    if exists.exists.is_some() && *exists.exists.as_ref().unwrap() {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn get_name_works() {
        let data = crate::data::Data::new().await;
        let data = actix_web::web::Data::new(data);
        let name = get_name(&data).await.unwrap();
        assert!(SHIPS.iter().any(|ship| ship == &name));
    }
}

use crate::battle::model::{CharaConfig, LuckyLevel};
use crate::battle::utils::{dir_files, dir_files_noasync};
use anyhow::Context;
use chrono::prelude::NaiveDateTime;
use once_cell::sync::Lazy;
use rand::prelude::IteratorRandom;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub enum PlayMode {
    Simple,
    Raid,
    Story { id: String },
}

impl PlayMode {
    pub fn try_from_value(value: &str) -> anyhow::Result<Self> {
        match value {
            "Simple" => Ok(Self::Simple),
            "Raid" => Ok(Self::Raid),
            _ => Err(anyhow::anyhow!(format!("No match {}", value))),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, PartialOrd)]
pub struct BattleData {
    uuid: Uuid,
    player_data: CharaConfig,
    enemy_data: CharaConfig,
    play_mode: PlayMode,
    elapesd_turns: u32,
    start_time: NaiveDateTime,
    is_running: bool,
}

impl CharaConfig {
    pub async fn chara_new(name_arg: &str) -> anyhow::Result<Self> {
        static REIMU_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"(?i)(^(h|H)+akurei)?(r|R)eimu$").unwrap());
        static SAKUYA_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"(?i)(^(i|I)+zayoi)?(s|S)akuya$").unwrap());
        static MARISA_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"(?i)(^(k|K)+irisame)?(m|M)arisa$").unwrap());

        if let Some(_) = REIMU_REGEX.find(&name_arg) {
            let chara_datas = dir_files("chara").await.unwrap();
            let reimu_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "博麗霊夢")
                .context("Not found")?;
            Ok(reimu_data)
        } else if let Some(_) = SAKUYA_REGEX.find(&name_arg) {
            let chara_datas = dir_files("chara").await.unwrap();
            let sakuya_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "十六夜咲夜")
                .context("Not found")?;
            Ok(sakuya_data)
        } else if let Some(_) = MARISA_REGEX.find(&name_arg) {
            let chara_datas = dir_files("chara").await.unwrap();
            let marisa_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "霧雨魔理沙")
                .context("Not found")?;
            Ok(marisa_data)
        } else {
            Err(anyhow::anyhow!("No match regex {:?}", &name_arg))
        }
    }

    pub fn chara_new_noasync(name_arg: &str) -> anyhow::Result<Self> {
        static REIMU_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"(?i)(^(h|H)+akurei)?(r|R)eimu$").unwrap());
        static SAKUYA_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"(?i)(^(i|I)+zayoi)?(s|S)akuya$").unwrap());
        static MARISA_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"(?i)(^(k|K)+irisame)?(m|M)arisa$").unwrap());

        if let Some(_) = REIMU_REGEX.find(&name_arg) {
            let chara_datas = dir_files_noasync("chara").unwrap();
            let reimu_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "博麗霊夢")
                .context("Not found")?;
            Ok(reimu_data)
        } else if let Some(_) = SAKUYA_REGEX.find(&name_arg) {
            let chara_datas = dir_files_noasync("chara").unwrap();
            let sakuya_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "十六夜咲夜")
                .context("Not found")?;
            Ok(sakuya_data)
        } else if let Some(_) = MARISA_REGEX.find(&name_arg) {
            let chara_datas = dir_files_noasync("chara").unwrap();
            let marisa_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "霧雨魔理沙")
                .context("Not found")?;
            Ok(marisa_data)
        } else {
            Err(anyhow::anyhow!("No match regex {:?}", &name_arg))
        }
    }
}

impl LuckyLevel {
    pub fn lucky_number(&self) -> f32 {
        match self {
            LuckyLevel::LuckyOne => 1.1,
            LuckyLevel::LuckyTwo => 1.3,
            LuckyLevel::LuckyThree => 1.5,
        }
    }
}

/// Amount of exp earned in battle
/// Exp = 18 + (Enemy level*2 - my level) * {enemy appear}th boss (* lucky_number)
///
pub fn math_exp(
    enemy_level: u32,
    player_level: u32,
    enemy_appear: u8,
    lucky_level: Option<LuckyLevel>,
) -> f32 {
    let mut base_exp = (18 + (enemy_level * 2 - player_level) * enemy_appear as u32) as f32;
    if let Some(l) = lucky_level {
        base_exp *= l.lucky_number()
    }

    base_exp as f32
}

impl PlayMode {
    /// get story id
    pub fn story_id(&self) -> Option<&str> {
        match self {
            Self::Simple => None,
            Self::Raid => None,
            Self::Story { id: a } => Some(a),
        }
    }
}

impl BattleData {
    pub fn new(
        uuid: Uuid,
        player_data: CharaConfig,
        enemy_data: CharaConfig,
        play_mode: PlayMode,
        start_time: NaiveDateTime,
        elapesd_turns: u32,
    ) -> Self {
        Self {
            uuid,
            player_data,
            enemy_data,
            play_mode,
            start_time,
            elapesd_turns,
            is_running: false,
        }
    }

    /// Advance the elapsed turn
    pub fn add_turn(&mut self) -> &mut Self {
        self.elapesd_turns += 1;
        self
    }
    fn _turn<'a>(player: &'a CharaConfig, enemy: &'a CharaConfig) -> Vec<&'a CharaConfig> {
        let mut vec: Vec<&CharaConfig> = Vec::new();
        if player.charabase.speed >= enemy.charabase.speed {
            vec.push(player);
            vec.push(enemy);
        } else {
            vec.push(enemy);
            vec.push(player);
        };
        vec
    }
    /// Functions tat manipulate turnsh
    /// When this function is called, the turn advances by 1.
    /// ([add_turn](add_turn))
    pub fn turn(&self) -> &CharaConfig {
        let turn_info = Self::_turn(&self.player_data, &self.enemy_data);
        // If it exceeds the length of `vec`, it will return to the first element of the array due
        // to the` Cycle` type.
        // Therefore, it is unlikely that it will be `None` unless the contents of` Vec` are empty.
        let info = turn_info
            .into_iter()
            .cycle()
            .nth(self.elapesd_turns as usize)
            .unwrap();
        info
    }

    /// get player data
    pub fn player(&self) -> &CharaConfig {
        &self.player_data
    }

    /// get enemy data
    pub fn enemy(&self) -> &CharaConfig {
        &self.enemy_data
    }

    /// get uuid
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// player -> enemy
    fn calculate_player_damage(&mut self) -> &Self {
        let mut rng = rand::thread_rng();
        let player_attack = self.player_data.attack.iter().choose(&mut rng).unwrap();
        let to_enemy_damage = self.enemy_data.charabase.hp - player_attack.damage as i16;
        self.enemy_data.charabase.hp = to_enemy_damage;
        self
    }

    /// enemy -> player
    fn calculate_enemy_damage(&mut self) -> &Self {
        let mut rng = rand::thread_rng();
        let enemy_attack = self.enemy_data.attack.iter().choose(&mut rng).unwrap();
        let to_player_damage = self.player_data.charabase.hp - enemy_attack.damage as i16;
        self.player_data.charabase.hp = to_player_damage;
        self
    }

    pub fn elapesd_turns(&self) -> u32 {
        self.elapesd_turns
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn start_running(&mut self) -> &mut Self {
        self.is_running = true;
        self
    }

    pub(crate) async fn result_battle<'a>(&'a mut self) -> &'a Self {
        let turn = self.turn();
        let damage = if turn == &self.player_data {
            self.calculate_player_damage()
        } else {
            self.calculate_enemy_damage()
        };
        damage
    }
}

/// Battle State
pub enum BattleState {
    BattleContinue,
    PlayerDown,
    EnemyDown,
}

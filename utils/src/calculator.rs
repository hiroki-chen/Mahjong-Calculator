extern crate enum_map;
extern crate objects;

use objects::base_models;

pub fn round_up_score(score: u32) -> u32 {
  let ans = (((score / 100) as f32).ceil() * 100 as f32) as u32;
  ans
}

pub fn calculate_score_with_fu(fu: u8, han: u8) -> u32 {
  let ans = 4 * (fu * u8::pow(2, han as u32 + 2)) as u32;
  ans
}

pub fn get_yaku(
  hai_in_hand: &Vec<base_models::Menzu>,
  atama: &Option<base_models::Atama>,
  dora: &Vec<base_models::Hai>,
  monzen: bool,
  riichi: bool,
  seat: u8,
  round: u8,
) -> Vec<base_models::Yaku> {
  let mut yaku: Vec<base_models::Yaku> = Vec::new();
  todo!();
  yaku
}

/// Calculates the final score.
pub fn get_final_result(
  hai_in_hand: &Vec<base_models::Menzu>,
  atama: &Option<base_models::Atama>,
  dora: &Vec<base_models::Hai>,
  go_out_type: base_models::GoOutType,
  yaku_map: enum_map::EnumMap<base_models::Yaku, u8>,
  monzen: bool,
  riichi: bool,
  is_dealer: bool,
  seat: u8,
  round: u8,
  honba_num: u8,
  riichi_stick_num: u8,
) -> u32 {
  let mut ans: u32 = 0;

  // First we need to check if the summed han's exceed Mankan; if so, there is no need to calculate Fu.
  let yaku = get_yaku(hai_in_hand, atama, dora, monzen, riichi, seat, round);
  let mut han_num: u8 = 0;
  yaku.into_iter().for_each(|x| han_num += yaku_map[x]);

  if han_num >= base_models::MANKAN {
    // There is no need to calculate the number of Fu.
    match han_num {
      // 満貫
      5 => ans = 8000,
      // 跳満
      6..=7 => ans = 12000,
      // 倍満
      8..=10 => ans = 16000,
      // 三倍満
      11..=12 => ans = 24000,
      // 役満以上や数え役満など
      13..=25 => ans = 32000,
      26..=38 => ans = 64000,
      39..=51 => ans = 96000,
      52..=64 => ans = 128000,
      65..=77 => ans = 160000,
      78..=90 => ans = 192000,
      _ => {}
    }

    if is_dealer {
      ans = (1.5 * ans as f32) as u32;
    }
  } else {
    // Need to calculate the number of Fu.
  }

  ans = round_up_score(ans);
  ans += honba_num as u32 * 300 + riichi_stick_num as u32 * 1000;
  ans
}

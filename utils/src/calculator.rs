extern crate enum_map;
extern crate objects;

use std::collections::HashMap;

use crate::checker;
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
  machi: &(Vec<base_models::Hai>, base_models::Hai),
  atama: &Option<base_models::Atama>,
  extra_information: &mut Vec<base_models::Yaku>,
  seat: u8,
  round: u8,
) -> Vec<base_models::Yaku> {
  let mut yaku: Vec<base_models::Yaku> = Vec::new();

  // Initialize some basic counters for Yaku's like Sananko.
  let mut ankou_number: u8 = 0;
  let mut minkou_number: u8 = 0;
  let mut shunzu_number: u8 = 0;
  let mut ankan_number: u8 = 0;
  let mut minkan_number: u8 = 0;

  // Initialize a map for temporarily storing the menzu type for extra Yaku lookup
  // This may include Yaku's like Sanshakutonshun.
  // This map ignores the type of each menzu, if menzu is not Zuhai, 
  // and only focuses on their contents.
  //
  // Typeref: [is_zuhai, hai_1, hai_2, hai_3] -> count
  let mut menzu_map: HashMap<(bool, u8, u8, u8), u8> = HashMap::new(); 

  // First we copy all the extra Yakus into the final vector.
  yaku.append(extra_information);

  // Iterate over the vector.
  for menzu in hai_in_hand {
    match menzu.menzu_type {
      base_models::MenzuType::Ankou => ankou_number += 1,
      base_models::MenzuType::Ankan => ankan_number += 1,
      base_models::MenzuType::Minkou => minkou_number += 1,
      base_models::MenzuType::Minkan => minkan_number += 1,
      base_models::MenzuType::Shunzu => shunzu_number += 1,
    }

    match menzu.content[0] {
      // Handle with Zuhai.
      base_models::Hai::Z(val) => match val {
        5 => yaku.push(base_models::Yaku::Haku),
        6 => yaku.push(base_models::Yaku::Hatsu),
        7 => yaku.push(base_models::Yaku::Chun),
        _ => {
          if val == seat {
            yaku.push(base_models::Yaku::Monfon);
          } else if val == round {
            yaku.push(base_models::Yaku::Chanfon);
          }
        }
      },

      // Handle with Non-zuhai type.
      _ => {}
    };
  }

  // Check suanko / sukanzu.
  if ankou_number == base_models::SUUANKOU || ankan_number == base_models::SUUANKOU {
    yaku.clear();

    // Tanki.
    if machi.0.len() == 1 {
      yaku.push(base_models::Yaku::Suuankoutanki);
    } else {
      yaku.push(base_models::Yaku::Suuankou);
    }
  } else if ankan_number + minkan_number == base_models::SUUKANZU {
    yaku.clear();
    yaku.push(base_models::Yaku::Suukanzu);
  }

  if ankou_number == base_models::SANANKOU {
    yaku.push(base_models::Yaku::Sanankou);
  } else if ankan_number + minkan_number == base_models::SANKANZU {
    yaku.push(base_models::Yaku::Sankanzu);
  }

  yaku
}

pub fn get_fu(
  hai_in_hand: &Vec<base_models::Menzu>,
  machi: &(Vec<base_models::Hai>, base_models::Hai),
  atama: &Option<base_models::Atama>,
  go_out_type: base_models::GoOutType,
  is_chitoi: bool,
  monzen: bool,
) -> u8 {
  if is_chitoi {
    base_models::CHIITOI
  } else {
    let mut final_fu: u8 = base_models::BASE;

    if let (base_models::GoOutType::Ron(_), _) = (go_out_type, monzen) {
      final_fu += base_models::MONZENKAFU;
    }

    for menzu in hai_in_hand {
      let is_yaokyu: bool = checker::check_if_is_yaokyu(&menzu.content[0]);
      match menzu.menzu_type {
        base_models::MenzuType::Minkou => {
          final_fu += if is_yaokyu { 4 } else { 2 };
        }
        base_models::MenzuType::Ankou => {
          final_fu += if is_yaokyu { 8 } else { 4 };
        }
        base_models::MenzuType::Ankan => {
          final_fu += if is_yaokyu { 32 } else { 16 };
        }
        base_models::MenzuType::Minkan => {
          final_fu += if is_yaokyu { 16 } else { 8 };
        }
        _ => {}
      };
    }

    // Handle the atama.

    final_fu
  }
}

/// Calculates the final score.
pub fn get_final_result(
  hai_in_hand: &Vec<base_models::Menzu>,
  machi: &(Vec<base_models::Hai>, base_models::Hai),
  atama: &Option<base_models::Atama>,
  dora: &Vec<base_models::Hai>,
  go_out_type: base_models::GoOutType,
  yaku_map: enum_map::EnumMap<base_models::Yaku, u8>,
  extra_information: &mut Vec<base_models::Yaku>,
  is_dealer: bool,
  is_chitoi: bool,
  monzen: bool,
  seat: u8,
  round: u8,
  honba_num: u8,
  riichi_stick_num: u8,
) -> u32 {
  assert_eq!(dora.len() <= 127, true);

  let mut ans: u32 = 0;
  // First we need to check if the summed han's exceed Mankan; if so, there is no need to calculate Fu.
  let yaku = get_yaku(hai_in_hand, machi, atama, extra_information, seat, round);
  let mut han_num: u8 = dora.len() as u8;
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
  } else {
    // Need to calculate the number of Fu.
    let fu: u8 = get_fu(hai_in_hand, machi, atama, go_out_type, is_chitoi, monzen);
    ans = calculate_score_with_fu(fu, han_num);
  }

  if is_dealer {
    ans = (1.5 * ans as f32) as u32;
  }

  ans = round_up_score(ans);

  // Do some clean-up.
  ans += honba_num as u32 * 300 + riichi_stick_num as u32 * 1000;
  ans
}

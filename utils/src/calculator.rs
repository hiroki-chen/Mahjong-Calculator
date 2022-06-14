use objects::base_models;
extern crate enum_map;

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
) -> u32 {
  let mut ans: u32 = 0;

  // First we need to check if the summed han's exceed Mankan; if so, there is no need to calculate Fu.
  let yaku = get_yaku(hai_in_hand, atama, dora, monzen, riichi, seat, round);
  let mut han_num: u8 = 0;
  yaku.into_iter().for_each(|x| han_num += yaku_map[x]);

  ans
}

use objects::base_models;

pub fn check_if_is_shunzu(hai_in_hand: &Vec<base_models::Hai>) -> bool {
  let first = &hai_in_hand[0];

  // Zihai cannot consist a Shunzu.
  if let base_models::Hai::Z(_) = first {
    return false;
  }

  hai_in_hand
    .windows(2)
    .all(|item| base_models::is_incremental(&item[0], &item[1]))
}

pub fn check_if_is_same_type(hai_in_hand: &Vec<base_models::Hai>) -> bool {
  match hai_in_hand
    .into_iter()
    .all(|item| base_models::hai_type_eq(item, &hai_in_hand[0]))
    .then(|| &hai_in_hand[0])
  {
    Some(_) => true,
    None => false,
  }
}

/// This function checks whether the current three hai's consist a Menzu.
/// Menzu can be in two forms: Kozu (Anko, Minko, Ankan, Minkan) or Shunzu.
/// Before calling this function, make sure that hai_in_hand is sorted by
/// Mahjong's rule.
pub fn check_if_is_menzu(hai_in_hand: &Vec<base_models::Hai>) -> bool {
  // If the number of the menzu is incorrect or the type of the hai's does not match,
  // there is no need to perform further check, and we directly return a false.
  if hai_in_hand.len() != 3 && hai_in_hand.len() != 4 || !check_if_is_same_type(hai_in_hand) {
    println!(
      "The number of hai's is not correct! Expect 3, got {}",
      hai_in_hand.len()
    );

    false
  } else if hai_in_hand.len() == 3 {
    // Case 1: If the current three hai's are Anko / Minko.
    let is_kozu: bool = hai_in_hand[0] == hai_in_hand[1] && hai_in_hand[1] == hai_in_hand[2];
    let is_shunzu: bool = check_if_is_shunzu(hai_in_hand);

    if is_kozu || is_shunzu {
      return true;
    }

    false
  } else {
    if hai_in_hand[0] != hai_in_hand[1]
      || hai_in_hand[1] != hai_in_hand[2]
      || hai_in_hand[2] != hai_in_hand[3]
    {
      false
    } else {
      true
    }
  }
}

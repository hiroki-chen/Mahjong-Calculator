use objects::base_models;
use utils::checker;

fn main() {
  let yaku_map = base_models::init_yaku_han_lookup();
  println!("{:?}", yaku_map);
  
  let sozu: base_models::Hai = base_models::Hai::M(3);
  let manzu: base_models::Hai = base_models::Hai::M(4);

  assert_ne!(sozu, manzu);

  let ankan = vec![sozu; 4];
  let shunzu = vec![sozu.clone(), manzu, base_models::Hai::M(5)];

  println!("Is a menzu? {}", checker::check_if_is_menzu(&ankan));
  println!("Is a menzu? {}", checker::check_if_is_menzu(&shunzu));

  println!("Hello, world!");
}

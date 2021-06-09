fn main() {
  let t1 = vec![1, u32::MAX-1];
  print_option_u32(sum_u32(&t1));

  let t2 = vec![1, u32::MAX];
  print_option_u32(sum_u32(&t2));
}

fn print_option_u32(v: Option<u32>) {
  match v {
    Some(v) => println!("sum: {}", v),
    None => println!("Value overflow!")
  };
}

fn sum_u32(v: &[u32]) -> Option<u32> {
  let mut total: u32 = 0;
  for i in v {
    total = match total.checked_add(*i) {
      Some(t) => t,
      None => return None,
    }
  }
  Some(total)
}

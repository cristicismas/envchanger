macro_rules! equit {
  () => ($crate::eprint!("\n"));
  ($($arg:tt)*) => ({
    use std::process::exit;
    eprintln!($($arg)*);
    exit(1);
  })
}

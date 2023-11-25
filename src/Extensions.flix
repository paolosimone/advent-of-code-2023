mod Option {
  pub def unwrap(option: Option[t]): t  = match option {
    case Some(value) => value
    case _ => bug!("Option was expected to have a value")
  }
}

mod Result {
  pub def unwrap(result: Result[e, t]): t with ToString[e]  = match result {
    case Ok(value) => value
    case Err(message) => bug!("${message}")
  }
}

mod Time.Duration {
  use Time.Duration.Duration

  pub def toSeconds(duration: Time.Duration): Float64 = 
    let Duration(nanoseconds) = duration;
    Int64.toFloat64(nanoseconds) / 1_000_000_000.0
}

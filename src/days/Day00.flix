
mod Advent.Days.Day00 {
  use Advent.Day;

  def load(_input: String): String = "input"

  def firstChallenge(_input: String): String = "first"

  def secondChallenge(_input: String): String = "second"

  pub def new(): Day[String] = {
    load = load,
    firstChallenge = firstChallenge,
    secondChallenge = secondChallenge
  }
}
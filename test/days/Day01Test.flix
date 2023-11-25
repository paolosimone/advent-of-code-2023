mod Advent.Days.Day01Test {
  use Advent.Days.Day01;

  @Test
  def testFirstChallenge(): Bool =
    let day = Day01.new();
    let input = day.load("");
    let actual = day.firstChallenge(input);
    Assert.eq("first", actual)
    
  @Test
  def testSecondChallenge(): Bool =
    let day = Day01.new();
    let input = day.load("");
    let actual = day.secondChallenge(input);
    Assert.eq("second", actual)
}
mod Advent.Days.Day00Test {
  use Advent.Days.Day00;

  @Test
  def testFirstChallenge(): Bool =
    let day = Day00.new();
    let input = day.load("");
    let actual = day.firstChallenge(input);
    Assert.eq("first", actual)
    
  @Test
  def testSecondChallenge(): Bool =
    let day = Day00.new();
    let input = day.load("");
    let actual = day.secondChallenge(input);
    Assert.eq("second", actual)
}
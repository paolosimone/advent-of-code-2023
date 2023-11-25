mod Advent {
  pub type alias DayResult = {
    day = Int32,
    loadElapsed = Time.Duration,
    firstResult = String,
    firstElapsed = Time.Duration,
    secondResult = String,
    secondElapsed = Time.Duration
  }

  pub def runDay(number: Int32): Result[String, DayResult] \ IO = 
    forM (
      day <- getDay(number);
      input <- readDayInput(number);
      (state, loadElapsed) <- timed(() -> day.load(input));
      (firstResult, firstElapsed) <- timed(() -> day.firstChallenge(state));
      (secondResult, secondElapsed) <- timed(() -> day.secondChallenge(state))
    ) 
      yield {
        day = number,
        loadElapsed = loadElapsed,
        firstResult = firstResult,
        firstElapsed = firstElapsed,
        secondResult = secondResult,
        secondElapsed = secondElapsed
      }

  pub def runAllDays(): Result[String, Vector[DayResult]] \ IO =
    Vector.range(1, 25)
    |> Vector.takeWhile(number -> number |> getDay |> Result.isOk)
    |> Vector.map(runDay)
    |> Vector.sequence

  def getDay(number: Int32): Result[String, Advent.Day[##java.lang.Object]] =
    let days = Vector#{
        unchecked_cast(Advent.Days.Day00.new() as Advent.Day[##java.lang.Object])
    };

    days
    |> Vector.nth(number)
    |> Option.toOk("Day not found: ${number}")


  def readDayInput(number: Int32): Result[String, String] \ IO = 
    "input/Day${String.padLeft(2, '0', "${number}")}.txt"
    |> Files.read
    |> Result.mapErr(match IOError.Generic(message) -> message)


  def timed(f: Unit -> t \ ef): Result[String, (t, Time.Duration)] \ {ef, IO} =
    let start = Time.Epoch.microseconds();
    let result = f();
    let stop = Time.Epoch.microseconds();
    let elapsed = (stop - start) 
      |> Int64.tryToInt32
      |> Option.unwrap
      |> Time.Duration.fromMicroSeconds
    ;
    Ok(result, elapsed)

}
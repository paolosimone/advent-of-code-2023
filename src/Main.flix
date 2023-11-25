def main(): Unit \ IO =
    let banner = "
   ___ _ _            __            _     _           _ 
  / __\\ (_)_  __   /\\ \\ \\__ ___   _(_) __| | __ _  __| |
 / _\\ | | \\ \\/ /  /  \\/ / _` \\ \\ / / |/ _` |/ _` |/ _` |
/ /   | | |>  <  / /\\  / (_| |\\ V /| | (_| | (_| | (_| |
\\/    |_|_/_/\\_\\ \\_\\ \\/ \\__,_| \\_/ |_|\\__,_|\\__,_|\\__,_|
                                                        
";
  println(banner);

  let results = runDays();
  let report = Report.render(results);
  println(report)

def runDays(): Vector[Advent.DayResult] \ IO =
  match dayArg() {
    case None => 
      Advent.runAllDays() |> Result.unwrap

    case Some(number) =>
      Vector#{number |> Advent.runDay |> Result.unwrap}
  }

def dayArg(): Option[Int32] =
  Environment.getArgs()
  |> List.head
  |> Option.flatMap(Int32.fromString)

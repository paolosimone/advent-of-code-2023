mod Report {
  use Advent.DayResult;

  pub def render(results: t[DayResult]): String \ IO with Iterable[t] =
    import new de.vandermeer.asciitable.AsciiTable(): ##de.vandermeer.asciitable.AsciiTable \ IO as newAsciiTable;
    import de.vandermeer.asciitable.AsciiTable.addRule(): Unit \ IO;
    import de.vandermeer.asciitable.AsciiTable.addRow(##java.util.Collection): ##de.vandermeer.asciitable.AT_Row \ IO;
    import de.vandermeer.asciitable.AsciiTable.render(): String \ IO;

    let table = newAsciiTable();

    // HEADER
    let header = Vector#{
      "Day",
      "LoadElapsed",
      "FirstResult",
      "FirstElapsed",
      "SecondResult",
      "SecondElapsed"
    };
    addRule(table);
    addRow(table, checked_cast(Adaptor.toArrayList(header)));
    addRule(table);

    // ROWS
    foreach (result <- results) {
      let row = Vector#{
        "${result.day}", 
        renderDuration(result.loadElapsed), 
        result.firstResult, 
        renderDuration(result.firstElapsed), 
        result.secondResult, 
        renderDuration(result.secondElapsed)
      };

      addRow(table, checked_cast(Adaptor.toArrayList(row)));
      addRule(table)
    };

    render(table)

  def renderDuration(duration: Time.Duration): String = 
    import static java.lang.String.format(String, Array[##java.lang.Object, Static]): String \ {};
    let seconds = Time.Duration.toSeconds(duration);
    region rc {
      let params = Array#{Float64.valueOf(seconds)} @ rc;
      format("%f s",  unchecked_cast(params as Array[##java.lang.Object, Static]))
    }
}

// The main entry point.
def main(): Unit \ IO =
    import new de.vandermeer.asciitable.AsciiTable(): ##de.vandermeer.asciitable.AsciiTable \ IO as newAsciiTable;
    import de.vandermeer.asciitable.AsciiTable.addRule(): Unit \ IO;
    import de.vandermeer.asciitable.AsciiTable.addRow(##java.util.Collection): ##de.vandermeer.asciitable.AT_Row \ IO;
    import de.vandermeer.asciitable.AsciiTable.render(): String \ IO;
    let table = newAsciiTable();
    addRule(table);
    addRow(table, checked_cast(Adaptor.toArrayList("Hello" :: "World" :: Nil)));
    addRule(table);
    println(render(table))

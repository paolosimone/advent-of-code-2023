mod Advent {
    pub type alias Day[t] = {
        load = String -> t,
        firstChallenge = t -> String,
        secondChallenge = t -> String
    }
}
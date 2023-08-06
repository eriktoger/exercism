module CarsAssemble

let successRate (speed: int) : float =
    match speed with
    | 0 -> 1
    | s when s < 5 -> 1
    | s when s < 9 -> 0.90
    | 9 -> 0.80
    | 10 -> 0.77
    | _ -> failwith "The dial only goes to 10"

let productionRatePerHour (speed: int) : float = 221.0 * float speed * successRate speed

let workingItemsPerMinute (speed: int) : int = int (productionRatePerHour speed) / 60

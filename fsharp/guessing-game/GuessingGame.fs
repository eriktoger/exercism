module GuessingGame

let reply (guess: int): string =
    match guess with
    | 42 -> "Correct"
    | 41 -> "So close"
    | 43 -> "So close"
    | guess when guess >43 ->"Too high"
    | _ -> "Too low"

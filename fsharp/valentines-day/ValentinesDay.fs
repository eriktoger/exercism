module ValentinesDay

type Approval =
    | Yes
    | No
    | Maybe

type Cuisine =
    | Korean
    | Turkish

type Genre =
    | Crime
    | Horror
    | Romance
    | Thriller

type Activity =
    | BoardGame
    | Chill
    | Movie of Genre
    | Restaurant of Cuisine
    | Walk of int

let rateActivity (activity: Activity) : Approval =
    match activity with
    | Restaurant Korean -> Yes
    | Restaurant Turkish -> Maybe
    | Walk i when i < 3 -> Yes
    | Walk i when i < 5 -> Maybe
    | Movie Romance -> Yes
    | _ -> No

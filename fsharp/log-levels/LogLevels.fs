module LogLevels

let (|Prefix|_|) (p: string) (s: string) =
    if s.StartsWith(p) then
        Some(s.Substring(p.Length).Trim())
    else
        None

let message (logLine: string) : string =
    match logLine with
    | Prefix "[ERROR]: " logMsg -> logMsg
    | Prefix "[WARNING]: " logMsg -> logMsg
    | Prefix "[INFO]: " logMsg -> logMsg
    | _ -> ""


let logLevel (logLine: string) : string =
    match logLine with
    | Prefix "[ERROR]: " _ -> "error"
    | Prefix "[WARNING]: " _ -> "warning"
    | Prefix "[INFO]: " _ -> "info"
    | _ -> ""

let reformat (logLine: string) : string =
    match logLine with
    | Prefix "[ERROR]: " logMsg -> logMsg + " (error)"
    | Prefix "[WARNING]: " logMsg -> logMsg + " (warning)"
    | Prefix "[INFO]: " logMsg -> logMsg + " (info)"
    | _ -> ""

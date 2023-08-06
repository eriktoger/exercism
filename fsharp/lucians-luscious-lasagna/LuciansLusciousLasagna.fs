module LuciansLusciousLasagna

let expectedMinutesInOven = 40

let remainingMinutesInOven timePassed = expectedMinutesInOven - timePassed

let preparationTimeInMinutes layers = layers * 2

let elapsedTimeInMinutes layers timePassed =
    preparationTimeInMinutes layers + timePassed

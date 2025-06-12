fn get_plant_name(plant_char: char) -> &'static str {
    match plant_char {
        'R' => "radishes",
        'G' => "grass",
        'C' => "clover",
        'V' => "violets",
        _ => "",
    }
}

fn get_student_index(student: &str) -> usize {
    (student.as_bytes()[0] - b'A') as usize
}
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let rows = diagram.lines();
    let student_index = get_student_index(student);

    rows.flat_map(|row| {
        let offset = student_index * 2;
        let plants = &row[offset..offset + 2];
        plants.chars().map(get_plant_name)
    })
    .collect::<Vec<&'static str>>()
}

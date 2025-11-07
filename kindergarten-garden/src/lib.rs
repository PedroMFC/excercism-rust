pub fn plant_match(plant: char) -> &'static str {
    match plant {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => "",
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = ["Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"];
    let mut result = Vec::new();

    let student_index = students.iter().position(|&s| s == student).unwrap();
    let student_plants = diagram.lines().map(|line| line.chars().skip(student_index * 2).take(2).collect::<String>()).collect::<Vec<String>>();
    
    for plant in student_plants {
        plant.chars().for_each(|c| result.push(plant_match(c)));
    }
    
    result
}

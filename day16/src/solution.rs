use std::collections::HashMap;

pub fn part1(fields: &HashMap<String, ((i32, i32), (i32, i32))>, tickets: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for ticket in tickets {
        for value in ticket {
            let mut valid = false;
            for ((min1, max1), (min2, max2)) in fields.values() {
                if (value > min1 && value < max1) || (value > min2 && value < max2) {
                    valid = true;
                    break;
                }
            }
            if !valid {
                sum += value;
            }
        }
    }
    sum
}

pub fn part2(
    fields: &HashMap<String, ((i32, i32), (i32, i32))>,
    my_ticket: &Vec<i32>,
    tickets: &Vec<Vec<i32>>,
) -> i64 {
    let mut field_columns: Vec<Vec<i32>> = Vec::new();
    let mut valid_tickets = tickets
        .iter()
        .filter(|ticket| ticket_valid(&ticket, fields));
    for value in valid_tickets.next().unwrap() {
        field_columns.push(vec![*value]);
    }
    for ticket in valid_tickets {
        for (column, value) in ticket.iter().enumerate() {
            field_columns[column].push(*value);
        }
    }
    let mut potential_field_names: Vec<Vec<String>> = Vec::new();
    for _ in 0..fields.len() {
        potential_field_names.push(Vec::new());
    }

    for (index, column) in field_columns.iter().enumerate() {
        for (name, ((min1, max1), (min2, max2))) in fields.iter() {
            if column
                .iter()
                .all(|el| in_either_range(*el, *min1, *max1, *min2, *max2))
            {
                potential_field_names[index].push(String::from(name));
            }
        }
    }

    let mut potential_field_names = potential_field_names
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, Vec<String>)>>();
    potential_field_names.sort_by_key(|(_, names)| names.len());
    let mut product = 1;
    let name_mapping = assign_names(&potential_field_names);
    for (index, name) in name_mapping {
        if name.starts_with("departure") {
            product *= my_ticket[index] as i64;
        }
    }
    product
}

fn ticket_valid(ticket: &Vec<i32>, fields: &HashMap<String, ((i32, i32), (i32, i32))>) -> bool {
    for value in ticket {
        let mut valid = false;
        for ((min1, max1), (min2, max2)) in fields.values() {
            if in_either_range(*value, *min1, *max1, *min2, *max2) {
                valid = true;
                break;
            }
        }
        if !valid {
            return false;
        }
    }
    true
}

fn in_either_range(value: i32, min1: i32, max1: i32, min2: i32, max2: i32) -> bool {
    (value >= min1 && value <= max1) || (value >= min2 && value <= max2)
}

// This was split out as a recursive csp solver thing but since this is a trivial case
// where the options are [[1], [1,2], [1,2,3]...] we can just loop and remove used options.
// This doesn't really need to be a separate function anymore
fn assign_names(potential_field_names: &[(usize, Vec<String>)]) -> Vec<(usize, String)> {
    let mut used: Vec<String> = Vec::new();
    let mut assigned: Vec<(usize, String)> = Vec::new();
    for (index, names) in potential_field_names {
        let unused_name = names
            .iter()
            .filter(|name| !used.iter().any(|used_name| used_name == *name))
            .map(|s| &**s)
            .next()
            .unwrap()
            .to_string();
        assigned.push((*index, unused_name.clone()));
        used.push(unused_name);
    }
    assigned
}

mod test {
    #[test]
    fn remove_invalid_tickets() {
        use std::collections::HashMap;
        let mut fields: HashMap<String, ((i32, i32), (i32, i32))> = HashMap::new();
        fields.insert(String::from("class"), ((1, 3), (5, 7)));
        fields.insert(String::from("row"), ((6, 11), (33, 44)));
        fields.insert(String::from("seat"), ((13, 40), (45, 50)));
        let tickets: Vec<Vec<i32>> = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];
        let valid_tickets: Vec<Vec<i32>> = tickets
            .into_iter()
            .filter(|ticket| super::ticket_valid(&ticket, &fields))
            .collect();
        assert_eq!(valid_tickets.len(), 1);
        assert_eq!(valid_tickets, vec![vec![7, 3, 47]]);
    }
}

pub mod mall;

pub use mall::floor::*;
pub use mall::{floor::store::employee::Employee, guard::Guard, *};

pub fn biggest_store(mall: Mall) -> store::Store {
    mall.floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .max_by_key(|store| store.square_meters)
        .unwrap()
        .clone()
}

pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let mut employees: Vec<&Employee> = mall
        .floors
        .iter()
        .flat_map(|floor| floor.stores.iter().flat_map(|store| store.employees.iter()))
        .collect();

    employees.sort_by(|a, b| b.salary.partial_cmp(&a.salary).unwrap());

    let highest_paid_salary = employees.first().map_or(0.0, |e| e.salary);
    let result: Vec<Employee> = employees
        .into_iter()
        .filter(|e| e.salary == highest_paid_salary)
        .cloned()
        .collect();
    result
        .iter()
        .map(|elem| Employee {
            salary: quatre_chiffres_apres_virgule(elem.salary),
            ..elem.clone()
        })
        .collect()
}

pub fn nbr_of_employees(mall: Mall) -> usize {
    mall.floors
        .iter()
        .flat_map(|floor| floor.stores.iter().flat_map(|store| store.employees.iter()))
        .count()
        + mall.guards.len()
}

pub fn check_for_securities(mall: &mut Mall, new_guards: Vec<Guard>) {
    let total_floor_size: u64 = mall.floors.iter().map(|f| f.size_limit).sum();

    let required_guards = (total_floor_size / 200) as usize - mall.guards.len();
    for i in 0..required_guards {
        mall.hire_guard(new_guards[i].clone())
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.iter_mut() {
        for store in floor.stores.iter_mut() {
            for employee in store.employees.iter_mut() {
                let (entry_hour, exit_hour) = employee.working_hours;
                if exit_hour - entry_hour > 10 {
                    employee.salary *= 1.1;
                } else {
                    employee.salary *= 0.9;
                }
                employee.salary = quatre_chiffres_apres_virgule(employee.salary);
            }
        }
    }
}
fn quatre_chiffres_apres_virgule(f: f64) -> f64 {
    format!("{:.4}", f).parse::<f64>().unwrap_or(0.0)
}

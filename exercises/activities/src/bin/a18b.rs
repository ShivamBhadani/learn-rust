// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this



enum EmployeeType {
    Maintenance,
    _Marketing,
    _Manager,
    _Supervisor,
    _Kitchen,
    _Assembly,
}

struct Employee {
    employee_type: EmployeeType,
    _employed: bool,
}

fn access_building(employee: &Employee) -> Result<(), String> {
    match employee.employee_type {
        EmployeeType::Maintenance => Ok(()),
        EmployeeType::_Marketing => Ok(()),
        EmployeeType::_Manager => Ok(()),
        _ => Err(String::from("Employee cannot access building")),
    }
}

fn main() {
    let employee = Employee {
        employee_type: EmployeeType::Maintenance,
        _employed: true,
    };
    match access_building(&employee) {
        Ok(()) => println!("Employee may access building"),
        Err(err) => println!("Employee may not access building: {}", err),
    }
}

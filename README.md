# Woodworking Workshop Management System

## Overview

The **Woodworking Workshop Management System** is a Rust-based application for managing a woodworking workshop's operations. It leverages the Internet Computer (IC) framework to provide stable storage and highly efficient management of resources, operations, and customer interactions.

### Key Features
- Manage workshop details, including location and equipment.
- Track projects and their associated tasks.
- Manage customer details and project orders.
- Record material inventory, usage, and procurement.
- Track employee details, roles, and time logs.
- Calculate costs, profits, and generate revenue reports.
- Comprehensive query system for all entities.
- Robust error handling and data validation.

---

## Technology Stack
- **Rust**: Language used for development.
- **IC SDK**: Framework for interacting with the Internet Computer.
- **Serde**: Serialization and deserialization of data.
- **Candid**: Interface description language for the IC.

---


## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown targetz
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

### Installation
1. Clone this repository:
   ```bash
   git clone https://github.com/codewithnaomi/woodworking-workshop.git
   cd woodworking-workshop
   ```
2. Start the Internet Computer locally:
   ```bash
   dfx start --background
   ```
3. Deploy the canister:
   ```bash
   dfx deploy
   ```

---

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Usage

### Core Functionalities

#### Workshop Management
- **Create Workshop**: Add workshop details like name, location, and equipment.
- **Query Workshop**: Retrieve workshop details.

#### Project Management
- **Create Project**: Record a new project.
- **Assign Tasks**: Allocate tasks to employees.
- **Track Progress**: Monitor the status and completion rate of projects.

#### Customer Management
- **Add Customer**: Add new customer details.
- **Query Customers**: Retrieve customer data.
- **Link Projects**: Associate projects with customers.

#### Inventory Management
- **Add Materials**: Record material procurement.
- **Track Usage**: Monitor inventory consumption for projects.
- **Generate Reports**: Calculate total costs and available inventory.

#### Employee Management
- **Add Employee**: Record employee details.
- **Assign Roles**: Define roles like carpenter, manager, etc.
- **Log Time**: Track employee hours for specific projects.

#### Revenue Management
- **Calculate Costs**: Compute material, labor, and overhead costs.
- **Generate Revenue Reports**: View total profits and costs.
- **Analyze Project Profitability**: Evaluate the profit margin of each project.

---

## Code Structure

- **`main.rs`**: Contains all logic for managing the workshop, projects, employees, inventory, and financials.
- **Entities**:
  - `Workshop`: Defines the workshop structure.
  - `Project`: Represents projects and associated tasks.
  - `Customer`: Holds customer information.
  - `Material`: Tracks materials and inventory usage.
  - `Employee`: Records employee data and time logs.
- **Storage**:
  - Uses `StableBTreeMap` for persistent data storage.

---

## API Endpoints

### Workshop
- `create_workshop`: Adds a new workshop.
- `get_workshop`: Retrieves workshop details.

### Project
- `create_project`: Creates a new project.
- `assign_task`: Assigns tasks to employees.
- `track_project_progress`: Updates project status.

### Customer
- `add_customer`: Adds customer information.
- `query_customer`: Fetches customer details.

### Inventory
- `add_material`: Records material procurement.
- `track_material_usage`: Tracks inventory usage for a project.

### Employee
- `add_employee`: Adds a new employee.
- `log_time`: Tracks employee hours worked on tasks.

### Revenue
- `calculate_total_cost`: Computes total project costs.
- `generate_revenue_report`: Generates financial reports for the workshop.

---

## Error Handling
The system includes robust error handling to manage invalid payloads, missing entities, and other exceptions. Errors are returned as:
- `Success(String)`: Operation completed successfully.
- `Error(String)`: Generic error message.
- `NotFound(String)`: Entity not found.
- `InvalidPayload(String)`: Input data is invalid.

---

## Contributing

Contributions are welcome! To contribute:
1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Submit a pull request with detailed information about your changes.

---
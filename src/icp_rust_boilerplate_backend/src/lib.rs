/// Woodworking Workshop Management System
#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// Type aliases for memory
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Workshop struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Workshop {
    id: u64,
    name: String,
    location: String,
    owner: String,
    contact: String,
    email: String,
    created_at: u64,
}

// Project struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Project {
    id: u64,
    workshop_id: u64,
    name: String,
    description: String,
    deadline: u64,
    cost_estimate: f64,
    status: String, // "ongoing", "completed", "canceled"
}

// Employee struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Employee {
    id: u64,
    workshop_id: u64,
    name: String,
    role: String,
    hourly_rate: f64,
    is_active: bool,
}

// Expense struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Expense {
    id: u64,
    workshop_id: u64,
    date: u64,
    category: String,
    amount: f64,
    description: String,
}

// Inventory struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Inventory {
    id: u64,
    workshop_id: u64,
    item_name: String,
    quantity: u64,
    unit_price: f64,
    restock_date: u64,
}

// Payload structs
#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateWorkshopPayload {
    name: String,
    location: String,
    owner: String,
    contact: String,
    email: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct CreateProjectPayload {
    workshop_id: u64,
    name: String,
    description: String,
    deadline: u64,
    cost_estimate: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct AddEmployeePayload {
    workshop_id: u64,
    name: String,
    role: String,
    hourly_rate: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct RecordExpensePayload {
    workshop_id: u64,
    category: String,
    amount: f64,
    description: String,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct UpdateInventoryPayload {
    workshop_id: u64,
    item_name: String,
    quantity: u64,
    unit_price: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

// Implementing Storable for Workshop
impl Storable for Workshop {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Workshop {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Project
impl Storable for Project {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Project {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Employee
impl Storable for Employee {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Employee {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Expense
impl Storable for Expense {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Expense {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Implementing Storable for Inventory
impl Storable for Inventory {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Inventory {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// Memory management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static WORKSHOPS: RefCell<StableBTreeMap<u64, Workshop, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        ));

    static PROJECTS: RefCell<StableBTreeMap<u64, Project, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        ));

    static EMPLOYEES: RefCell<StableBTreeMap<u64, Employee, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
        ));

    static EXPENSES: RefCell<StableBTreeMap<u64, Expense, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
        ));

    static INVENTORIES: RefCell<StableBTreeMap<u64, Inventory, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
        ));
}

// Functions

fn is_email_valid(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

#[ic_cdk::update]
fn create_workshop(payload: CreateWorkshopPayload) -> Result<Workshop, Message> {
    if payload.name.trim().is_empty()
        || payload.contact.trim().is_empty()
        || payload.email.trim().is_empty()
    {
        return Err(Message::InvalidPayload("Required fields are missing.".to_string()));
    }

    if !is_email_valid(&payload.email) {
        return Err(Message::InvalidPayload("Invalid email address.".to_string()));
    }

    let workshop_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("ID counter increment failed");

    let workshop = Workshop {
        id: workshop_id,
        name: payload.name,
        location: payload.location,
        owner: payload.owner,
        contact: payload.contact,
        email: payload.email,
        created_at: time(),
    };

    WORKSHOPS.with(|workshops| {
        workshops.borrow_mut().insert(workshop_id, workshop.clone());
    });

    Ok(workshop)
}

#[ic_cdk::update]
fn delete_workshop(workshop_id: u64) -> Result<Message, Message> {
    let removed = WORKSHOPS.with(|workshops| workshops.borrow_mut().remove(&workshop_id));

    match removed {
        Some(_) => Ok(Message::Success("Workshop deleted successfully.".to_string())),
        None => Err(Message::NotFound("Workshop not found.".to_string())),
    }
}

#[ic_cdk::update]
fn list_workshops() -> Result<Vec<Workshop>, Message> {
    let workshops = WORKSHOPS.with(|map| {
        map.borrow()
            .iter()
            .map(|(_, workshop)| workshop.clone())
            .collect::<Vec<_>>()
    });

    Ok(workshops)
}

#[ic_cdk::query]
fn get_workshop_by_id(workshop_id: u64) -> Result<Workshop, Message> {
    let workshop = WORKSHOPS.with(|workshops| workshops.borrow().get(&workshop_id));

    match workshop {
        Some(w) => Ok(w),
        None => Err(Message::NotFound("Workshop not found.".to_string())),
    }
}

// Create Project
#[ic_cdk::update]
fn create_project(payload: CreateProjectPayload) -> Result<Project, Message> {
    if payload.name.is_empty() || payload.description.is_empty() || payload.cost_estimate <= 0.0 {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let workshop_exists = WORKSHOPS.with(|workshops| workshops.borrow().contains_key(&payload.workshop_id));
    if !workshop_exists {
        return Err(Message::NotFound("Workshop not found".to_string()));
    }

    let project_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let project = Project {
        id: project_id,
        workshop_id: payload.workshop_id,
        name: payload.name,
        description: payload.description,
        deadline: payload.deadline,
        cost_estimate: payload.cost_estimate,
        status: "ongoing".to_string(),
    };

    PROJECTS.with(|projects| {
        projects.borrow_mut().insert(project_id, project.clone());
    });

    Ok(project)
}

// Add Employee
#[ic_cdk::update]
fn add_employee(payload: AddEmployeePayload) -> Result<Employee, Message> {
    if payload.name.is_empty() || payload.role.is_empty() || payload.hourly_rate <= 0.0 {
        return Err(Message::InvalidPayload("Missing required fields".to_string()));
    }

    let workshop_exists = WORKSHOPS.with(|workshops| workshops.borrow().contains_key(&payload.workshop_id));
    if !workshop_exists {
        return Err(Message::NotFound("Workshop not found".to_string()));
    }

    let employee_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let employee = Employee {
        id: employee_id,
        workshop_id: payload.workshop_id,
        name: payload.name,
        role: payload.role,
        hourly_rate: payload.hourly_rate,
        is_active: true,
    };

    EMPLOYEES.with(|employees| {
        employees.borrow_mut().insert(employee_id, employee.clone());
    });

    Ok(employee)
}

// Record Expense
#[ic_cdk::update]
fn record_expense(payload: RecordExpensePayload) -> Result<Expense, Message> {
    if payload.amount <= 0.0 {
        return Err(Message::InvalidPayload("Invalid expense amount".to_string()));
    }

    let workshop_exists = WORKSHOPS.with(|workshops| workshops.borrow().contains_key(&payload.workshop_id));
    if !workshop_exists {
        return Err(Message::NotFound("Workshop not found".to_string()));
    }

    let expense_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let expense = Expense {
        id: expense_id,
        workshop_id: payload.workshop_id,
        date: time(),
        category: payload.category,
        amount: payload.amount,
        description: payload.description,
    };

    EXPENSES.with(|expenses| {
        expenses.borrow_mut().insert(expense_id, expense.clone());
    });

    Ok(expense)
}

#[ic_cdk::update]
fn update_workshop_details(workshop_id: u64, updated_name: Option<String>, updated_location: Option<String>) -> Result<Message, Message> {
    WORKSHOPS.with(|workshops| {
        let mut map = workshops.borrow_mut();
        if let Some(mut workshop) = map.remove(&workshop_id) {
            if let Some(name) = updated_name {
                if !name.trim().is_empty() {
                    workshop.name = name;
                }
            }
            if let Some(location) = updated_location {
                if !location.trim().is_empty() {
                    workshop.location = location;
                }
            }
            map.insert(workshop_id, workshop);
            return Ok(Message::Success("Workshop details updated successfully.".to_string()));
        }
        Err(Message::NotFound("Workshop not found.".to_string()))
    })
}

#[ic_cdk::query]
fn count_workshops() -> usize {
    WORKSHOPS.with(|workshops| workshops.borrow().len().try_into().unwrap())
}

// Update Inventory
#[ic_cdk::update]
fn update_inventory(payload: UpdateInventoryPayload) -> Result<Inventory, Message> {
    if payload.quantity == 0 || payload.unit_price <= 0.0 {
        return Err(Message::InvalidPayload("Invalid inventory data".to_string()));
    }

    let workshop_exists = WORKSHOPS.with(|workshops| workshops.borrow().contains_key(&payload.workshop_id));
    if !workshop_exists {
        return Err(Message::NotFound("Workshop not found".to_string()));
    }

    let inventory_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Counter increment failed");

    let inventory = Inventory {
        id: inventory_id,
        workshop_id: payload.workshop_id,
        item_name: payload.item_name,
        quantity: payload.quantity,
        unit_price: payload.unit_price,
        restock_date: time(),
    };

    INVENTORIES.with(|inventories| {
        inventories.borrow_mut().insert(inventory_id, inventory.clone());
    });

    Ok(inventory)
}

// Calculate Total Expenses
#[ic_cdk::query]
fn calculate_total_expenses(workshop_id: u64) -> Result<f64, Message> {
    let workshop_exists = WORKSHOPS.with(|workshops| workshops.borrow().contains_key(&workshop_id));
    if !workshop_exists {
        return Err(Message::NotFound("Workshop not found".to_string()));
    }

    let total_expenses: f64 = EXPENSES.with(|expenses| {
        expenses
            .borrow()
            .iter()
            .filter(|(_, expense)| expense.workshop_id == workshop_id)
            .map(|(_, expense)| expense.amount)
            .sum()
    });

    Ok(total_expenses)
}

// Calculate Inventory Value
#[ic_cdk::query]
fn calculate_inventory_value(workshop_id: u64) -> Result<f64, Message> {
    let workshop_exists = WORKSHOPS.with(|workshops| workshops.borrow().contains_key(&workshop_id));
    if !workshop_exists {
        return Err(Message::NotFound("Workshop not found".to_string()));
    }

    let total_value: f64 = INVENTORIES.with(|inventories| {
        inventories
            .borrow()
            .iter()
            .filter(|(_, inventory)| inventory.workshop_id == workshop_id)
            .map(|(_, inventory)| inventory.quantity as f64 * inventory.unit_price)
            .sum()
    });

    Ok(total_value)
}

// Exporting the candid interface
ic_cdk::export_candid!();

use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;

// User related structures
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct User {
    username: String,
    password_hash: String,
    created_at: u64,
}

#[derive(CandidType, Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(CandidType, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

// Task related structures
type TaskId = u64;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Task {
    id: TaskId,
    title: String,
    completed: bool,
    important: bool,
    created_at: u64,
    due_date: u64,
    owner: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct CreateTaskPayload {
    title: String,
    important: bool,
    due_date: u64,
}

// State management
thread_local! {
    static TASKS: RefCell<BTreeMap<TaskId, Task>> = RefCell::new(BTreeMap::new());
    static NEXT_TASK_ID: RefCell<TaskId> = RefCell::new(0);
    static USERS: RefCell<HashMap<String, User>> = RefCell::new(HashMap::new());
    static ACTIVE_SESSIONS: RefCell<HashMap<Principal, String>> = RefCell::new(HashMap::new());
}

fn ensure_authenticated() -> Result<String, String> {
    let caller = ic_cdk::caller();
    ACTIVE_SESSIONS.with(|sessions| {
        sessions
            .borrow()
            .get(&caller)
            .cloned()
            .ok_or_else(|| "User not authenticated".to_string())
    })
}

#[init]
fn init() {
    ic_cdk::println!("Canister initialized");
}

#[update]
fn register(request: RegisterRequest) -> Result<(), String> {
    USERS.with(|users| {
        let mut users = users.borrow_mut();
        
        if users.contains_key(&request.username) {
            return Err("Username already exists".to_string());
        }

        let new_user = User {
            username: request.username.clone(),
            password_hash: request.password,
            created_at: ic_cdk::api::time(),
        };

        users.insert(request.username, new_user);
        Ok(())
    })
}

#[update]
fn login(request: LoginRequest) -> Result<(), String> {
    USERS.with(|users| {
        let users = users.borrow();
        
        if let Some(user) = users.get(&request.username) {
            if user.password_hash == request.password {
                let caller = ic_cdk::caller();
                ACTIVE_SESSIONS.with(|sessions| {
                    sessions.borrow_mut().insert(caller, request.username.clone());
                });
                return Ok(());
            }
        }
        
        Err("Invalid username or password".to_string())
    })
}

#[update]
fn logout() -> Result<(), String> {
    let caller = ic_cdk::caller();
    ACTIVE_SESSIONS.with(|sessions| {
        if sessions.borrow_mut().remove(&caller).is_some() {
            Ok(())
        } else {
            Err("Not logged in".to_string())
        }
    })
}

#[update]
fn create_task(payload: CreateTaskPayload) -> Result<TaskId, String> {
    let _username = ensure_authenticated()?;
    
    let id = NEXT_TASK_ID.with(|counter| {
        let next_id = *counter.borrow();
        *counter.borrow_mut() = next_id + 1;
        next_id
    });

    let task = Task {
        id,
        title: payload.title,
        completed: false,
        important: payload.important,
        created_at: ic_cdk::api::time(),
        due_date: payload.due_date,
        owner: ic_cdk::caller(),
    };

    TASKS.with(|tasks| {
        tasks.borrow_mut().insert(id, task);
    });

    Ok(id)
}

#[query]
fn get_my_tasks() -> Result<Vec<Task>, String> {
    let _username = ensure_authenticated()?;
    let caller = ic_cdk::caller();
    
    Ok(TASKS.with(|tasks| {
        tasks.borrow()
            .values()
            .filter(|task| task.owner == caller)
            .cloned()
            .collect()
    }))
}

#[query]
fn get_completed_tasks() -> Result<Vec<Task>, String> {
    let _username = ensure_authenticated()?;
    let caller = ic_cdk::caller();

    Ok(TASKS.with(|tasks| {
        tasks.borrow()
            .values()
            .filter(|task| task.owner == caller && task.completed)
            .cloned()
            .collect()
    }))
}

#[update]
fn toggle_task_status(title: String) -> Result<(), String> {
    let _username = ensure_authenticated()?;
    let caller = ic_cdk::caller();
    
    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        let task_id_to_toggle = tasks.iter()
            .find(|(_, task)| task.title == title && task.owner == caller)
            .map(|(id, _)| *id);

        if let Some(id) = task_id_to_toggle {
            let task = tasks.get_mut(&id).unwrap();
            task.completed = !task.completed;
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    })
}

#[update]
fn toggle_task_importance(title: String) -> Result<(), String> {
    let _username = ensure_authenticated()?;
    let caller = ic_cdk::caller();
    
    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        let task_id_to_toggle = tasks.iter()
            .find(|(_, task)| task.title == title && task.owner == caller)
            .map(|(id, _)| *id);

        if let Some(id) = task_id_to_toggle {
            let task = tasks.get_mut(&id).unwrap();
            task.important = !task.important;
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    })
}

#[update]
fn delete_task(title: String) -> Result<(), String> {
    let _username = ensure_authenticated()?;
    let caller = ic_cdk::caller();

    TASKS.with(|tasks| {
        let mut tasks = tasks.borrow_mut();
        let task_id_to_remove = tasks.iter()
            .find(|(_, task)| task.title == title && task.owner == caller)
            .map(|(id, _)| *id);

        if let Some(id) = task_id_to_remove {
            tasks.remove(&id);
            Ok(())
        } else {
            Err("Task not found".to_string())
        }
    })
}

#[query]
fn get_caller() -> Principal {
    ic_cdk::caller()
}

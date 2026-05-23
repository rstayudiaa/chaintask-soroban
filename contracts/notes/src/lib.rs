#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, String, Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub is_done: bool,
}

const TASK_DATA: Symbol = symbol_short!("TASK_DATA");

#[contract]
pub struct ChainTaskContract;

#[contractimpl]
impl ChainTaskContract {

    pub fn get_tasks(env: Env) -> Vec<Task> {
        env.storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env))
    }

    pub fn add_task(env: Env, id: u64, title: String) -> String {
        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        let task = Task {
            id,           // ← id sekarang diinput manual
            title,
            is_done: false,
        };

        tasks.push_back(task);
        env.storage().instance().set(&TASK_DATA, &tasks);

        String::from_str(&env, "Task berhasil ditambahkan")
    }

    pub fn complete_task(env: Env, id: u64) -> String {
        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            let mut task = tasks.get(i).unwrap();
            if task.id == id {
                task.is_done = true;
                tasks.set(i, task);
                env.storage().instance().set(&TASK_DATA, &tasks);
                return String::from_str(&env, "Task berhasil diselesaikan");
            }
        }

        String::from_str(&env, "Task tidak ditemukan")
    }

    pub fn delete_task(env: Env, id: u64) -> String {
        let mut tasks: Vec<Task> = env
            .storage()
            .instance()
            .get(&TASK_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            if tasks.get(i).unwrap().id == id {
                tasks.remove(i);
                env.storage().instance().set(&TASK_DATA, &tasks);
                return String::from_str(&env, "Task berhasil dihapus");
            }
        }

        String::from_str(&env, "Task tidak ditemukan")
    }
}

mod test;
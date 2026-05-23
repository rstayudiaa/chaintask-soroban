# ChainTask — Decentralized Task Manager

**ChainTask** - Blockchain-Based Decentralized Task Management System

## Project Description

ChainTask is a decentralized smart contract solution built on the 
Stellar blockchain using Soroban SDK. It provides a secure, immutable 
platform for managing personal tasks directly on the blockchain. 
The contract ensures that your data is stored transparently and is 
only manageable through predefined smart contract functions, 
eliminating reliance on centralized database providers.

The system allows users to add, view, complete, and delete tasks, 
leveraging the efficiency and security of the Stellar network. 
Each task is uniquely identified and stored within the contract's 
instance storage, ensuring data persistence and reliability.

## Project Vision

Our vision is to revolutionize personal productivity in the digital 
age by:

- **Decentralizing Data**: Moving task management from centralized 
  servers to a global, distributed blockchain
- **Ensuring Ownership**: Empowering users to have complete control 
  and ownership over their tasks and productivity data
- **Guaranteeing Immutability**: Providing a permanent, tamper-proof 
  record of tasks that cannot be altered by third parties
- **Enhancing Privacy**: Leveraging blockchain security to protect 
  personal task information from unauthorized access
- **Building Trustless Systems**: Creating a platform where data 
  integrity is guaranteed by code, not by company promises

## Key Features

### 1. **Add New Task**
- Create tasks with a simple function call
- Specify a unique ID and title for each task
- Default status set to incomplete (`is_done: false`)
- Persistent storage on the Stellar blockchain

### 2. **View All Tasks**
- Fetch all stored tasks in a single call
- Structured data with ID, title, and completion status
- Real-time synchronization with the blockchain state

### 3. **Complete Task**
- Mark specific tasks as done using their unique ID
- Updates `is_done` status to `true` on the blockchain
- Permanent and transparent status tracking

### 4. **Delete Task**
- Remove specific tasks using their unique IDs
- Permanent removal from the contract storage
- Immediate update of the task list after deletion

## Deployed Smart Contract Details

> ⚠️ Important

- **Contract ID** : [ISI CONTRACT ID KAMU DI SINI]
- **Network**     : Stellar Testnet (Soroban)

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar Testnet network

## Getting Started

Deploy the smart contract to Stellar's Soroban testnet and interact 
with it using the four main functions:

- `add_task(id, title)` - Add a new task with an ID and title
- `get_tasks()` - Retrieve all stored tasks from the contract
- `complete_task(id)` - Mark a specific task as completed
- `delete_task(id)` - Remove a specific task by its ID

---

**ChainTask** - Managing Your Tasks on the Blockchain

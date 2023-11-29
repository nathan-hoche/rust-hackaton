# Rust Hackaton with ink! and Substrate ( 8 am - 6 pm )

## 8:30 - 9:00 am: Welcome and Introductions

- Brief introduction of the instructor and Allfeat labs.
- Briefly talk about the agenda and learning objectives for the day.

## 9:00 - 12:00 am: Introduction to Rust

1 - For beginers that were not here last year at the rust bundle

[Rust Bundle](https://nasso.dev/rust-bundle/introduction.html)

- Introduction to Rust: Its significance and real-world applications. ( live talking session)
- Understanding Rust syntax.
- Setting up the Rust environment.
- Variables, data types, control flow, and functions in Rust.
- Understanding ownership, borrowing, and lifetime.
- Structs and enums.

2 - For people that were here last year at the rust bundle/already know well rust

You can find 5 more exercices inside the `rust advanced exercices` folder. There is also a bonus exercice with a little command line interface application to create for those who want to go further :)

## 12:00 - 1:30 pm: Lunch Break

## 1:30 - 2:00 pm: Introduction to Blockchain and Smart Contracts

- Brief overview of Blockchain technology.
- Understanding Smart Contracts.

### What is Blockchain?

Blockchain is a decentralized and distributed digital ledger that records transactions across many computers in such a way that the registered transactions cannot be altered retroactively. This technology aims to increase security and transparency in all business sectors.

### How Does Blockchain Work?

- `Transaction`: Each transaction made is known as a block in the blockchain.
- `Block`: Every time a block is completed, it gives way to the next block in the blockchain.
- `Verification`: The transaction within the block needs to be verified by the majority of the participants in the system.
- `Hash Function`: Each block contains a unique code known as a hash. It also contains the hash of the previous block in the chain.
- `Immutable`: The information recorded in the blocks can never be altered.

### What are Smart Contracts?

A smart contract is a self-executing contract with the terms of the agreement between parties being directly written into lines of code. They automate transactions and remove the need for intermediaries.

### How Do Smart Contracts Work?

- `Agreement`: Parties involved in a transaction agree to terms and conditions.
- `Coding`: The agreement is coded into a smart contract.
- `Blockchain`: The contract is stored on the blockchain.
- `Execution`: When predetermined conditions are met, the contract executes automatically.
- `Validation`: The blockchain validates the transaction.
- `Record`: The result is recorded onto the blockchain.

### Blockchain and Smart Contracts: A Powerful Combo

The decentralization, security, and transparency of the blockchain combined with the automation of smart contracts create a powerful tech combination. Blockchain provides a trustworthy environment for smart contracts, and smart contracts bring programmable automation and efficiency to blockchain systems. They are particularly powerful in applications like DeFi (Decentralized Finance), supply chain management, voting systems, and any application where trust, transparency, and immutability are key.

## 2:00 - 2:30 pm: Introduction to ink! and Substrate

- Role of Substrate in the Polkadot ecosystem.
- Introduction to ink!.

### What is Substrate?

Substrate is a modular and extensible blockchain framework that allows developers to build their own blockchains for different applications. It is developed by Parity Technologies and used as the foundational layer for the Polkadot network, but it can also be used standalone.

Key Features of Substrate:

- `Modular`: Substrate's modularity allows developers to choose what components they need for their blockchain, promoting innovation and variety.
- `Runtime` Upgradeability: Substrate chains can upgrade their runtime without a hard fork, reducing disruption and enabling easy feature additions and bug fixes.
- `Interoperability`: When built with Polkadot, Substrate chains can communicate with each other and external blockchains.

### What is ink!?

ink! is a Rust-based eDSL (embedded Domain Specific Language) for writing smart contracts on Substrate-based blockchains. ink! allows developers to write secure and efficient smart contracts with all the benefits of the Rust language.

Key Features of ink!:

- `Integration`: As a Rust-based eDSL, ink! allows you to use the power of Rust’s toolset, including its compiler and type checking.
- `Efficiency`: ink! contracts are compiled to WebAssembly (Wasm), providing great execution speed and efficient use of resources.
- `Security`: With Rust’s strict compiler, ink! contracts provide a high level of security and robustness.

### How do Substrate and ink! Work Together?

Substrate provides the base layer for creating blockchains, while ink! provides the toolset for writing smart contracts to be deployed on those blockchains. They work together to offer a comprehensive framework for decentralized application (dApp) development.

Here's a simple process:

- Create Blockchain with Substrate: Utilize Substrate's modules (called pallets) to create the specific functionalities your blockchain needs.
- Write Smart Contracts with ink!: Use ink! to write the business logic of your dApp as a smart contract.
- Deploy Contracts: Deploy the ink! smart contracts onto your Substrate-based blockchain.
- Interact with Contracts: Users can interact with the smart contracts, invoking its functions and changing its state.

In summary, Substrate and ink! are both critical elements of the Polkadot ecosystem, providing developers with the flexibility and tools they need to build innovative and unique blockchain solutions.

## 2:30 - 3:00 pm: Setting up the ink! Development Environment

- Install Ink! eSDL (Embedded Domain Specific Language) and clone the substrate-node-template [here](https://use.ink/getting-started/setup)
- Install the Substrate SDK [here](https://docs.substrate.io/install)

## 3:00 - 5:30 pm: Writing and Deploying an ink! Smart Contract

- Basics of ink! with example of todo list smart contract
- Writing a simple ink! smart contract on live code session
- Deploying and interacting with the contract on a local Substrate node
- Interactive session: Students write, deploy, and interact with their own ink! smart contracts

Write a smart contract to handle the roles of a forum application. The contract should have the following features:

- One user is SuperAdmin role and can set any user to any role ( the SuperAdmin user is set upon contract initialization ).
- Users can be set to Admin role, which lets your add/remove moderators.
- Users can be set to Moderator role, which lets them add/remove posts.
- A method `get_role` that returns the role of the caller.
- Use the `AccountId` of ink! framework to differentiate users inside your storage.
- Don't mind the posts part, just handle the roles.

## 5:30 - 6:00 pm: Wrap-up and Closing Thoughts

- Review of the day's work.
- Discussion on the challenges faced and how they were overcome.
- Open forum for questions and sharing of experiences.
- Introduction to further resources for learning and possible next steps.

# CosmWasm Starter Pack

## Simple Smart Contract built on Terra
This is a simple smart contract with the following functionalities:
- you should be able to instantiate the contract and set the owner
- you should support a read query to get the owner of the smart contract
- you should store the score for different addresses in the smart contract state (ex. {address_1: 10, address_2: 20}) 
- you should support an execute message where only the owner of the smart contract can set the score of an address 
- you should support a read query to get the score for a particular address
- you should write unit tests for all of these scenarios

Bonus:
- Address's score breakdown by token type
- Deploy on Terra testnet

## Resources for building smart contract
- Terra official documentation: https://docs.terra.money/
- Terra Academy (Step-by-step environment setup guides): https://academy.terra.money/courses/cosmwasm-smart-contracts-i
- CosmWasm (Smart contract packages): https://docs.cosmwasm.com/docs/1.0/
- Rust Programming Language Guide: https://doc.rust-lang.org/book/title-page.html
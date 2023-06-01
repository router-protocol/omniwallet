# Omniwallet
A POC to showcase Account Abstraction using Routerchain

### Workflow
#### Onboarding Workflow

- Alice connects her wallet to Routerchain.
- Setup Forwarder contract
    - Alice will deploy forwarded contract which acts as a forwarder for all his requests. It’s owned by Alice only.
- Setup Abstract Accounts.
    - Select Chain and click deploy.
    - This will deploy Abstract Account smart contract on the specified chain.
- Register AbstractAccount with Forwarder.
    - Register abstract Account with Forwarder.
    - Forwarder contract will and add following mapping in its state.
        
        (**ChainID - AbstractAccount**)
        
- Now Alice has abstract accounts on various blockchain networks that she can access from Forwarder on Routerchain.
    - User has to transfer funds or provide approval to these accounts separately to access his funds.

#### Chain Agnostic experience

- After onboarding, User can see all his abstract account balances in the wallet.
- Alice can interact with his ForwarderContract on Routerchain for all transactions (let's say transfer USDC to Bob on chainA).
    - This could be a Batch of transfers as well. (Dust collection)
- Upon receiving the request, The ForwarderContract will initiate an outbound request to chainA.
- The request is forwarded to his **AbstractAccount** on chainA which will
    - Validate if sender is ForwarderContract.
    - Does the required transfer.

#### Security Considerations

- The AbstractAccount can be invoked only by Forwarder contract.
- Forwarder Contract can be invoked only by respective owner (user)

#### Documentation

To Build the code documentation, Please use the following command.

```shell
cd middlwware
cargo doc --document-private-items
```
The documentation will be generated in HTML format and can be viewed by opening the `target/doc` directory. Open the `index.html` file in your browser to navigate through the documentation.

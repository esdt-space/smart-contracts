# Donations 
Smart contract for raising donations on Elrond Network

> **Warning**
> This code is meant to be used for inspirational purposes, use it at your own risk.

# Features

- [x] Whitelist for tokens to be used for donations
- [x] Whitelist for addresses to claim the donations
- [x] View for getting the total amount of tokens raised
- [x] View for getting the total amount of tokens donated by an address

# Contract build
```bash
erdpy --verbose contract build "/path/to/folder/donations"
```

# Deploying the smart contract

```bash
erdpy --verbose contract deploy --recall-nonce --pem="wallet.pem" --gas-limit=30000000 --proxy="https://testnet-gateway.elrond.com" --chain=T --project=donations --send || return
```

> **Note**
> Make sure to replace the pem file location with your own. In case that you are deploying to an environment different than testnet, you would need to change the --proxy and the --chain parameters as well.

### Deploying the smart contract through erdpy.json
```json
{
    "configurations": {
        "default": {
            "proxy": "https://testnet-gateway.elrond.com",
            "chainID": "T"
        }
    },
    "contract":{
        "deploy":{
            "verbose": true,
            "bytecode": "output/donations.wasm",
            "recall-nonce": true,
            "pem": "../wallet.pem",
            "gas-limit": 30000000,
            "arguments": [],
            "send": true,
            "outfile": "donations.json"
        }
     }
}
```
Using this command, you will deploy your smart contract on testnet with the above specified "configurations"

```bash
erdpy contract deploy
```

# Upgrading the smart contract

```bash
erdpy --verbose contract upgrade erd1qqqqqqqqqqqqqpgqdujpgae2kszektz63rxtdd0tkvpnl5qprp8sxue9kq --recall-nonce --pem="wallet.pem" --gas-limit=30000000 --proxy="https://testnet-gateway.elrond.com" --chain=T --project=donations --send || return
```

> **Note**
> Make sure to replace the contract address and the pem file location with your own. In case that you are deploying to an environment different than testnet, you would need to change the --proxy and the --chain parameters as well.

# Endpoints

### Contract Owner Endpoints

- `enableToken` (It allows the smart contract owner to enable a token)
```rust
fn enable_token(&self, token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>)
```

- `disableToken` (It allows the smart contract owner to disable a token)
```rust
fn disable_token(&self, token_identifier: &EgldOrEsdtTokenIdentifier<Self::Api>)
```

- `whitelistWithdrawAddress` (It allows the smart contract owner to whitelist an address for withdrawing the tokens)
```rust
fn whitelist_withdraw_address(&self, address: ManagedAddress<Self::Api>) 
```

- `removeWithdrawAddress` (It allows the smart contract owner to remove a withdraw address from the whitelist)
```rust
fn remove_withdraw_address(&self, address: &ManagedAddress<Self::Api>)
```

### Protected Endpoints
- `claimTokens` (It allows the whitelisted addresses to claim the tokens donated)
```rust
#[endpoint(claimTokens)]
fn claim_tokens(&self)
```

### Public Endpoints
- `donateEgld` (It allows anyone to donate with EGLD)
```rust
#[payable("EGLD")]
fn donate_egld(&self)
```

- `donateEsdt` (It allows anyone to donate with ESDT)
```rust
#[payable("*")]
fn donate_esdt(&self)
```
   
### Views
- `getTotalDonations` (It returns the total amount of tokens raised)
```rust
fn get_total_donations(&self) -> MultiValueEncoded<(EgldOrEsdtTokenIdentifier, BigUint)>
```

- `getUserDonations` (It returns the total amount of tokens donated by an address)
```rust
#[view(getUserDonations)]
fn get_user_donations(&self, address: &ManagedAddress) -> MultiValueEncoded<(EgldOrEsdtTokenIdentifier, BigUint)>
```
  

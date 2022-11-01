# Subscriptions 
Smart contract for managing subscriptions on Elrond Network

> **Warning**
> This code is meant to be used for inspirational purposes, use it at your own risk.

# Features

- to be added

# Contract build
```bash
erdpy --verbose contract build "/path/to/folder/subscriptions"
```

# Deploying the smart contract

```bash
erdpy --verbose contract deploy --recall-nonce --pem="wallet.pem" --gas-limit=30000000 --proxy="https://testnet-gateway.elrond.com" --chain=T --project=subscriptions --send || return
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
            "bytecode": "output/subscriptions.wasm",
            "recall-nonce": true,
            "pem": "../wallet.pem",
            "gas-limit": 30000000,
            "arguments": [],
            "send": true,
            "outfile": "subscriptions.json"
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
erdpy --verbose contract upgrade erd1qqqqqqqqqqqqqpgqdujpgae2kszektz63rxtdd0tkvpnl5qprp8sxue9kq --recall-nonce --pem="wallet.pem" --gas-limit=30000000 --proxy="https://testnet-gateway.elrond.com" --chain=T --project=subscriptions --send || return
```

> **Note**
> Make sure to replace the contract address and the pem file location with your own. In case that you are deploying to an environment different than testnet, you would need to change the --proxy and the --chain parameters as well.

# Endpoints

TO BE ADDED
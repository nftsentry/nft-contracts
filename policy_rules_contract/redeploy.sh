key=$(near view-state --finality final policies.rocketscience.testnet | grep key | cut -d ':' -f 2 | cut -d "'" -f 2)

near call --account_id rocketscience.testnet policies.rocketscience.testnet clean '{"keys": ["'$key'"]}'

near deploy --force policies.rocketscience.testnet res/policy_rules_contract.wasm new '{"owner_id": "rocketscience.testnet"}'

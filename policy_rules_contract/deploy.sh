base_account=rocketscience.testnet

near create-account policies.$base_account --masterAccount $base_account --initialBalance 7.5

near deploy --force policies.$base_account res/policy_rules_contract.wasm new '{"owner_id": "'$base_account'"}'

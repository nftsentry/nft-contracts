base_account=rocketscience.testnet

key=$(near view-state --finality final policies.$base_account | grep key | cut -d ':' -f 2 | cut -d "'" -f 2)

near call --account_id $base_account policies.$base_account clean '{"keys": ["'$key'"]}'

near deploy --force policies.$base_account res/policy_rules_contract.wasm new '{"owner_id": "'$base_account'"}'

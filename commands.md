#### Init raffle
##### 30000000000000 - 8.3 hours
##### 300000000000 - 5 minutes
near call dev-1661888032269-81442203045683 new '{ "raffle_owner": "raffly-root.near", "ticket_price": "1000000000000000000000000", "starting_period": "300000000000", "ending_period": "300000000000", "owner_commission": "4", "raffle_name": "Liquid Raffle", "raffle_description": "Honest, unlosable, and transparent raffle" }' --accountId dev-1661888032269-81442203045683

#### Participate in raffle
near call liquid.raffly-root.near participate '{ "is_auto_prolong": true }' --accountId verkhohliad.near --amount 1.001 --gas 300000000000000

#### Start Raffle
near call liquid.raffly-root.near start_raffle --accountId liquid.raffly-root.near

#### End Raffle
near call liquid.raffly-root.near end_raffle --accountId liquid.raffly-root.near --gas 300000000000000

#### Change Ending Period
near call liquid.raffly-root.near change_ending_period '{ "new_ending_period": "2678400000000000" }' --accountId liquid.raffly-root.near

#### Change Starting Period
near call liquid.raffly-root.near change_starting_period '{ "new_starting_period": "172800000000000" }' --accountId liquid.raffly-root.near

#### Start Unstake Period
near call liquid.raffly-root.near start_unstake_period --accountId liquid.raffly-root.near --gas 300000000000000

#### Migration
near call liquid.raffly-root.near migrate --accountId liquid.raffly-root.near

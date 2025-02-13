mod mock;
use frame_support::assert_ok;

use mock::*;
use pallet_subspace::global::BurnConfiguration;
use sp_core::U256;

// test subnet specific burn
#[test]
fn test_local_subnet_burn() {
    new_test_ext().execute_with(|| {
        let min_burn = to_nano(10);
        let max_burn = to_nano(1000);

        let mut burn_config = BurnConfiguration::<Test>::default();

        // set the min_burn to 10 $COMAI
        burn_config.min_burn = min_burn;

        // Adjust max burn to allow for the burn to move
        burn_config.max_burn = max_burn;

        // Adjust max registrations per block to a high number.
        // We will be doing "registration raid"
        burn_config.adjustment_interval = 200;
        burn_config.expected_registrations = 25;

        assert_ok!(burn_config.apply());

        SubspaceModule::set_max_registrations_per_block(5);

        // register the general subnet
        assert_ok!(register_module(0, U256::from(0), to_nano(20)));

        // register 500 modules on yuma subnet
        let netuid = 1;
        let n = 300;
        let initial_stake: u64 = to_nano(500);

        SubspaceModule::set_max_registrations_per_block(1000);
        // this will perform 300 registrations and step in between
        for i in 1..n {
            // this registers five in block
            assert_ok!(register_module(netuid, U256::from(i), initial_stake));
            if i % 5 == 0 {
                // after that we step 30 blocks
                // meaning that the average registration per block is 0.166..
                step_block(30);
            }
        }

        // We are at block 1,8 k now.
        // We performed 300 registrations
        // this means avg.  0.166.. per block
        // burn has incrased by 90% > up

        let subnet_zero_burn = SubspaceModule::get_burn(0);
        assert_eq!(subnet_zero_burn, min_burn);
        let subnet_one_burn = SubspaceModule::get_burn(1);
        assert!(min_burn < subnet_one_burn && subnet_one_burn < max_burn);
    });
}

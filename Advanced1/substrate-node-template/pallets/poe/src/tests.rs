use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn test_create_claim() {
    new_test_ext().execute_with(|| {
        // create_claim must be called by account signed .
        assert_noop!(
            Poe::create_claim(
                Origin::root(),
                "hello, world".to_string().as_bytes().to_vec()
            ),
            BadOrigin
        );

        assert_ok!(Poe::create_claim(
            Origin::signed(1),
            "hello, world".to_string().as_bytes().to_vec()
        ));

        // if claim is existed, call create_claim will failed.
        assert_noop!(
            Poe::create_claim(
                Origin::signed(1),
                "hello, world".to_string().as_bytes().to_vec()
            ),
            Error::<Test>::ProofAlreadyExist
        );
    });
}

#[test]
fn test_create_claim_exceed_size_limit() {
    let limit_size = ClaimSizeLimit::get() as usize;
    new_test_ext().execute_with(|| {
        // test claim with large size.
        assert_noop!(
            Poe::create_claim(
                Origin::signed(1),
                vec![0u8; limit_size.checked_add(1).unwrap()]
            ),
            Error::<Test>::ClaimSizeOverflow
        );

        assert_ok!(Poe::create_claim(
            Origin::signed(1),
            "hello, world".to_string().as_bytes().to_vec()
        ));
    });
}

#[test]
fn test_revoke_claim() {
    new_test_ext().execute_with(|| {
        // Create claim "hello, world".
        assert_ok!(Poe::create_claim(
            Origin::signed(1),
            "hello, world".to_string().as_bytes().to_vec()
        ));

        // revoke_claim must be called by account signed .
        assert_noop!(
            Poe::revoke_claim(
                Origin::root(),
                "hello, world".to_string().as_bytes().to_vec()
            ),
            BadOrigin
        );

        // revoke claim must be owner.
        assert_noop!(
            Poe::revoke_claim(
                Origin::signed(2),
                "hello, world".to_string().as_bytes().to_vec()
            ),
            Error::<Test>::NotClaimOwner
        );

        // revoke claim.
        assert_ok!(Poe::revoke_claim(
            Origin::signed(1),
            "hello, world".to_string().as_bytes().to_vec()
        ));
    });
}

#[test]
fn test_transfer_claim() {
    new_test_ext().execute_with(|| {
        // Create claim "hello, world".
        assert_ok!(Poe::create_claim(
            Origin::signed(1),
            "hello, world".to_string().as_bytes().to_vec()
        ));

        // transfer_claim must be called by account signed .
        assert_noop!(
            Poe::transfer_claim(
                Origin::root(),
                "hello, world".to_string().as_bytes().to_vec(),
                2
            ),
            BadOrigin
        );

        // transfer_claim must be called by the owner.
        assert_noop!(
            Poe::transfer_claim(
                Origin::signed(3),
                "hello, world".to_string().as_bytes().to_vec(),
                2
            ),
            Error::<Test>::NotClaimOwner
        );

        // transfer claim from 1 to 2.
        assert_ok!(Poe::transfer_claim(
            Origin::signed(1),
            "hello, world".to_string().as_bytes().to_vec(),
            2
        ));
    });
}

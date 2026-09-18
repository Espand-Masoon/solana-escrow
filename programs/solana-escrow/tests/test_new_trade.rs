use anchor_lang::{AccountDeserialize, InstructionData, Space, ToAccountMetas};
use litesvm::LiteSVM;
use solana_address::Address;
use solana_escrow::state::Trade;
use solana_keypair::Keypair;
use solana_message::{self, Instruction, VersionedMessage};
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;

#[test]
fn test_new_trade() {
    let mut svm = LiteSVM::new();
    let party_1 = Keypair::new();
    let party_2 = Keypair::new();
    let program_bytes = include_bytes!("../../../target/deploy/solana_escrow.so");
    let program_id = solana_escrow::id();
    svm.add_program(program_id, program_bytes).unwrap();
    svm.airdrop(&party_1.pubkey(), 1_000_000).unwrap();

    let data = &solana_escrow::instruction::NewTrade {}.data();
    let (trade, _) = Address::find_program_address(
        &[&party_1.pubkey().to_bytes(), &party_2.pubkey().to_bytes()],
        &program_id,
    );
    let accounts = solana_escrow::accounts::NewTrade {
        party_1: party_1.pubkey(),
        party_2: party_2.pubkey(),
        trade,
        system_program: solana_system_interface::program::id(),
    }
    .to_account_metas(None);
    let instruction = Instruction::new_with_bytes(program_id, data, accounts);
    let blockhash = svm.latest_blockhash();
    let mut message = solana_message::Message::new_with_blockhash(
        &[instruction],
        Some(&party_1.pubkey()),
        &blockhash,
    );

    let tx = VersionedTransaction::try_new(
        solana_message::VersionedMessage::Legacy(message.clone()),
        &[&party_1],
    )
    .unwrap();
    let tx_result = svm.send_transaction(tx.clone());

    assert!(tx_result.is_ok());

    // Check the PDA is created correctly
    let trade_account = svm.get_account(&trade).unwrap();

    // -- Owner should be the program
    assert_eq!(trade_account.owner, program_id);

    // -- Should have two fields in its data with the value of false
    let trade_struct = Trade::try_deserialize(&mut trade_account.data.as_slice()).unwrap();
    assert!(!trade_struct.approved_by_party_1);
    assert!(!trade_struct.approved_by_party_2);

    // -- The size of its data must be correct
    assert_eq!(trade_account.data.len(), 8 + Trade::INIT_SPACE);

    // Check what happens if I try to recreate an existing trade
    svm.expire_blockhash();
    message.recent_blockhash = svm.latest_blockhash();
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(message), &[party_1]).unwrap();
    let tx_result = svm.send_transaction(tx);
    assert!(tx_result.is_err(), "Transaction should fail, because a Trade already exists");
}

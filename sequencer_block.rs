use app::{generate_random_transactions, run_block};
use axum::body::Body;
use axum::http::{response, Request};
use axum::Json;
use constants::{BACH_SIZE, BLOCK_SIZE};
use crates::common::NO_OF_ROWS_PER_TRANSACTION;
use crates::ecdsa::math::field::field::FieldElement;
use crates::state_updates::broadcast_event;
use crates::table::table::Table;
use crates::table::trace_table::TraceTable;
use database::in_memory::IS_BLOCK_RUNNING;
use database::in_memory::{
    drop_transactions, fetch_transactions, get_new_block_id, get_new_transaction_id, TREE,
    USER_DATA,
};
use database::{Database, PERSISTANT_DB};
use lazy_static::lazy_static;
use models::api::block::{
    GetBlockResponse, GetTransactionResponse, ListBlockItem, ListTransactionItem,
};
use models::api::cursor::Cursor;
use models::api::response::GenericError;
use models::block::{BlockCreationEventData, BlockData, BlockStatus, SendProofRequest, StringList};
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::ser::SerializeStruct;
use serde::Serialize;
use serde_json::{error, json};
use std::cmp::{max, Reverse};
use std::sync::Mutex;
use std::time::Instant;
use utils_v2::get_current_timestamp;
use wallet::{Transaction, TransactionRequest, UserData, WalletAddress};

lazy_static! {
    static ref BLOCK_CREATION_MUTEX: Mutex<()> = Mutex::new(());
}

pub struct BlockService {}

const MAIN_PROVER_HOST: &str = "http://13.234.157.247:8080";

impl BlockService {
    pub fn get_transaction_by_id(transaction_id: String) -> GetTransactionResponse {
        let transaction_data = PERSISTANT_DB
            .lock()
            .unwrap()
            .get_transaction_by_id(transaction_id);

        match transaction_data {
            Ok(transaction) => GetTransactionResponse {
                transaction_id: transaction.id,
                block_id: transaction.block_id,
                age_ts: transaction.age_ts,
                creation_td: transaction.creation_td,
                amount: transaction.amount,
                to: transaction.to.as_hex(),
                from: transaction.from.as_hex(),
                status: BlockStatus::Validated,
            },
            Err(_) => GetTransactionResponse::default(),
        }
    }

    pub fn get_block_by_id(block_id: String) -> GetBlockResponse {
        let block_data = PERSISTANT_DB.lock().unwrap().get_block_by_id(block_id);

        match block_data {
            Ok(block) => GetBlockResponse {
                block_id: block.block_id.clone(),
                age_ts: block.age_ts,
                transactions_count: block.txn_ids.len(),
                creation_td: block.creation_td,
                status: BlockStatus::Validated,
                block_hash: block.block_hash,
                gas_cost: 32453,
            },
            Err(_) => GetBlockResponse::default(),
        }
    }

    pub fn list_transactions(
        limit: Option<usize>,
        cursor: Option<Cursor>,
    ) -> (Vec<ListTransactionItem>, Cursor, usize) {
        let mut cursor = match cursor {
            Some(cursor_value) => cursor_value,
            None => Cursor::new(),
        };
        let last_transaction_id_sent = cursor.get("t".to_string());

        let mut transactions_list;
        {
            transactions_list = PERSISTANT_DB.lock().unwrap().list_transactions();
        }
        let total_transactions_count = transactions_list.len();
        // Sort by transaction id in descending order
        transactions_list.sort_by_key(|k| Reverse(k.id.parse::<usize>().unwrap()));
        // Slice all transactions greater than the last sent transaction
        if let Some(last_transaction_id_sent) = last_transaction_id_sent {
            transactions_list.retain(|transaction| {
                let transaction_id = transaction.id.parse::<u128>().unwrap();
                let last_transaction_id = last_transaction_id_sent.parse::<u128>().unwrap();
                transaction_id < last_transaction_id
            });
        }
        // limit the number of transactions sent
        let transactions_list = match limit {
            Some(limit_value) => transactions_list.into_iter().take(limit_value).collect(),
            None => transactions_list,
        };

        let mut transactions_data = vec![];
        let transactions_count = transactions_list.len();
        for (idx, transaction) in transactions_list.into_iter().enumerate() {
            transactions_data.push(ListTransactionItem {
                transaction_id: transaction.id.clone(),
                age_ts: transaction.age_ts,
                creation_td: transaction.creation_td,
                amount: (transaction.amount as f64 / 10u128.pow(18) as f64),
                to: transaction.to.as_hex(),
                from: transaction.from.as_hex(),
                status: BlockStatus::Validated,
            });
            // Update Next cursor with the last block id being sent
            if idx == transactions_count - 1 {
                cursor.insert("t".to_string(), transaction.id);
            }
        }

        (transactions_data, cursor, total_transactions_count)
    }

    pub fn list_blocks(
        limit: Option<usize>,
        cursor: Option<Cursor>,
    ) -> (Vec<ListBlockItem>, Cursor, usize) {
        let mut cursor = match cursor {
            Some(cursor_value) => cursor_value,
            None => Cursor::new(),
        };
        let last_block_id_sent = cursor.get("b".to_string());

        let mut blocks_list;
        {
            blocks_list = PERSISTANT_DB.lock().unwrap().list_blocks();
        }
        let total_blocks_count = blocks_list.len();
        // Sort by block id in descending order
        blocks_list.sort_by_key(|k| Reverse(k.block_id.parse::<u128>().unwrap()));
        // Slice all blocks greater than the last sent block
        if let Some(last_block_id_sent) = last_block_id_sent {
            blocks_list.retain(|block| {
                let block_id = block.block_id.parse::<u128>().unwrap();
                let last_block_id = last_block_id_sent.parse::<u128>().unwrap();
                block_id < last_block_id
            });
        }
        // limit the number of blocks sent
        let blocks_list = match limit {
            Some(limit_value) => blocks_list.into_iter().take(limit_value).collect(),
            None => blocks_list,
        };

        let mut blocks_data = vec![];
        let blocks_count = blocks_list.len();
        for (idx, block) in blocks_list.into_iter().enumerate() {
            blocks_data.push(ListBlockItem {
                block_id: block.block_id.clone(),
                age_ts: block.age_ts,
                transactions_count: block.txn_ids.len(),
                creation_td: block.creation_td,
                status: BlockStatus::Validated,
                block_hash: block.block_hash.clone(),
            });
            // Update Next cursor with the last block id being sent
            if idx == blocks_count - 1 {
                cursor.insert("b".to_string(), block.block_id);
            }
        }
        (blocks_data, cursor, total_blocks_count)
    }

    // pub fn generate_block() -> bool {
    //     match IS_BLOCK_RUNNING.try_lock() {
    //         Ok(mut guard) => {
    //             if *guard == true {
    //                 return false;
    //             }
    //             *guard = true;
    //             tokio::spawn(async move {
    //                 // The lock was available, so we acquired it
    //                 let block_id = get_new_block_id();
    //                 println!("Started block generation {block_id}");

    //                 let mut transaction_list: Vec<TransactionRequest> =
    //                     fetch_transactions(BLOCK_SIZE);

    //                 let age_ts = get_current_timestamp();
    //                 let start_time = Instant::now();
    //                 let trace: Table;
    //                 {
    //                     let mut tree = TREE.lock().unwrap();
    //                     let mut user_data = USER_DATA.lock().unwrap();
    //                     let mut user_data_list: Vec<(WalletAddress, UserData)> = user_data
    //                         .iter()
    //                         .map(|(address, user_data)| (*address, user_data.clone()))
    //                         .collect();
    //                     user_data_list.sort_by_key(|(_, user_data)| user_data.id);
    //                     let addiitonal_random_transactions = generate_random_transactions(
    //                         max(transaction_list.len().next_power_of_two(), BLOCK_SIZE)
    //                             - transaction_list.len(),
    //                         user_data_list,
    //                     );
    //                     transaction_list.extend_from_slice(&addiitonal_random_transactions);
    //                     trace = run_block(&mut tree, &mut user_data, transaction_list.clone());
    //                 };
    //                 let creation_td = start_time.elapsed().as_millis();
    //                 drop_transactions(transaction_list.clone());
    //                 let mut txn_ids = vec![];
    //                 let mut amount = 0;
    //                 let mut transactions = vec![];
    //                 for txn in transaction_list {
    //                     txn_ids.push(txn.id);
    //                     amount += txn.amount;
    //                     transactions.push(Transaction {
    //                         to: txn.to,
    //                         from: txn.from,
    //                         amount: txn.amount,
    //                         id: get_new_transaction_id(),
    //                         age_ts: age_ts,
    //                         creation_td: creation_td,
    //                         block_id: block_id.clone(),
    //                     })
    //                 }
    //                 let block_data = BlockData {
    //                     block_id: block_id.clone(),
    //                     txn_ids: { StringList(txn_ids.clone()) },
    //                     amount: amount,
    //                     age_ts: age_ts,
    //                     creation_td: creation_td,
    //                     block_hash: block_hash.clone(),
    //                 };

    //                 {
    //                     let db = PERSISTANT_DB.lock().unwrap();
    //                     db.add_block(block_data.clone());
    //                     db.add_transactions(transactions);
    //                     {
    //                         db.upsert_user_data_bulk(USER_DATA.lock().unwrap().clone());
    //                     }
    //                 }

    //                 println!("Done block generation {block_id}");
    //                 broadcast_event(
    //                     "block_creation",
    //                     &BlockCreationEventData {
    //                         block_id: block_id,
    //                         age_ts: age_ts,
    //                         transactions_count: txn_ids.len(),
    //                         creation_td: creation_td,
    //                         status: BlockStatus::Validated,
    //                         block_hash: block_hash,
    //                     },
    //                 );
    //                 // Set the running flag back to false
    //                 {
    //                     let mut is_running = IS_BLOCK_RUNNING.lock().unwrap();
    //                     *is_running = false;
    //                     drop(is_running);
    //                 }
    //             });
    //             true
    //         }
    //         Err(_) => false,
    //     }
    // }
    pub fn generate_trace() -> bool {
        match IS_BLOCK_RUNNING.try_lock() {
            Ok(mut guard) => {
                if *guard == true {
                    return false;
                }
                *guard = true;

                tokio::spawn(async move {
                    // The lock was available, so we acquired it
                    let block_id = get_new_block_id();
                    println!("Started block generation {block_id}");

                    let age_ts = get_current_timestamp();
                    let start_time = Instant::now();
                    let mut txns = Vec::new();
                    let num_batches = BLOCK_SIZE / BACH_SIZE;
                    let num_txns = num_batches * BACH_SIZE;
                    let mut trace_1: Table = Table::new(vec![
                        vec![FieldElement::ZERO; 2];
                        num_txns * 128
                    ]);
                    let mut trace_2: Table = Table::new(vec![
                        vec![FieldElement::ZERO; 32];
                        num_txns * 256
                    ]);
                    let mut trace_3: Table = Table::new(vec![
                        vec![FieldElement::ZERO; 4];
                        num_txns * 16
                    ]);
                    let mut trace_4: Table = Table::new(vec![
                        vec![FieldElement::ZERO; 31];
                        num_txns * 256
                    ]);

                    // read trace_1 from trace_1.txt
                    let mut reader = csv::Reader::from_path("trace_1.txt").unwrap();
                    println!("reading trace_1 from file trace_1.txt");
                    let mut row_count = 0;
                    for result in reader.records() {
                        if row_count == trace_1.num_of_rows() {
                            break;
                        }
                        let record = result.unwrap();
                        let mut row: Vec<FieldElement> = Vec::new();
                        for field in record.iter() {
                            row.push(FieldElement::from_be_hex(field));
                        }
                        trace_1.update_row_at(row_count, &row);
                        row_count += 1;
                    }

                    // read trace_2 from trace_2.txt
                    let mut reader = csv::Reader::from_path("trace_2.txt").unwrap();
                    println!("reading trace_2 from file trace_2.txt");
                    let mut row_count = 0;
                    for result in reader.records() {
                        if row_count == trace_2.num_of_rows() {
                            break;
                        }
                        let record = result.unwrap();
                        let mut row: Vec<FieldElement> = Vec::new();
                        for field in record.iter() {
                            row.push(FieldElement::from_be_hex(field));
                        }
                        trace_2.update_row_at(row_count, &row);
                        row_count += 1;
                    }

                    // read trace_3 from trace_3.txt
                    let mut reader = csv::Reader::from_path("trace_3.txt").unwrap();
                    println!("reading trace_3 from file trace_3.txt");
                    let mut row_count = 0;
                    for result in reader.records() {
                        if row_count == trace_3.num_of_rows() {
                            break;
                        }
                        let record = result.unwrap();
                        let mut row: Vec<FieldElement> = Vec::new();
                        for field in record.iter() {
                            row.push(FieldElement::from_be_hex(field));
                        }
                        trace_3.update_row_at(row_count, &row);
                        row_count += 1;
                    }

                    // read trace_4 from trace_4.txt
                    let mut reader = csv::Reader::from_path("trace_4.txt").unwrap();
                    println!("reading trace_4 from file trace_4.txt");
                    let mut row_count = 0;
                    for result in reader.records() {
                        if row_count == trace_4.num_of_rows() {
                            break;
                        }
                        let record = result.unwrap();
                        let mut row: Vec<FieldElement> = Vec::new();
                        for field in record.iter() {
                            row.push(FieldElement::from_be_hex(field));
                        }
                        trace_4.update_row_at(row_count, &row);
                        row_count += 1;
                    }

                    for trace_id in 0..BLOCK_SIZE / BACH_SIZE {
                        let mut transaction_list: Vec<TransactionRequest> =
                            fetch_transactions(BACH_SIZE);

                        let mut current_trace_1: Table =
                            Table::new(vec![vec![FieldElement::ZERO; 2]; BACH_SIZE * 128]);
                        let mut current_trace_2: Table =
                            Table::new(vec![vec![FieldElement::ZERO; 32]; BACH_SIZE * 256]);
                        let mut current_trace_3: Table =
                            Table::new(vec![vec![FieldElement::ZERO; 4]; BACH_SIZE * 16]);
                        let mut current_trace_4: Table =
                            Table::new(vec![vec![FieldElement::ZERO; 31]; BACH_SIZE * 256]);

                        for i in 0..BACH_SIZE * 128 {
                            current_trace_1
                                .update_row_at(i, &trace_1.data[i + trace_id * BACH_SIZE * 128]);
                        }

                        for i in 0..BACH_SIZE * 256 {
                            current_trace_2
                                .update_row_at(i, &trace_2.data[i + trace_id * BACH_SIZE * 256]);
                        }

                        for i in 0..BACH_SIZE * 16 {
                            current_trace_3
                                .update_row_at(i, &trace_3.data[i + trace_id * BACH_SIZE * 16]);
                        }

                        for i in 0..BACH_SIZE * 256 {
                            current_trace_4
                                .update_row_at(i, &trace_4.data[i + trace_id * BACH_SIZE * 256]);
                        }

                        let trace: TraceTable = TraceTable::new(
                            current_trace_1,
                            current_trace_2,
                            current_trace_3,
                            current_trace_4,
                        );
                        // {
                        //     let mut tree = TREE.lock().unwrap();
                        //     let mut user_data = USER_DATA.lock().unwrap();
                        //     let mut user_data_list: Vec<(WalletAddress, UserData)> = user_data
                        //         .iter()
                        //         .map(|(address, user_data)| (*address, user_data.clone()))
                        //         .collect();
                        //     user_data_list.sort_by_key(|(_, user_data)| user_data.id);
                        //     let addiitonal_random_transactions = generate_random_transactions(
                        //         max(transaction_list.len().next_power_of_two(), BACH_SIZE)
                        //             - transaction_list.len(),
                        //         user_data_list,
                        //     );
                        //     transaction_list.extend_from_slice(&addiitonal_random_transactions);
                        //     trace = run_block(&mut tree, &mut user_data, transaction_list.clone());
                        //     txns.append(&mut transaction_list.clone());
                        // }
                        println!("trace generated succesfully with id {:?}", trace_id);
                        /**
                         * trace table sizes per transaction
                         * trace_table_1 (128 x 2)
                         * trace_table_2 (256 x 32)
                         * trace_table_3 (16 x 4)
                         * trace_table_4 (256 x 31)
                         */
                        // append trace.trace_1 to trace_1.txt file
                        // use std::fs::OpenOptions;
                        // use std::io::Write;

                        // let mut writer = std::io::BufWriter::new(
                        //     OpenOptions::new()
                        //         .create(true)
                        //         .append(true)
                        //         .open("trace_1.txt")
                        //         .unwrap(),
                        // );
                        // println!("saving trace_1 to file");
                        // for row in 0..trace.trace_1.num_of_rows() {
                        //     let field_concat = trace
                        //         .trace_1
                        //         .row_at(row)
                        //         .iter()
                        //         .map(|field| field.0.to_string())
                        //         .collect::<Vec<String>>()
                        //         .join(",");

                        //     writeln!(writer, "{}", field_concat).unwrap();
                        // }

                        // let mut writer = std::io::BufWriter::new(
                        //     OpenOptions::new()
                        //         .create(true)
                        //         .append(true)
                        //         .open("trace_2.txt")
                        //         .unwrap(),
                        // );
                        // println!("saving trace_2 to file");
                        // for row in 0..trace.trace_2.num_of_rows() {
                        //     let field_concat = trace
                        //         .trace_2
                        //         .row_at(row)
                        //         .iter()
                        //         .map(|field| field.0.to_string())
                        //         .collect::<Vec<String>>()
                        //         .join(",");

                        //     writeln!(writer, "{}", field_concat).unwrap();
                        // }

                        // let mut writer = std::io::BufWriter::new(
                        //     OpenOptions::new()
                        //         .create(true)
                        //         .append(true)
                        //         .open("trace_3.txt")
                        //         .unwrap(),
                        // );
                        // println!("saving trace_3 to file");
                        // for row in 0..trace.trace_3.num_of_rows() {
                        //     let field_concat = trace
                        //         .trace_3
                        //         .row_at(row)
                        //         .iter()
                        //         .map(|field| field.0.to_string())
                        //         .collect::<Vec<String>>()
                        //         .join(",");

                        //     writeln!(writer, "{}", field_concat).unwrap();
                        // }

                        // let mut writer = std::io::BufWriter::new(
                        //     OpenOptions::new()
                        //         .create(true)
                        //         .append(true)
                        //         .open("trace_4.txt")
                        //         .unwrap(),
                        // );
                        // println!("saving trace_4 to file");
                        // for row in 0..trace.trace_4.num_of_rows() {
                        //     let field_concat = trace
                        //         .trace_4
                        //         .row_at(row)
                        //         .iter()
                        //         .map(|field| field.0.to_string())
                        //         .collect::<Vec<String>>()
                        //         .join(",");

                        //     writeln!(writer, "{}", field_concat).unwrap();
                        // }
                        // println!("trace generated succesfully with id {:?}", trace_id);
                        // let url = "http://127.0.0.1:3030/health_check";
                        // let client = Client::new();
                        // let response = client.get(url).send();
                        let trace_data =
                            build_proof_request(trace_id, block_id.clone(), trace.clone());
                        // let url = "http://127.0.0.1:8080/proof/distribute_trace";
                        let url = format!("{}/proof/distribute_trace", MAIN_PROVER_HOST);
                        let response = HttpRequest::post(url.to_string(), trace_data);

                        match response.await {
                            Ok(res) => {
                                println!("server P* response is {:?}", res)
                            }
                            Err(err) => {
                                println!("server P* throws errors {:?}", err)
                            }
                        }
                        // let trace_data_1 = build_proof_request(trace[0].clone());
                        // let trace_data_2 = build_proof_request(trace[1].clone());
                        // let url_1 = "http://127.0.0.1:9090/proof/generate_proof";
                        // let url_2 = "http://127.0.0.1:9091/proof/generate_proof";
                        // let response_1 = HttpRequest::post(url_1.to_string(), trace_data_1);
                        // let response_2 = HttpRequest::post(url_2.to_string(), trace_data_2);

                        // match response_1.await {
                        //     Ok(res) => {
                        //         println!("server 1 response is {:?}", res)
                        //     }
                        //     Err(err) => {
                        //         println!("server 1 throws errors {:?}", err)
                        //     }
                        // }
                        // match response_2.await {
                        //     Ok(res) => {
                        //         println!("server 2 response is {:?}", res)
                        //     }
                        //     Err(err) => {
                        //         println!("server 2 throws errors {:?}", err)
                        //     }
                        // }
                    }
                    let creation_td = start_time.elapsed().as_millis();
                    drop_transactions(txns.clone());
                    let mut txn_ids = vec![];
                    let mut amount = 0;
                    let mut transactions = vec![];
                    for txn in txns {
                        txn_ids.push(txn.id);
                        amount += txn.amount;
                        transactions.push(Transaction {
                            to: txn.to,
                            from: txn.from,
                            amount: txn.amount,
                            id: get_new_transaction_id(),
                            age_ts: age_ts,
                            creation_td: creation_td,
                            block_id: block_id.clone(),
                        })
                    }

                    {
                        let db = PERSISTANT_DB.lock().unwrap();
                        //db.add_trace(block_data.clone());
                        db.add_transactions(transactions);
                        {
                            db.upsert_user_data_bulk(USER_DATA.lock().unwrap().clone());
                        }
                    }

                    println!("Block generated succesfully {block_id}");

                    // Set the running flag back to false
                    {
                        let mut is_running = IS_BLOCK_RUNNING.lock().unwrap();
                        *is_running = false;
                        drop(is_running);
                    }
                });

                true
            }
            Err(_) => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpRequest {}

impl HttpRequest {
    pub async fn post(url: String, body: SendProofRequest) -> Result<String, GenericError> {
        let client = Client::new();
        let response = client.post(url.clone()).json(&body).send().await.unwrap();
        if response.status().is_success() {
            let body = response.text().await.unwrap();
            Ok(body)
        } else {
            let status_code = response.status().as_u16();
            Err(GenericError {
                display_msg: "Error".to_string(),
                error_code: status_code.clone(),
                log_msg: format!("Error while calling {url} | Status Code : {status_code}"),
            })
        }
    }
}
pub fn build_proof_request(
    trace_id: usize,
    block_id: String,
    trace: TraceTable,
) -> SendProofRequest {
    let trace_data = SendProofRequest {
        trace_id,
        block_id,
        trace: trace.clone(),
        public_inputs: vec![
            trace.trace_2.cell_at(NO_OF_ROWS_PER_TRANSACTION - 1, 4),
            trace.trace_2.cell_at(trace.trace_2.num_of_rows() - 1, 28),
        ],
    };
    trace_data
}

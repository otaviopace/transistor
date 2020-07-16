use edn_rs::{
    ser_struct,
    Serialize,
    parse_edn
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct CruxId(String);

impl Serialize for CruxId {
    fn serialize(mut self) -> String {
        self.0.insert(0, ':');
        format!("{}", self.0)
    }
}

impl CruxId {
    pub fn new(id: &str) -> Self {
        Self {0: id.to_string()}
    }
}

ser_struct!{
    #[derive(Debug, PartialEq)]
    #[allow(non_snake_case)]
    /// Definition for the response of the `state` endpoint
    pub struct StateResponse {
        index___index_version: usize,
        doc_log___consumer_state: Option<String>,
        tx_log___consumer_state:  Option<String>,
        kv___kv_store: String,
        kv___estimate_num_keys: usize,
        kv___size: usize
    }
}

impl StateResponse {
    pub fn deserialize(resp: String) -> StateResponse {
        let edn = parse_edn(&resp).unwrap();
        StateResponse {
            index___index_version: edn[":crux.index/index-version"].to_string().parse::<usize>().unwrap_or(0usize),
            doc_log___consumer_state: nullable_str(edn[":crux.doc-log/consumer-state"].to_string()),
            tx_log___consumer_state:  nullable_str(edn[":crux.tx-log/consumer-state"].to_string()),
            kv___kv_store: edn[":crux.kv/kv-store"].to_string().replace("\"", ""),
            kv___estimate_num_keys: edn[":crux.kv/estimate-num-keys"].to_string().parse::<usize>().unwrap_or(0usize),
            kv___size: edn[":crux.kv/size"].to_string().parse::<usize>().unwrap_or(0usize),
        }
    }

    #[cfg(test)]
    pub fn default() -> StateResponse{
        StateResponse {
            index___index_version: 5usize,
            doc_log___consumer_state: None,
            tx_log___consumer_state:  None,
            kv___kv_store: String::from("crux.kv.rocksdb.RocksKv"),
            kv___estimate_num_keys: 34usize,
            kv___size: 88489usize,
        }
    }
}

ser_struct!{
    #[derive(Debug, PartialEq)]
    #[allow(non_snake_case)]
    /// Definition for the response of the `state` endpoint
    pub struct TxLogResponse {
        tx___tx_id: usize, 
        tx___tx_time: String
    }
}

impl TxLogResponse {
    pub fn deserialize(resp: String) -> Self {
        let edn = parse_edn(&resp).unwrap();
        Self {
            tx___tx_id: edn[":crux.tx/tx-id"].to_string().parse::<usize>().unwrap(), 
            tx___tx_time: edn[":crux.tx/tx-time"].to_string()
        }
    }
}

fn nullable_str(s: String) -> Option<String> {
    if s.contains("nil") {
        None
    } else {
        Some(s)
    }
}

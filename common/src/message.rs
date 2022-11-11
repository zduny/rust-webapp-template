pub mod server {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Message {
        Init {
            user_name: String,
            connected_user_names: Vec<String>,
        },
        UserConnected {
            user_name: String,
        },
        UserDisconnected {
            user_name: String,
        },
        Message {
            user_name: String,
            content: String,
        },
    }
}

pub mod client {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Message {
        pub content: String,
    }
}

pub mod worker {
    use num_bigint::BigUint;
    use serde::{Deserialize, Serialize};

    pub mod host {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub enum Message {
            CalculateFactorial { input: u128 },
            CalculateFibonacci { index: u128 },
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Message {
        ResultFactorial {
            output: BigUint,
        },
        ResultFibonacci {
            output: BigUint,
        },
    }
}

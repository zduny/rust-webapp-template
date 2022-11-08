pub mod server {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
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

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Message {
        content: String,
    }
}

pub mod worker {
    use serde::{Deserialize, Serialize};

    use crate::FibonacciInputError;

    pub mod host {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub enum Message {
            CalculateFactorial { input: u128 },
            CalculateFibonacci { index: u128 },
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Message {
        ResultFactorial {
            output: Result<u128, FibonacciInputError>,
        },
        ResultFibonacci {
            output: u128,
        },
    }
}

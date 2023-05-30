struct Token {
    token_id: String,
    token_symbol: String,
    token_name: String,
    total_supply: u64,
    owner_address: String,
    total_balance: u64,
}

struct User {
    user_address: String,
    total_balance: u64,
}

struct TokenDatabase {
    tokens: Vec<Token>,
    users: Vec<User>,
}

impl TokenDatabase {
    fn new() -> Self {
        TokenDatabase {
            tokens: Vec::new(),
            users: Vec::new(),
        }
    }

    fn mint_token(
        &mut self,
        token_id: String,
        token_symbol: String,
        token_name: String,
        total_supply: u64,
        owner_address: String,
    ) {
        let token = Token {
            token_id: token_id.clone(),
            token_symbol: token_symbol.clone(),
            token_name: token_name.clone(),
            total_supply,
            owner_address: owner_address.clone(),
            total_balance: total_supply,
        };
        self.tokens.push(token);

        let user = User {
            user_address: owner_address,
            total_balance: total_supply,
        };
        self.users.push(user);
    }

    fn get_token_details(&self, token_id: &str) -> Option<&Token> {
        self.tokens.iter().find(|token| token.token_id == token_id)
    }

    fn transfer_token(&mut self, token_id: &str, receiver_address: &str, amount: u64) {
        if amount > 0 {
            if let Some(sender) = self.tokens.iter_mut().find(|token| token.token_id == token_id) {
                if sender.total_balance >= amount {
                    sender.total_balance -= amount;
                    if let Some(receiver) = self
                        .users
                        .iter_mut()
                        .find(|user| user.user_address == receiver_address)
                    {
                        receiver.total_balance += amount;
                    } else {
                        let user = User {
                            user_address: receiver_address.to_string(),
                            total_balance: amount,
                        };
                        self.users.push(user);
                    }
                } else {
                    println!("Insufficient balance");
                }
            } else {
                println!("No such sender identified");
            }
        } else {
            println!("Amount must be greater than zero");
        }
    }

    fn get_balance(&self, user_address: &str) -> Option<&User> {
        self.users.iter().find(|user| user.user_address == user_address)
    }
}

fn main() {
    let mut token_db = TokenDatabase::new();

    token_db.mint_token(
        String::from("TK1"),
        String::from("ANU"),
        String::from("Anurag"),
        1000,
        String::from("0x1df456fh55hjd88893h58"),
    );

    let token_id = "TK1";
    if let Some(token) = token_db.get_token_details(token_id) {
        println!(
            "Token Id: {}, Token Symbol: {}, Token Name: {}, Token Total Supply: {}, Token Owner Address: {}, Token Total Balance: {}",
            token.token_id, token.token_symbol, token.token_name, token.total_supply, token.owner_address, token.total_balance
        );
    } else {
        println!("No such token listed");
    }

    let receiver_address = "0x8d7s";
    token_db.transfer_token(token_id, receiver_address, 100);
    if let Some(user) = token_db.get_balance(receiver_address) {
        println!("User Address: {}, User total balance: {}", user.user_address, user.total_balance);
    } else {
        println!("No such user found");
    }
    
       let token_id = "TK1";
    if let Some(token) = token_db.get_token_details(token_id) {
        println!(
            "Token Total Balance: {}", token.total_balance
        );
    } else {
        println!("No such token listed");
    }
}    

struct Token {
    tokenId: String,
    tokenSymbol: String,
    tokenName: String,
    totalSupply: u64,
    ownerAddress: String,
    totalBalance: u64,
}

struct User {
    userId: String,
    userAddress: String,
    totalBalance: u64,
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
        tokenId: String,
        tokenSymbol: String,
        tokenName: String,
        totalSupply: u64,
        ownerAddress: String,
    ) {
        let token = Token {
            tokenId: tokenId.clone(),
            tokenSymbol: tokenSymbol.clone(),
            tokenName: tokenName.clone(),
            totalSupply,
            ownerAddress: ownerAddress.clone(),
            totalBalance: totalSupply,
        };
        self.tokens.push(token);

        let user = User {
            userId: tokenId,
            userAddress: ownerAddress,
            totalBalance: totalSupply,
        };
        self.users.push(user);
    }

    fn get_token_details(&self, tokenId: &str) -> Option<&Token> {
        self.tokens.iter().find(|token| token.tokenId == tokenId)
    }

    fn transfer_token(&mut self, tokenId: &str, receiverAddress: &str, amount: u64) {
        if amount > 0 {
            if let Some(sender) = self.tokens.iter_mut().find(|token| token.tokenId == tokenId) {
                if sender.totalBalance >= amount {
                    sender.totalBalance -= amount;
                    if let Some(receiver) = self
                        .users
                        .iter_mut()
                        .find(|user| user.userAddress == receiverAddress)
                    {
                        receiver.totalBalance += amount;
                    } else {
                        let user = User {
                            userId: receiverAddress.to_string(),
                            userAddress: receiverAddress.to_string(),
                            totalBalance: amount,
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

    fn get_balance(&self, userAddress: &str) -> Option<&User> {
        self.users.iter().find(|user| user.userAddress == userAddress)
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

    let tokenId = "TK1";
    if let Some(token) = token_db.get_token_details(tokenId) {
        println!(
            "Token Id: {}, Token Symbol: {}, Token Name: {}, Token Total Supply: {}, Token Owner Address: {}, Token Total Balance: {}",
            token.tokenId, token.tokenSymbol, token.tokenName, token.totalSupply, token.ownerAddress, token.totalBalance
        );
    } else {
        println!("No such token listed");
    }

    let receiverAddress = "0x8d7s";
    token_db.transfer_token(tokenId, receiverAddress, 100);
    if let Some(user) = token_db.get_balance(receiverAddress) {
        println!("User Address: {}, User total balance: {}", user.userAddress, user.totalBalance);
    } else {
        println!("No such user found");
    }
    
       let tokenId = "TK1";
    if let Some(token) = token_db.get_token_details(tokenId) {
        println!(
            "Token Total Balance: {}", token.totalBalance
        );
    } else {
        println!("No such token listed");
    }
}    

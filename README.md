# Whitelist-Gated Token Sale Program

This program allows users to participate in a whitelist-gated token sale where only whitelisted addresses can purchase tokens at a static price, with a limit on the number of tokens each wallet can buy. The program is built using Anchor, a framework for Solana smart contracts.

## Features

- **Whitelist-Gated Access**: Only addresses on the whitelist can participate in the token sale.
- **Static Token Price**: The price of the token remains constant throughout the sale.
- **Purchase Limit Per Wallet**: Each wallet address has a maximum number of tokens it can purchase.

## Instructions

### 1. Initialize the Sale

Initialize the sale by calling the `initialize` function with the following parameters:

- `admin`: The public key of the admin account.
- `token_to_sol_price`: The price of one token in lamports.
- `token_scale`: The scale of the token price.
- `max_tokens_per_buyer`: The maximum number of tokens each wallet can buy.
- `tokens_available_for_sale`: The total number of tokens available for sale.

### 2. Add Addresses to Whitelist

Admin can add addresses to the whitelist by calling the `add_address_to_whitelist` function with the address to be whitelisted.

### 3. Buy Tokens

Whitelisted users can purchase tokens by calling the `buy_token_from_sale` function with the amount of tokens they want to buy. The total cost will be calculated based on the static token price.

## Error Codes

- **Unauthorized**: Attempt to perform an admin-only action without proper authorization.
- **PDAFull**: The PDA (Program Derived Address) is full and cannot add more addresses.
- **PDAListEmpty**: The PDA list is empty.
- **NotWhitelisted**: Attempt to buy tokens by a non-whitelisted address.
- **SaleInactive**: The token sale is not in progress.
- **AmountExceedsMax**: Purchase amount exceeds the maximum tokens allowed per wallet.
- **PrevCurrentAmountExceedsMax**: Previous amount bought plus current amount exceeds the maximum tokens allowed per wallet.

## Deployment

Deploy the program to the Solana blockchain using Anchor, and interact with it using the provided instructions.

### Example Commands

#### Initialize the Sale
```rust
let tx = await program.rpc.initialize(
    admin_public_key,
    token_to_sol_price,
    token_scale,
    max_tokens_per_buyer,
    tokens_available_for_sale,
    {
        accounts: {
            tokenToSolPrice: token_to_sol_price_account,
            whitelist: whitelist_account,
            whitelistPdas: whitelist_pdas_account,
            tokenSaleDetails: token_sale_details_account,
            admin: admin_account,
            systemProgram: anchor.web3.SystemProgram.programId,
        },
    }
);

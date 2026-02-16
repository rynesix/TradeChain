// use super::ipfs::CreateTokenMetadata;

pub struct TokenCollection {
    pub tokens: Vec<CreateTokenMetadata>,
}

use std::sync::LazyLock;

use pumpfun::utils::CreateTokenMetadata;

pub static TOKEN_COLLECTION: LazyLock<TokenCollection> = LazyLock::new(|| TokenCollection {
    tokens: vec![
        CreateTokenMetadata {
            name: "Solana Cat".to_string(),
            symbol: "SCAT".to_string(),
            description: "A playful cat bringing liquidity to Solana one paw at a time."
                .to_string(),
            file: "./images/cat.png".to_string(),
            twitter: Some("https://x.com/xetonsol".to_string()),
            telegram: Some("https://t.me/solanacat".to_string()),
            website: Some("https://solanacat.fun".to_string()),
        },
        CreateTokenMetadata {
            name: "Giga Chad Coin".to_string(),
            symbol: "CHAD".to_string(),
            description: "For those who buy the top and still make it. Only Chads allowed."
                .to_string(),
            file: "./images/chad.png".to_string(),
            twitter: Some("https://x.com/xetonsol".to_string()),
            telegram: Some("https://t.me/gigachadcoin".to_string()),
            website: Some("https://gigachad.sol".to_string()),
        },
        CreateTokenMetadata {
            name: "Pepe Returns".to_string(),
            symbol: "PEPER".to_string(),
            description: "The legendary frog is back. Memes are forever.".to_string(),
            file: "./images/pepe.png".to_string(),
            twitter: Some("https://x.com/xetonsol".to_string()),
            telegram: Some("https://t.me/pepereturns".to_string()),
            website: Some("https://pepereturns.xyz".to_string()),
        },
        CreateTokenMetadata {
            name: "Quantum Banana".to_string(),
            symbol: "QBN".to_string(),
            description: "The only banana that exists in all parallel blockchains at once."
                .to_string(),
            file: "./images/banana.png".to_string(),
            twitter: Some("https://x.com/xetonsol".to_string()),
            telegram: Some("https://t.me/quantumbanana".to_string()),
            website: Some("https://quantumbanana.io".to_string()),
        },
        CreateTokenMetadata {
            name: "Rustacean".to_string(),
            symbol: "RUST".to_string(),
            description: "Token for real builders who ship in Rust. Crab included.".to_string(),
            file: "./images/rustacean.png".to_string(),
            twitter: Some("https://x.com/xetonsol".to_string()),
            telegram: Some("https://t.me/rustaceansol".to_string()),
            website: Some("https://rustacean.build".to_string()),
        },
        CreateTokenMetadata {
            name: "Pumpkin Pie".to_string(),
            symbol: "PIE".to_string(),
            description: "A deliciously seasonal token for degens who love autumn.".to_string(),
            file: "./images/pumpkin_pie.png".to_string(),
            twitter: Some("https://x.com/xetonsol".to_string()),
            telegram: Some("https://t.me/pumpkinpiecoin".to_string()),
            website: Some("https://pumpkinpie.lol".to_string()),
        },
        CreateTokenMetadata {
            name: "LFG Inu".to_string(),
            symbol: "LFGI".to_string(),
            description: "LFG, we go to the moon! Another legendary dog meme on Solana."
                .to_string(),
            file: "./images/lfg_inu.png".to_string(),
            twitter: Some("https://x.com/xetonsol".to_string()),
            telegram: Some("https://t.me/lfginu".to_string()),
            website: Some("https://lfginu.com".to_string()),
        },
        CreateTokenMetadata {
            name: "FOMO Apes".to_string(),
            symbol: "FOMO".to_string(),
            description: "The more you FOMO, the more bananas you get. That’s the deal."
                .to_string(),
            file: "./images/fomo_apes.png".to_string(),
            twitter: Some("https://x.com/xetonsol".to_string()),
            telegram: Some("https://t.me/fomoapes".to_string()),
            website: Some("https://fomoapes.wtf".to_string()),
        },
        CreateTokenMetadata {
            name: "Serious Stablecoin".to_string(),
            symbol: "SERIOUS".to_string(),
            description: "We promise not to rug. Pinky swear. Your money is safe (maybe)."
                .to_string(),
            file: "./images/serious_stablecoin.png".to_string(),
            twitter: Some("https://x.com/xetonsol".to_string()),
            telegram: Some("https://t.me/seriousstablecoin".to_string()),
            website: Some("https://seriousstablecoin.com".to_string()),
        },
        CreateTokenMetadata {
            name: "Kraken of Solana".to_string(),
            symbol: "KRAKEN".to_string(),
            description: "Unleash the tentacles! The deep sea monster of Solana DeFi.".to_string(),
            file: "./images/kraken.png".to_string(),
            twitter: Some("https://x.com/xetonsol".to_string()),
            telegram: Some("https://t.me/solanakraken".to_string()),
            website: Some("https://kraken.sol".to_string()),
        },
    ],
});

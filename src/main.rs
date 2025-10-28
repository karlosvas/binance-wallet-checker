mod models;

use {
    crate::models::{AccountResponse, Transaction},
    colored::*,
    csv::{Reader, ReaderBuilder},
    dotenv::dotenv,
    hmac::{Hmac, Mac},
    reqwest::blocking::Client,
    sha2::Sha256,
    std::{
        collections::HashMap,
        env,
        error::Error,
        fs::File,
        time::{SystemTime, UNIX_EPOCH},
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr: Reader<File> = ReaderBuilder::new().has_headers(true).from_path(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/csv/account-statement_2025-01-01_2025-10-28_es-es_7c4848.csv"
    ))?;

    let binance_payments: Vec<Transaction> = rdr
        .deserialize()
        .filter_map(|r| r.ok())
        .filter(|t: &Transaction| t.descripcion.to_lowercase().contains("binance"))
        .collect();

    let total: f64 = binance_payments.iter().map(|tx| tx.importe.abs()).sum();

    let mut por_mes: HashMap<String, f64> = HashMap::new();
    for payment in &binance_payments {
        *por_mes
            .entry(payment.fecha_inicio[..7].to_string())
            .or_insert(0.0) += payment.importe.abs();
    }

    println!(
        "{} {}",
        "Binance transactions:".cyan().bold(),
        binance_payments.len().to_string().yellow()
    );
    println!(
        "{} {}",
        "Total spent:".cyan().bold(),
        format!("€{:.2}", total).red().bold()
    );

    println!("\n{}", "Monthly breakdown:".green().bold());
    let mut meses: Vec<_> = por_mes.keys().collect();
    meses.sort();
    for mes in meses {
        println!(
            "  {}: {}",
            mes.bright_blue(),
            format!("€{:.2}", por_mes[mes]).yellow()
        );
    }

    dotenv().ok();
    let (api_key, secret) = match (env::var("BINANCE_API_KEY"), env::var("BINANCE_SECRET")) {
        (Ok(k), Ok(s)) => (k, s),
        _ => {
            println!(
                "\n{}",
                "Binance credentials not set, skipping balances".yellow()
            );
            return Ok(());
        }
    };

    let account: AccountResponse = fetch_account(&api_key, &secret)?;
    let total_wallet: f64 = match calculate_portfolio_value(&account) {
        Ok(value) => value,
        Err(e) => {
            println!("\n{} {}", "Error calculating portfolio value:".red(), e);
            return Ok(());
        }
    };

    if total_wallet == total {
        println!(
            "\n{}",
            "Total spent matches the current portfolio value."
                .green()
                .bold()
        );
    } else if total_wallet > total {
        println!(
            "\n{} {}",
            "Portfolio has increased in value:".green().bold(),
            format!("€{:.2}", (total_wallet as f64) - total)
                .green()
                .bold()
        );
    } else {
        println!(
            "\n{} {}",
            "Portfolio has decreased in value:".red().bold(),
            format!("€{:.2}", total - (total_wallet as f64))
                .red()
                .bold()
        );
    }

    Ok(())
}

fn fetch_account(api_key: &str, secret: &str) -> Result<AccountResponse, Box<dyn Error>> {
    let timestamp_ms = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    let query = format!("timestamp={}&recvWindow=5000", timestamp_ms);

    let mut mac: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes())?;
    mac.update(query.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());

    let url = format!(
        "https://api.binance.com/api/v3/account?{}&signature={}",
        query, signature
    );

    let resp = Client::builder()
        .user_agent("data_analysis/0.1")
        .build()?
        .get(&url)
        .header("X-MBX-APIKEY", api_key)
        .send()?;

    if !resp.status().is_success() {
        return Err(format!("Error {}: {}", resp.status(), resp.text()?).into());
    }

    Ok(resp.json()?)
}

fn calculate_portfolio_value(account: &AccountResponse) -> Result<f64, Box<dyn Error>> {
    let client: Client = Client::new();
    let mut total_eur: f64 = 0.0;

    println!("\n{}", "Portfolio value in EUR:".green().bold());

    for balance in &account.balances {
        let amount: f64 = balance.free.parse::<f64>().unwrap_or(0.0)
            + balance.locked.parse::<f64>().unwrap_or(0.0);

        if amount <= 0.0 {
            continue;
        }

        let asset: &str = balance.asset.trim_start_matches("LD");
        let symbol: String = format!("{}EUR", asset);
        let url: String = format!(
            "https://api.binance.com/api/v3/ticker/price?symbol={}",
            symbol
        );

        if let Ok(data) = client.get(&url).send()?.json::<serde_json::Value>() {
            if let Some(price_str) = data["price"].as_str() {
                let price: f64 = price_str.parse()?;
                let value: f64 = amount * price;
                total_eur += value;
                println!(
                    "  {}: {} × €{:.2} = €{:.2}",
                    asset.bright_blue(),
                    amount,
                    price,
                    value
                );
            }
        }
    }

    println!(
        "\n{} {}",
        "Total portfolio:".cyan().bold(),
        format!("€{:.2}", total_eur).green().bold()
    );

    Ok(total_eur)
}

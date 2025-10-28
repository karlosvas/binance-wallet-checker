# Binance Portfolio Tracker

Rust CLI application to monitor Binance investment gains/losses with automated Telegram reports.

## Features

- 📊 Transaction analysis from bank CSV statements
- 💰 Total investment vs current portfolio value calculation
- 📈 Real-time valuation via Binance API
- 📱 Automated Telegram reports (weekly/monthly)
- 🎨 Colorized terminal output

## How It Works

1. Parses bank statements (CSV) to calculate total deposits
2. Fetches current balances from Binance via API
3. Gets EUR prices for each crypto
4. Calculates ROI: `(current_value - total_investment) / total_investment * 100`
5. Sends periodic reports to Telegram with key metrics

## Setup

1. Create a `csv/` folder in the project root and place your bank statement CSV files there
2. Create a `.env` file with your credentials:

```env
BINANCE_API_KEY=your_api_key_here
BINANCE_SECRET=your_secret_key_here
TELEGRAM_BOT_TOKEN=your_bot_token_here
TELEGRAM_CHAT_ID=your_chat_id_here
```

## Stack

- Rust + reqwest + serde + csv
- Binance API (account + ticker endpoints)
- Telegram Bot API

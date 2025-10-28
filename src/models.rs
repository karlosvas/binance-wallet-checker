use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "Tipo")]
    pub tipo: String,
    #[serde(rename = "Producto")]
    pub producto: String,
    #[serde(rename = "Fecha de inicio")]
    pub fecha_inicio: String,
    #[serde(rename = "Fecha de finalización")]
    pub fecha_fin: String,
    #[serde(rename = "Descripción")]
    pub descripcion: String,
    #[serde(rename = "Importe")]
    pub importe: f64,
    #[serde(rename = "Comisión")]
    pub comision: f64,
    #[serde(rename = "Divisa")]
    pub divisa: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Saldo")]
    pub saldo: f64,
}

// Estructura mínima para deserializar balances
// Minimal structure to deserialize balances
#[derive(Debug, serde::Deserialize)]
pub struct Balance {
    pub asset: String,
    pub free: String,
    pub locked: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct AccountResponse {
    pub balances: Vec<Balance>,
}

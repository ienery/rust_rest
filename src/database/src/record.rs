#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    user_id: String,
    point_id: String,
	period_year: String,
	period_month: String,
	readings: String,
    send_date_timestamp: String,
    period_timestamp: String
}
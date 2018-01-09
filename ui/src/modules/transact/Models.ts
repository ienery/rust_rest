/**
 * Интерфейс для записи транзакции.
 */
export interface IRecord {
    user_id: String,
    point_id: String,
    period_year: String,
    period_month: String,
    readings: String,
    send_date_time: String
}
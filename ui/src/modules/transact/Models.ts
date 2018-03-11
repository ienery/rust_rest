/**
 * Интерфейс для записи.
 * 
 * @prop {string} user_id Идентификатор пользователя.
 * @prop {string} point_id Идентификатор точки учета.
 * @prop {string} period_year Идентификатор периода - год.
 * @prop {string} period_month Идентификатор периода - месяц.
 * @prop {string} readings Данные учета.
 * @prop {string} send_date_timestamp Временная метка отправки.
 * @prop {string} period_timestamp Временная метка периода год+месяц.
 */
export interface IRecord {
    user_id: string;
    point_id: string;
    period_year: string;
    period_month: string;
    readings: string;
    send_date_timestamp: string;
    period_timestamp: string
}

/**
 * Интерфес для транзакции.
 * 
 * @prop {string} parent_transact_id Идентификатор родительской транзакции.
 * @prop {string} transact_id Идентификатор транзакции.
 * @prop {IRecord} record Запись в транзакции.
 * @prop {number} timestamp Временной штамп транзакции.
 */
export interface ITransact {
    parent_transact_id: string;
    transact_id: string;
    record: IRecord;
    timestamp: string;
}

/** 
 * Стандартный ответ сервера.
 * 
 * @prop {T} body Тело ответа.
 * @prop {boolean} success Признак учпешного ответа.
 */
export interface IBackendResponse<T> {
    body: T;
    success: boolean
}
/**
 * Статусы запросов.
 * 
 * IDLE - Состояние ожидания.
 * LOADING - Загрузка.
 * SUCCESS - Запрос выполнен успешно.
 * FAILURE - Ошибка.
 */
export enum EStatusResponse {
    IDLE = <any>'IDLE',
    LOADING = <any>'LOADING',
    SUCCESS = <any>'SUCCESS',
    FAILURE = <any>'FAILURE',
}
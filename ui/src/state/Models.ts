import {EStatusResponse} from './Enums';
import {ITransact} from '../modules/transact/Models';

/**
 * Главный стейт приложения.
 * 
 * @prop {any} transacts Транзакции.
 */
export interface IAppState {
    transacts: ITransactsState
};

/**
 * Стейт Транзакция приложения.
 * 
 * @prop {any} transacts Транзакции.
 */
export interface ITransactsState {
    data: ITransact[];
    status: EStatusResponse
};
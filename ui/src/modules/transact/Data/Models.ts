import {EStatusResponse} from '../../../Data/Enums';
import {ITransact} from '../Models';

/**
 * Стейт Транзакция приложения.
 * 
 * @prop {any} transacts Транзакции.
 */
export interface ITransactsState {
    data: ITransact[];
    status: EStatusResponse
};
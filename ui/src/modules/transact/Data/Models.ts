import {EStatusResponse} from '../../../Data/Enums';
import {ITransact} from '../Models';
import { IBlock } from '../../Block/Models';

/**
 * Стейт Транзакция приложения.
 * 
 * @prop {ITransact[]} transacts Транзакции.
 * @prop {EStatusResponse} status Статус запроса.
 * @prop {IBlock} block Блок транзакций.
 */
export interface ITransactsState {
    transacts: ITransact[];
    status: EStatusResponse;
    block: IBlock;
};
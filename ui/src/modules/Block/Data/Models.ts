import {EStatusResponse} from '../../../Data/Enums';
import {IBlock} from '../Models';

/**
 * Стейт Транзакция приложения.
 * 
 * @prop {IBlock[]} blocks Блок транзакций.
 * @prop {EStatusResponse} status Статус запроса.
 */
export interface IBlockState {
    blocks: IBlock[];
    status: EStatusResponse;
};

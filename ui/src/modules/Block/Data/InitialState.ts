import {EStatusResponse} from '../../../Data/Enums';
import {IBlockState} from './Models';

/** 
 * Начальное состояние стора блоков. 
 */
export const blocksInitialState: IBlockState = {
    blocks: null,
    status: EStatusResponse.IDLE
}
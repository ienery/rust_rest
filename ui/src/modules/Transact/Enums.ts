/**
 * Тип предыдущей страницы откуда был переход.
 * 
 * CREATE - Со страницы создания транзакции.
 * NO_BLOCK - Со страницы транзакций без блока.
 * BLOCK - Со страницы блока.
 */
export enum EPreviousPage {
    CREATE = <any>'create',
    NO_BLOCK = <any>'NO_BLOCK',
    BLOCK = <any>'block'
}

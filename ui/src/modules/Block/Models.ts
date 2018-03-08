import { ITransact } from "../transact/Models";

/**
 * Интерфейс для блока.
 * 
 * @prop {string} block_id Идентификатор блока.
 * @prop {string} parent_block_id Идентификатор родительского блока.
 * @prop {ITransact[]} transacts Транзакции в блоке.
 * @prop {string} timestamp Временной штамп блока.
 * @prop {string} block_no Порядковый номер блока.
 */
export interface IBlock {
    block_id: string,
    parent_block_id: string,
    transacts: ITransact[],
    timestamp: string,
    block_no: string
}
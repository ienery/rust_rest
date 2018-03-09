import axios from 'axios';

import { IBlock } from '../Models';
import { ITransact } from '../../transact/Models';

/**
 * Создание блока из списка свободных транзакций.
 *
 */
export async function createBlock () {
    try {
        const response = await axios.post('/rest/block/create');
        //console.debug('response', response);
        if (response.data.success === true) {
            return response.data.body;
        }

        return false;
    } catch (error) {
        return false;
    } 
}

/**
 * Чтение данных блока.
 * 
 * @param {string} block_id Идентификатор блока.
 */
export async function readBlock (block_id: string) {
    try {
        const response = await axios.post('/rest/block/read', {
            block_id
        });
        //console.debug('response', response);
        if (response.data.success === true) {
            return response.data.body;
        }

        return false;
    } catch (error) {
        return false;
    } 
}

/**
 * Чтение данных всех блоков.
 */
export async function readBlocks () {
    try {
        const response = await axios.post('/rest/blocks/read');
        if (response.data.success === true) {
            return response.data.body;
        }

        return false;
    } catch (error) {
        return false;
    } 
}


import * as React from 'react';
import { List } from 'antd';
import { Link } from 'react-router-dom';

import { IBlock } from '../../Models';
/**
 * Свойства компонента.
 * 
 * @prop {IBlock} item Элемент транзакция.
 */
interface IPropsDispatch {
    item: IBlock;
}

/** Свойства компонента. */
type IProps = IPropsDispatch;

/** 
 * Компонент просмотра транзакции в списке.
 */
export class BlockListItem extends React.Component<IProps, {}> {
    static displayName = 'BlockListItem';

    /**
     * Рендер части записи блока.
     * 
     * @param {string} block_no Глмер блока.
     */
    renderBlock = (block_no: string): JSX.Element => {
        return (
            <div>
                {block_no}
            </div>
        );
    }

    render() {
        const { 
            item: {
                block_id,
                block_no,
                timestamp,
            }
        } = this.props;

        let search = `?blockId=${block_id}`;

        return (
            <List.Item>
                <List.Item.Meta
                    title={
                        <Link 
                            to={{
                                pathname: '/transacts',
                                search
                            }}
                        >
                            {block_id}
                        </Link>
                    }
                    description={timestamp}
                />
                {this.renderBlock(block_no)}
            </List.Item>
        );
    }
}
import * as React from 'react';
import { List } from 'antd';
import { Link } from 'react-router-dom';

import {ITransact, IRecord} from '../../Models';
import {EPreviousPage} from '../../Enums';
/**
 * Свойства компонента.
 * 
 * @prop {ITransact} item Элемент транзакция.
 * @prop {Function} push Работа с роутом.
 * @prop {string} [blockId] Идентификатор блока.
 */
interface IPropsDispatch {
    item: ITransact;
    push: any;
    blockId?: string;
}

/** Свойства компонента. */
type IProps = IPropsDispatch;

/** 
 * Компонент просмотра транзакции в списке.
 */
export class TransactListItem extends React.Component<IProps, {}> {
    static displayName = 'TransactListItem';

    /**
     * Рендер части записи транзакции.
     */
    renderRecord = (record: IRecord): JSX.Element => {
        return (
            <div>
                {record.point_id}
            </div>
        );
    }

    render() {
        const {
            blockId, 
            item: {
                transact_id,
                timestamp,
                record
            }
        } = this.props;

        let search = `?transactId=${transact_id}`;
        if (blockId) {
            search += `&blockId=${blockId}`;
        } else {
            search += `&previous=${EPreviousPage.NO_BLOCK}`;
        }

        return (
            <List.Item>
                <List.Item.Meta
                    title={
                        <Link 
                            to={{
                                pathname: '/transact-details',
                                search
                            }}
                        >
                            {transact_id}
                        </Link>
                    }
                    description={timestamp}
                />
                {this.renderRecord(record)}
            </List.Item>
        );
    }
}
import * as React from 'react';
import { List } from 'antd';
import { Link } from 'react-router-dom';

import {ITransact, IRecord} from '../../Models';

/**
 * Свойства компонента.
 * 
 * @prop {ITransact} item Элемент транзакция.
 * @prop {Function} push Работа с роутом.
 */
interface IPropsDispatch {
    item: ITransact;
    push: any;
}

/** Свойства компонента. */
type IProps = IPropsDispatch;

/** 
 * Компонент просмотра транзакции в списке.
 */
export class TransactListItem extends React.Component<IProps, {}> {
    static displayName = 'TransactListItem';

    /**
     * Обработчик клика на элементе.
     * 
     * @deprecated
     */
    handleClickItem = () => {
        this.props.push({
            pathname: '/transact-details',
            search: `?transactId=${this.props.item.transact_id}`
        });
    }

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
        const {item} = this.props;

        return (
            <List.Item>
                <List.Item.Meta
                    title={
                        <Link 
                            to={{
                                pathname: '/transact-details',
                                search: `?transactId=${item.transact_id}`
                            }}
                        >
                            {item.transact_id}
                        </Link>
                    }
                    description={item.timestamp}
                />
                {this.renderRecord(item.record)}
            </List.Item>
        );
    }
}
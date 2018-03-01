import * as React from 'react';

import {ITransact} from '../../Models';

/**
 * Свойства компонента.
 * 
 * @prop {ITransact} item Элемент транзакция.
 * @prop {Function} push Работа с роутом.
 */
interface IProps {
    item: ITransact;
    push: any;
}

/** 
 * Компонент просмотра транзакции в списке.
 */
export class TransactListItem extends React.Component<IProps, {}> {
    static displayName = 'TransactListItem';

    /**
     * Обработчик клика на элементе.
     */
    handleClickItem = () => {
        this.props.push({
            pathname: '/transact-details',
            search: `?transactId=${this.props.item.transact_id}`
          });
    }

    render () {
        const {item} = this.props;

        return(
            <li onClick={this.handleClickItem}>
                {item.transact_id}
                {/* {item.parent_transact_id}
                {item.record.period_year}
                {item.record.period_month}
                {item.record.readings} */}
            </li>
        )
    }
}
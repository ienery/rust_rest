import * as React from 'react';

import {ITransact} from '../../Models';

/**
 * Свойства компонента.
 * 
 * @prop {ITransact} item Элемент транзакция.
 */
interface IProps {
    item: ITransact
}

/** 
 * Компонент просмотра транзакции в списке.
 */
export class TransactListItem extends React.Component<IProps, {}> {
    static displayName = 'TransactListItem';

    render () {
        const {item} = this.props;

        return(
            <li>
                {item.transact_id}
                {/* {item.parent_transact_id}
                {item.record.period_year}
                {item.record.period_month}
                {item.record.readings} */}
            </li>
        )
    }
}
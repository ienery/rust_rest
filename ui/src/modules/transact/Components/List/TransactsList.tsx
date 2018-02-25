import { includes } from 'lodash';
import * as React from 'react';
import { connect } from 'react-redux'

import { readTransacts } from '../../Data/Service';
import { ITransact } from '../../Models';
import { EStatusResponse } from '../../../../Data/Enums';
import { loadTransactsAction } from '../../Data/actions/TransactsActions';

import { TransactListItem } from './TransactListItem';
/** 
 * Свойства из connect State.
 * 
 * @prop {boolean} isLoading Загрузка данных.
 * @prop {boolean} hasError Наличие ошибок.
 * @prop {ITransact[]} data Данные.
 */
interface IPropsState {
    isLoading: boolean;
    hasError: boolean;
    data: ITransact[];
}

/**
 * Свойства из connect Dispatch.
 * 
 * @prop {Function} readTransacts Наличие ошибок.
 */
interface IPropsDispatch {
    readTransacts: any;
}

/** Свойства компонента. */
type IProp = IPropsState & IPropsDispatch;

/**
 * Компонент просмотра списка транзакций.
 */
class TransactList extends React.Component<IProp, {}> {
    static displayName = 'TransactList';

    componentDidMount() {
        //console.debug('componentDidMount');
        const {readTransacts} = this.props;
        //const transacts: ITransact[] | void = readTransacts();
        //console.debug('transacts', transacts);
        readTransacts();
    }

    /** 
     * Отрисовка элементов списка.
     */
    renderItems (): JSX.Element {
        const {data} = this.props;

        return (
            <ul>
                {data.map((item, index) => {
                    return (
                        <TransactListItem
                            item={item}
                            key={index}
                        />
                    );
                })}
            </ul>
        );
    }

    render() {
        const {isLoading, hasError} = this.props;

        return (
            <div>
                {isLoading ?
                    <div>Loading... </div> :
                    this.renderItems()
                }
            </div>
        );
    }
}

const mapStateToProps = (state) => {
    const {transacts: {status, data}} = state;
    const isLoading = includes([EStatusResponse.IDLE, EStatusResponse.LOADING], status);
    const hasError = status === EStatusResponse.FAILURE;

    return {
        isLoading,
        hasError,
        data
    }
}
  
const mapDispatchToProps = (dispatch) => {
    return {
        readTransacts: () => {
            dispatch(loadTransactsAction());
        }
    //   onClick: () => {
    //     dispatch(setVisibilityFilter(ownProps.filter))
    //   }
    }
}

const TransactListConnect = connect(
    mapStateToProps,
    mapDispatchToProps
)(TransactList);

export default TransactListConnect;
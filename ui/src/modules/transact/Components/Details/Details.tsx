import { isEmpty } from 'lodash'; 
import * as React from 'react';
import { connect } from 'react-redux';
import  * as queryString from 'query-string';

import {ITransact} from '../../Models';
import {readTransact, readBlockTransact} from '../../Data/Service';
import {EPreviousPage} from '../../Enums';
import {DetailsItem} from './DetailsItem' ;

/** 
 * Свойства из connect State.
 * 
 * @prop {any} location Router.
 * @prop {string} transactId Идентификатор транзакции.
 * @prop {EPreviousPage} previous Страница - источник перехода.
 * @prop {string} blockId Идентификатор блока.
 */
interface IPropsState {
    location: any;
    transactId: string; 
    previous: EPreviousPage;
    blockId: string;
}

/**
 * Состояние компонента.
 * 
 * @prop {ITransact} value Транзакция.
 */
interface IState {
    value: ITransact;
}

/** Свойства компонента. */
type IProp = IPropsState ;

/**
 * Компонент формы детального просмотра транзакции.
 */
class DetailsTransact extends React.Component<IProp, IState> {
    static displayName = 'DetailsTransact';

    state = {
        value: null
    };

    componentDidMount() {
        const {previous, transactId, blockId} = this.props;

        switch (previous) {
            // Переход со страниц создания или просмотра транзакций без блока.
            case EPreviousPage.CREATE:
            case EPreviousPage.NO_BLOCK:
                readTransact(transactId).then(
                    (result) => {
                        if (result) {
                            //console.debug('result', result);
                            this.setState({
                                value: result
                            })
                        }
                    }
                );
                break;

            default:
                if (blockId) {
                    const params = {
                        transact_id: transactId,
                        block_id:  blockId
                    };
                    readBlockTransact(params).then(
                        (result) => {
                            if (result) {
                                this.setState({
                                    value: result
                                })
                            }
                        }
                    );
                }
                break;
        }
    };

    render() {
        const {previous, blockId} = this.props;
        const {value} = this.state;

        return (
            <div>
                {!isEmpty(value) ? (
                    <DetailsItem 
                        transact={value}
                        previous={previous}
                        blockId={blockId}
                    />
                ) : (
                    <div>{'Loading...'}</div>
                )}
            </div>
        );
    };
}

const mapStateToProps = (state, ownProps) => {
    //console.debug('ownProps', ownProps);
    const {location} = ownProps;
    const {
        transactId,
        previous = null,
        blockId = null
    } = queryString.parse(location.search);

    return {
        location,
        transactId,
        previous,
        blockId
    }
}
  
// const mapDispatchToProps = (dispatch) => {
//     return {
//         push: (path) => {
//             dispatch(push(path));
//         }
//     }
// }

const DetailsTransactConnect = connect(
    mapStateToProps,
    //mapDispatchToProps
)(DetailsTransact);

export default DetailsTransactConnect;

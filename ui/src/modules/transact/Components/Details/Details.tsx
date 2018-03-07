import { isEmpty } from 'lodash'; 
import * as React from 'react';
import { connect } from 'react-redux';
import  * as queryString from 'query-string';

import {ITransact} from '../../Models';
import {readTransact} from '../../Data/Service';
import {DetailsItem} from './DetailsItem';
/** 
 * Свойства из connect State.
 * 
 * @prop {any} location Router.
 * @prop {string} transactId Идентификатор транзакции.
 */
interface IPropsState {
    location: any;
    transactId: string;
    
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
        readTransact(this.props.transactId).then(
            (result) => {
                if (result) {
                    //console.debug('result', result);
                    this.setState({
                        value: result
                    })
                }
            }
        )
    };

    render() {
        const {value} = this.state;

        return (
            <div>
                {!isEmpty(value) ? (
                    <DetailsItem transact={value} />
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
    const {transactId} = queryString.parse(location.search);

    return {
        location,
        transactId
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

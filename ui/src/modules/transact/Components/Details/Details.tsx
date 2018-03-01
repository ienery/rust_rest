import * as React from 'react';
import { connect } from 'react-redux';
import  * as queryString from 'query-string';

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

/** Свойства компонента. */
type IProp = IPropsState ;

/**
 * Компонент формы детального просмотра транзакции.
 */
class DetailsTransact extends React.Component<IProp, {}> {
    static displayName = 'DetailsTransact';

    render() {
        console.debug('transactId', this.props.transactId);

        return (
            <div>
                DetailsTransact:
                <h3>{this.props.transactId}</h3>
            </div>
        );
    }
}

const mapStateToProps = (state, ownProps) => {
    console.debug('ownProps', ownProps);
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

import * as React from 'react';

const DemoBox = props => <p className={`height-${props.value}`}>{props.children}</p>;

/**
 * Компонент меню создание/просмотр транзакции.
 */
class Transact extends React.Component {
    static displayName = 'Transact';

    render() {
        return (
            <div>
                Transact
            </div>
        );
    }
}

export default Transact;
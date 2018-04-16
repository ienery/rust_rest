import * as React from 'react';

/**
 * Свойства компонента.
 * 
 * @prop {any} history Объект истории для роутера.
 */
interface IProps {
    history: any;
}

class AppSimpleRouter extends React.Component<IProps, {}> {
    render() {
        const {history} = this.props;

        return (
            <div>AppSimpleRouter</div>
        );
    }
}

export {AppSimpleRouter};
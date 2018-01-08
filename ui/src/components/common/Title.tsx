import * as React from 'react';
import * as ReactDOM from 'react-dom';

/**
 * Свойства заголовка.
 * 
 * @prop {string} title Текст заголовка.
 */
interface IAppProps {
    title: string;
}

/**
 * Компонент вывода просто заголовка.
 */
class TitleSimple extends React.Component<IAppProps> {

    static displayName = 'TitleSimple';

    render() {
        const {title} = this.props;
 
        return (
            <h1>{title}</h1>
        );
    }
}

export {TitleSimple};
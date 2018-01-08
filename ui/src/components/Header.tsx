import * as React from 'react';

import { TitleSimple } from "./common/Title";
/**
 * Свойства хедера страницы.
 * 
 * @prop {string} title 
 */
interface IAppProps {
    title?: string;
}

/**
 * Компонент заголовка страницы.
 */
class Header extends React.Component<IAppProps> {

    static displayName = 'MainHeader';

    clickHeader = (): void => {
        const {title} = this.props;

        alert(`clickHeader ${title}`);
    }

    render() {
        const {title} = this.props;
 
        return (
            <div>
                <h3 
                    onClick={this.clickHeader}
                >
                   Заголовок страницы
                </h3>
                {
                    title ? 
                        <TitleSimple title={title} /> : 
                        null
                }
            </div>
        );
    }
}

export {Header};
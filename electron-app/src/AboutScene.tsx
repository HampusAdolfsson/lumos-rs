import React from 'react';
import './styles/About';

export class AboutScene extends React.Component {
    render() {
        return(
            <div className="aboutScene">
                <p>Created by <strong>Hampus Adolfsson</strong>.</p>
                <p>Check out this project on <a href="https://github.com/HampusAdolfsson/win-rt-rgb">GitHub</a>!</p>
            </div>
        )
    }
}